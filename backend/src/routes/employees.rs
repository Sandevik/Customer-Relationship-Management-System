use actix_web::{body::{EitherBody, BoxBody}, delete, dev::{ServiceFactory, ServiceRequest, ServiceResponse}, get, post, put, web, Error, HttpResponse, Responder, Scope};
use actix_web_httpauth::middleware::HttpAuthentication;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDate;
use crate::{models::employee::Employee, routes::Response};
use crate::{middleware::owns_or_admin_middleware::validator, AppState};

use super::Limit;

pub fn employees() -> Scope<impl ServiceFactory<ServiceRequest, Config = (), Response = ServiceResponse<EitherBody<BoxBody>>, Error = Error, InitError = ()>> {
    let owner_or_admin_middleware = HttpAuthentication::bearer(validator);
    
    let scope = web::scope("/employees")
        .wrap(owner_or_admin_middleware)
        .service(by_uuid)
        .service(create_employee)
        .service(get_all)
        ;
        
    scope
}

#[derive(Serialize, Deserialize)]
struct CustomerByUuidRequest {
    #[serde(rename(deserialize = "crmUuid"))]
    crm_uuid: String, //crm uuid
    #[serde(rename(deserialize = "employeeUuid"))]
    employee_uuid: String,
}

#[get("")]
async fn by_uuid(query: web::Query<CustomerByUuidRequest>, data: web::Data<AppState>) -> impl Responder {
    let employee_uuid = Uuid::parse_str(&query.employee_uuid).unwrap_or_default();
    let crm_uuid = Uuid::parse_str(&query.crm_uuid).unwrap_or_default();
    match Employee::get_by_uuid(&employee_uuid, &crm_uuid, &data).await {
        Err(err) => HttpResponse::InternalServerError().json(Response::<String>::internal_server_error(&err.to_string())),
        Ok(customer) => HttpResponse::Ok().json(Response::ok("Successfully fetched employee", customer))
    }
}




#[derive(Deserialize)]
struct AllRequest {
    #[serde(rename(deserialize = "crmUuid"))]
    crm_uuid: String,
    offset: Option<u16>,
    limit: Option<u16>
}

#[get("/all")]
async fn get_all(data: web::Data<AppState>, query: web::Query<AllRequest>) -> impl Responder {
    match Employee::get_all(&Uuid::parse_str(&query.crm_uuid).unwrap_or_default(), match &query.limit {None => Limit::None, Some(u) => Limit::Some(*u)}, query.offset, &data).await {
        Err(err) => HttpResponse::InternalServerError().json(Response::<String>::internal_server_error(&err.to_string())),
        Ok(customer) => HttpResponse::Ok().json(Response::ok("Successfully fetched employee", Some(customer)))
    }
}


#[derive(Deserialize)]
struct SearchRequest {
    #[serde(rename(deserialize = "crmUuid"))]
    crm_uuid: String,
    q: String,
}

#[get("/search")]
async fn search(data: web::Data<AppState>, query: web::Query<SearchRequest>) -> impl Responder {
    match Employee::search(&Uuid::parse_str(&query.crm_uuid).unwrap_or_default(), &query.q, Limit::Some(20), &data).await {
        Err(err) => HttpResponse::InternalServerError().json(Response::<String>::internal_server_error(&err.to_string())),
        Ok(customer) => HttpResponse::Ok().json(Response::ok("Successfully searched employees", Some(customer)))
    }
}


#[derive(Deserialize)]
struct CreateEmployeeRequest {
    #[serde(rename(serialize = "crmUuid", deserialize = "crmUuid"))]
    pub crm_uuid: String,
    #[serde(rename(serialize = "userUuid", deserialize = "userUuid"))]
    user_uuid: Option<String>,
    #[serde(rename(serialize = "firstName", deserialize = "firstName"))]
    first_name: Option<String>,
    #[serde(rename(serialize = "lastName", deserialize = "lastName"))]
    last_name: Option<String>,
    #[serde(rename(serialize = "dateOfBirth", deserialize = "dateOfBirth"))]
    date_of_birth: Option<NaiveDate>,
    ssn: Option<String>,
    address: Option<String>,
    #[serde(rename(serialize = "zipCode", deserialize = "zipCode"))]
    zip_code: Option<String>,
    city: Option<String>,
    #[serde(rename(serialize = "phoneNumber", deserialize = "phoneNumber"))]
    phone_number: Option<String>,
    role: Option<String>,
    #[serde(rename(serialize = "drivingLicenseClass", deserialize = "drivingLicenseClass"))]
    driving_license_class: Option<String>,
    #[serde(rename(serialize = "periodOfValidity", deserialize = "periodOfValidity"))]
    period_of_validity: Option<String>,
    #[serde(rename(serialize = "bankNumber", deserialize = "bankNumber"))]
    bank_number: Option<String>,
    #[serde(rename(serialize = "clearingNumber", deserialize = "clearingNumber"))]
    clearing_number: Option<String>,
    #[serde(rename(serialize = "bankName", deserialize = "bankName"))]
    bank_name: Option<String>,
    email: Option<String>,
    #[serde(rename(serialize = "employmentType", deserialize = "employmentType"))]
    employment_type: Option<String>,
    access_level: Option<String>,
}


#[post("/create")]
async fn create_employee(data: web::Data<AppState>, body: web::Json<CreateEmployeeRequest>) -> impl Responder {
    let customer: Employee = Employee {
        user_uuid: match body.user_uuid.clone() {None => None, Some(str) => Some(match Uuid::parse_str(&str) {Err(_) => Uuid::new_v4(), Ok(u) => u})} ,
        first_name: body.first_name.clone(),
        last_name: body.last_name.clone(),
        date_of_birth: body.date_of_birth.clone(),
        ssn: body.ssn.clone(),
        address: body.address.clone(),
        zip_code: body.zip_code.clone(),
        city: body.city.clone(),
        phone_number: body.phone_number.clone(),
        role: body.role.clone(),
        driving_license_class: body.driving_license_class.clone(),
        period_of_validity: body.period_of_validity.clone(),
        bank_number: body.bank_number.clone(),
        clearing_number: body.clearing_number.clone(),
        bank_name: body.bank_name.clone(),
        email: body.email.clone(),
        employment_type: match body.employment_type.clone() {None => None, Some(str) => Some(match Uuid::parse_str(&str) {Err(_) => Uuid::new_v4(), Ok(u) => u})},
        access_level: body.access_level.clone(),
        ..Employee::default()
    };
    match customer.insert(&Uuid::parse_str(&body.crm_uuid).unwrap_or_default(), &data).await {
        Err(err) => HttpResponse::InternalServerError().json(Response::<String>::internal_server_error(&err.to_string())),
        Ok(_) => HttpResponse::Created().json(Response::<String>::created("Successfully created employee"))
    }
}