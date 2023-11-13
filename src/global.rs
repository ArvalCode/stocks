pub struct Stock {
    pub g_name: String,
    pub g_symbol: String,
    pub g_market_cap: u64,
    pub g_share_price: u64,
    pub g_number_of_shares: u64,
}

impl Stock {
    pub fn new(name: &str, symbol: &str, market_cap: u64, share_price: u64) -> Stock {
        Stock {
            g_name: name.to_string(),
            g_symbol: symbol.to_string(),
            g_market_cap: market_cap,
            g_share_price: share_price,
            g_number_of_shares: market_cap / share_price,
        }
    }
    // Method to change the share price
    pub fn change_share_price(&mut self, new_share_price: u64) {
        self.g_share_price = new_share_price;
        self.g_number_of_shares = self.g_market_cap / new_share_price;
    }

    // Method to calculate the total market value of the stock
    pub fn market_value(&self) -> u64 {
        self.g_share_price * self.g_number_of_shares
    }


}

pub struct StockHolding<'a> {
    pub stock: &'a Stock,  // Reference to the associated Stock object
    pub quantity: u32,
    pub purchase_price: f64,
}

impl StockHolding<'_> {
    pub fn new(stock: &Stock, quantity: u32, purchase_price: f64) -> StockHolding {
        StockHolding {
            stock,
            quantity,
            purchase_price,
        }
    }

    // Method to calculate the total cost of the stock holding
    pub fn total_cost(&self) -> f64 {
        self.quantity as f64 * self.purchase_price
    }

    // Method to calculate the current value of the stock holding
    pub fn current_value(&self) -> f64 {
        self.quantity as f64 * self.stock.g_share_price as f64
    }

    // Method to calculate the profit or loss of the stock holding
    pub fn profit_or_loss(&self) -> f64 {
        self.current_value() - self.total_cost()
    }

}