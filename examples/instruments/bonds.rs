use tinkoff_invest_sdk::{types::InstrumentsList, TinkoffInvestClient};

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").unwrap();
    
    let client = TinkoffInvestClient::connect(&token).await.unwrap();
    let mut instruments_client = client.instruments();    
    
    let bonds = instruments_client.bonds(InstrumentsList::Base).await.unwrap();    
    println!("Base bonds (5 of {}):", bonds.len());
    for bond in bonds.iter().take(5) {
        println!("{} {}", bond.figi(), bond.name());
    }
    println!("");
    
    let bonds = instruments_client.bonds(InstrumentsList::All).await.unwrap();
    println!("All bonds (5 of {}):", bonds.len());
    for bond in bonds.iter().take(5) {
        println!("{} {}", bond.figi(), bond.name());
    }

}