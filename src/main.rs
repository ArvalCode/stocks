mod local;
mod global;

fn main() {
    // Create a User instance
    let SHB = global::Stock::new("Shelby's Stock Holding", "SHB", 100, 10);    
    let mut shelby = local::User::new("shelby", "OJ8oR@example.com", "password123");
    let mut username = shelby.return_username();
    let mut email = shelby.return_email();
    let mut password = shelby.return_password();

    println!("Username: {}", username);
    println!("Email: {}", email);
    println!("Password: {}", password);

}
