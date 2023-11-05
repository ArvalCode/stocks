mod local;
mod global;

fn main() {
    // Create a User instance
    let SHB = global::Stock::new("Shelby's Stock Holding", "SHB", 100, 10);    
    let mut user = local::User::new("shelby", "OJ8oR@example.com", "password123");
    let mut username = user.return_username();
    let mut email = user.return_email();
    let mut password = user.return_password();

    println!("Username: {}", username);
    println!("Email: {}", email);
    println!("Password: {}", password);
}
