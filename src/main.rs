use actix_web::{web, App, HttpServer};
use std::sync::Mutex;
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::{FmtSubscriber, fmt::time::UtcTime};
use crate::controllers::loan_controller::{apply_loan, check_loan_status, get_loan};
use crate::controllers::payment_controller::make_payment;
use crate::models::loan_model::Loan;

mod models;
mod controllers;

struct AppState {
    loans: Mutex<Vec<Loan>>,
}

fn init_tracer() -> Result<sdktrace::Tracer, TraceError> {
    // Configure the OpenTelemetry tracer
    opentelemetry_jaeger::new_pipeline()
        .with_service_name("loan-api")
        .install_simple()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let file_appender = rolling::daily("logs", "loan_api.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = FmtSubscriber::builder()
        .json()
        .with_timer(UtcTime::rfc_3339()) // Timestamp in RFC 3339 format
        .with_writer(non_blocking) // Write to file instead of console
        .with_current_span(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    info!("Starting loan API server on 127.0.0.1:8080");

    let state = web::Data::new(AppState {
        loans: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .route("/loan/apply", web::post().to(apply_loan))
            .route("/loan/{loan_id}", web::get().to(get_loan))
            .route("/loan/{loan_id}/pay", web::post().to(make_payment))
            .route("/loan/{loan_id}/status", web::get().to(check_loan_status))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}