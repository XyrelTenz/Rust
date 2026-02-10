pub struct BankModel {
    pub balance: i64,
}

impl BankModel {
    pub fn new(balance: i64) -> BankModel {
        BankModel { balance }
    }
}
