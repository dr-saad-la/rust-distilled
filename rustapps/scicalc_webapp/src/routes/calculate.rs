use actix_web::{post, web, HttpResponse, Responder};
use serde_derive::{Deserialize, Serialize};
use log::error;
use eval::eval;

#[derive(Deserialize)]
pub struct CalculationRequest {
    pub expression: String,
}

#[derive(Serialize)]
pub struct CalculationResponse {
    pub result: f64,
    pub error: Option<String>,
}

#[post("/calculate")]
async fn calculate(request: web::Json<CalculationRequest>) -> impl Responder {
    let expression = &request.expression;

    let result = match eval(expression) {
        Ok(value) => CalculationResponse { result: value.as_f64().unwrap_or(0.0), error: None },
        Err(e) => {
            error!("Failed to evaluate expression: {}", e);
            CalculationResponse { result: 0.0, error: Some(format!("Failed to evaluate expression: {}", e)) }
        }
    };

    HttpResponse::Ok().json(result)
}
