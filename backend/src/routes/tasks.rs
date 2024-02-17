use actix_web::{body::{EitherBody, BoxBody}, dev::{ServiceFactory, ServiceRequest, ServiceResponse}, post, get, web, Error, HttpResponse, Responder, Scope};
use actix_web_httpauth::middleware::HttpAuthentication;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

use crate::{middleware::owns_or_admin_middleware::{validator, RequiresUuid}, models::task::{Task, TaskStatus}, AppState};
use crate::routes::Response;


pub fn tasks() -> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<EitherBody<BoxBody>>, Error = Error, InitError = ()>> {
    let owns_or_admin_middleware = HttpAuthentication::bearer(validator);
    
    let scope = web::scope("/tasks")
        .wrap(owns_or_admin_middleware)
        .service(create_task)
        .service(get_by_client);
        
    scope
}

#[derive(Deserialize)]
struct CreateTodoRequest {
    #[serde(rename(deserialize = "crmUuid"))]
    crm_uuid: String,
    deadline: Option<i64>,
    status: Option<String>,
    #[serde(rename(deserialize = "clientUuid"))]
    client_uuid: Option<String>,
    title: Option<String>,
}

#[post("/create")]
async fn create_task(data: web::Data<AppState>, body: web::Json<CreateTodoRequest>) -> impl Responder {
    let deadline: Option<DateTime<Utc>> = match body.deadline {None => None, Some(i) => Some(Utc.from_local_datetime(&NaiveDateTime::from_timestamp_millis(i).expect("Could not convert milliseconds to date")).unwrap())};
    let crm_uuid: Uuid = Uuid::parse_str(&body.crm_uuid).unwrap_or_default();
    let client_uuid: Option<Uuid> = match &body.client_uuid { Some(uuid) => Some(Uuid::parse_str(&uuid).unwrap_or_default()), None => None};
    let todo: Task = Task {
        client_uuid, 
        crm_uuid, 
        deadline, 
        title: body.title.clone(),
        status: if let None = &body.status {None} else {Some(TaskStatus::from_string(&body.status.clone().unwrap()))},
        ..Task::default()
    }; 

    match todo.insert(&data).await {
        Err(err) => HttpResponse::InternalServerError().json(Response::<String>::internal_server_error(&err.to_string())),
        Ok(_) => HttpResponse::Created().json(Response::<String>::created("Successfully created todo"))
    }
}


#[derive(Serialize, Deserialize)]
struct ByClientRequestQuery {
    #[serde(rename(serialize = "crmUuid", deserialize = "crmUuid"))]
    crm_uuid: String,
    #[serde(rename(serialize = "clientUuid", deserialize = "clientUuid"))]
    client_uuid: String
}

#[get("/by-client")]
async fn get_by_client(data: web::Data<AppState>, query: web::Query<ByClientRequestQuery>) -> impl Responder {
    match Task::get_by_client_uuid(&Uuid::parse_str(&query.client_uuid).unwrap_or_default(), &Uuid::parse_str(&query.crm_uuid).unwrap_or_default(), &data).await {
        Err(err) => HttpResponse::InternalServerError().json(Response::<String>::internal_server_error(&err.to_string())),
        Ok(tasks) => HttpResponse::Ok().json(Response::ok("Successfully fetched tasks", Some(tasks)))
    }
}