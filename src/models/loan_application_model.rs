use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoanApplication {
    pub user_id: u32,
    pub loan_type_id: u32,
    pub amount: f32,
    pub currency: String,
    pub term_months: String,
    pub interest_rate: f32,
}