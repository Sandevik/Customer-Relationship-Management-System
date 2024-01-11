use actix_web::{dev::ServiceRequest, web::Data, error::ErrorUnauthorized, HttpMessage};
use actix_web_httpauth::extractors::{bearer::{BearerAuth, self}, AuthenticationError};
use jsonwebtoken::TokenData;
use crate::{controllers::jwt::{Claims, JWT}, models::user::User, AppState};


pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (actix_web::Error, ServiceRequest)> {
    let jwt_secret: Data<String> = Data::new(std::env::var("BACKEND_JWT_SECRET").expect("BACKEND_JWT_SECRET must be set"));
    let token_string: String = credentials.token().to_string();

    let data = req.app_data::<Data<AppState>>().expect("Appstate could not be found.");

    let claims: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = JWT::decode_jwt(&token_string, &jwt_secret);
    match claims {
        Err(_) => {
            let config = req.app_data::<bearer::Config>().cloned().unwrap_or_default().scope("");
            Err((AuthenticationError::from(config).into(), req))
        },
        Ok(value) => {

            match User::get_by_uuid(&value.claims.user.uuid.hyphenated().to_string(), data).await {
                Err(err) => Err((ErrorUnauthorized(err), req)),
                Ok(user) => {
                    match user {
                        None => Err((ErrorUnauthorized(r#"{"code": 401, "message": "Unauthorized"}"#), req)),
                        Some(_) => {
                            req.extensions_mut().insert(value.claims);
                            Ok(req)
                        }
                    }
                }
            }
        }
    }
}