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
            g_market_cap: 10000,
            g_share_price: 10,
            g_number_of_shares: market_cap / share_price,
        }

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
}
