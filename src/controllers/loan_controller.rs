use actix_web::{web, HttpResponse, Responder};
use tracing::{info, warn};
use crate::AppState;
use crate::models::loan_application_model::LoanApplication;
use crate::models::loan_model::{Loan, LoanResponse};

pub async fn apply_loan(data: web::Data<AppState>, loan_application: web::Json<LoanApplication>) -> impl Responder {
    info!(?loan_application, "Received loan application");

    let new_loan = Loan {
        id: data.loans.lock().unwrap().len() as u32 + 1,
        user_id: loan_application.user_id,
        loan_type_id: loan_application.loan_type_id,
        amount: loan_application.amount,
        currency: loan_application.currency.clone(),
        term_months: loan_application.term_months.clone(),
        interest_rate: loan_application.interest_rate,
        monthly_payment: loan_application.amount / loan_application.term_months.parse::<f32>().unwrap(),
        balance: loan_application.amount,
        status: "Approved".to_string(),
        start_date: "2024-11-04".to_string(),
        end_date: "2025-11-04".to_string(),
    };

    data.loans.lock().unwrap().push(new_loan.clone());
    info!(loan_id = new_loan.id, "Loan approved and created");

    HttpResponse::Ok().json(LoanResponse {
        loan_id: new_loan.id,
        message: "Loan approved and created successfully".to_string(),
    })
}

pub async fn get_loan(data: web::Data<AppState>, loan_id: web::Path<u32>) -> impl Responder {
    let loans = data.loans.lock().unwrap();
    info!(loan_id = *loan_id, "Fetching loan details");

    if let Some(loan) = loans.iter().find(|&loan| loan.id == *loan_id) {
        HttpResponse::Ok().json(loan)
    } else {
        warn!(loan_id = *loan_id, "Loan not found");
        HttpResponse::NotFound().json("Loan not found")
    }
}

pub async fn check_loan_status(data: web::Data<AppState>, loan_id: web::Path<u32>) -> impl Responder {
    let loans = data.loans.lock().unwrap();
    info!(loan_id = *loan_id, "Checking loan status");

    if let Some(loan) = loans.iter().find(|&loan| loan.id == *loan_id) {
        HttpResponse::Ok().json(format!("Loan status: {}", loan.status))
    } else {
        warn!(loan_id = *loan_id, "Loan not found for status check");
        HttpResponse::NotFound().json("Loan not found")
    }
}