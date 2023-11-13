mod local;
mod global;

use std::fmt::Debug;

fn main() {
    // Create a User instance
    let SHB = global::Stock::new("Shelby's Stock Holding", "SHB", 100, 10);    
    let mut shelby = local::User::new("shelby", "OJ8oR@example.com", "password123");

    shelby.add_stock_holding(&SHB, 10, 10.0);
//Change purchase price type to u64
    shelby.combine_stock_holdings();
    println!("{:?}", shelby.get_stock_holdings());
  
}
