use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, {}!", "World");
    dotenv::from_filename(".env").ok();

    let descriptor = env::var("WALLET_DESCRIPTOR")?;

    println!("Descriptor: {}", descriptor);
    dbg!(descriptor);

    Ok(())
}
