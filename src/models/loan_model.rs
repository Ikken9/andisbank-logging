use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Loan {
    pub id: u32,
    pub user_id: u32,
    pub loan_type_id: u32,
    pub amount: f32,
    pub currency: String,
    pub term_months: String,
    pub interest_rate: f32,
    pub monthly_payment: f32,
    pub balance: f32,
    pub status: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanResponse {
    pub(crate) loan_id: u32,
    pub(crate) message: String,
}