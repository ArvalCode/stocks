use crate::global::{StockHolding, Stock};

pub struct User<'a> {
    username: String,
    email: String,
    password: String,
    stock_holdings: Vec<StockHolding<'a>>,
}

impl<'a> User<'a> {
    pub fn new(username: &str, email: &str, password: &str) -> User<'a> {
        User {
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            stock_holdings: vec![],
        }
    }

    pub fn change_username(&mut self, new_username: &str) {
        self.username = new_username.to_string();
    }

    pub fn change_email(&mut self, new_email: &str) {
        self.email = new_email.to_string();
    }

    pub fn change_password(&mut self, new_password: &str) {
        self.password = new_password.to_string();
    }

    pub fn return_username(&self) -> &str {
        &self.username
    }

    pub fn return_email(&self) -> &str {
        &self.email
    }

    pub fn return_password(&self) -> &str {
        &self.password
    }

    pub fn add_stock_holding(&mut self, stock: &'a Stock, quantity: u32, purchase_price: f64) {
        let holding = StockHolding::new(stock, quantity, purchase_price);
        self.stock_holdings.push(holding);
    }

    pub fn get_stock_holdings(&self) -> &Vec<StockHolding<'a>> {
        &self.stock_holdings
    }
}
