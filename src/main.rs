use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use puan_pv::{Proposition, PropT};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub const APPLICATION_JSON: &str = "application/json";
pub type StringInterpretation = HashMap<String, f64>;

#[derive(Debug, Serialize, Deserialize)]
pub struct EvaluableInterpretation {
    propositions: Vec<Proposition>,
    interpretations: Vec<StringInterpretation>,
}

#[post("evaluate")]
pub async fn evaluate(proposition_interpretations: web::Json<EvaluableInterpretation>) -> HttpResponse {
    HttpResponse::Ok()
        .content_type(APPLICATION_JSON)
        .json(
            proposition_interpretations.0
                .propositions
                .iter()
                .map(
                    |proposition| 
                    proposition_interpretations.0.interpretations.iter().enumerate().filter_map(
                        |(index, interpretation)| 
                        match proposition.interpret(interpretation) == 1.0 {
                            true => Some(index),
                            false => None,
                        }
                    ).collect::<Vec<usize>>()
                ).collect::<Vec<Vec<usize>>>(),
        )
}

#[get("/health")]
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().body("OK")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(evaluate)
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}