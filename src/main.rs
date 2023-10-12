use actix_web::{post, web, App, HttpResponse, HttpServer};
use puan_pv::PropKey;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub const APPLICATION_JSON: &str = "application/json";
pub type StringInterpretation = HashMap<String, f64>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluableInterpretation {
    evaluables: Vec<PropKey>,
    interpretation: StringInterpretation,
}

#[post("evaluate")]
pub async fn evaluate(evaluable_interpretation: web::Json<EvaluableInterpretation>) -> HttpResponse {
    let evaluated: Vec<String> = evaluable_interpretation.0
        .evaluables
        .iter()
        .filter_map(
            |x| 
            x.evaluate(&evaluable_interpretation.0.interpretation)
        ).collect();
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(
            evaluated
        )
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(evaluate)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}