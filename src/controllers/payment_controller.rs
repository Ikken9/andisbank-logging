use actix_web::{web, HttpResponse, Responder};
use tracing::{info, warn};
use crate::AppState;
use crate::models::payment_model::Payment;

pub async fn make_payment(data: web::Data<AppState>, loan_id: web::Path<u32>, payment: web::Json<Payment>, ) -> impl Responder {
    let mut loans = data.loans.lock().unwrap();
    info!(loan_id = *loan_id, payment_amount = payment.amount, "Processing payment");

    if let Some(loan) = loans.iter_mut().find(|loan| loan.id == *loan_id) {
        loan.balance -= payment.amount;
        info!(loan_id = loan.id, new_balance = loan.balance, "Payment processed");
        HttpResponse::Ok().json(format!("Payment of ${:.2} received. Remaining balance: ${:.2}", payment.amount, loan.balance))
    } else {
        warn!(loan_id = *loan_id, "Loan not found for payment");
        HttpResponse::NotFound().json("Loan not found")
    }
}
