mod api;

fn main() {
    // Init tracing
    tracing_subscriber::fmt::init();
    tracing::info!("Starting the program");

    // Get the vault
    let vault = api::get_vault("9745", "14");
    tracing::info!("API Vault: {:?}", vault);
    println!("{:?}", vault);
}
