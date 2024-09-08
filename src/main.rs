use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;

// Stellar testnet için gerekli API anahtarları
const STELLAR_PUBLIC_KEY: &str = "GAAW3AXSEJ7AMXF2U5W5YCEOKFS2T2ENJMVQZ6QZVQF6TAHZ4544OLB2";
//const STELLAR_SECRET_KEY: &str = "SBKO62AG7E2YGOVBYQDP6EEWDV4RY3I7YMEQPR3JQGB4CQPN35SD2OLD";

// Balance yapısının tanımı, Stellar API yanıtının bu alana göre biçimlendirileceği varsayılmaktadır.
#[derive(Serialize, Deserialize, Debug)]
struct Balance {
    // Örnek bir alan; gerçek yapı Stellar API yanıtına göre uyarlanmalıdır
    balance: String,
}

// Transaction yapısının tanımı, Stellar API yanıtının bu alana göre biçimlendirileceği varsayılmaktadır.
#[derive(Serialize, Deserialize, Debug)]
struct Transaction {
    // Örnek bir alan; gerçek yapı Stellar API yanıtına göre uyarlanmalıdır
    id: String,
}

// Ödeme yapısının tanımı, Stellar API'ye gönderilecek ödeme bilgilerini içerir.
#[derive(Serialize, Deserialize, Debug)]
struct Payment {
    // Örnek alanlar; gerçek yapı Stellar API'ye göre uyarlanmalıdır
    destination: String,
    amount: f64,
    asset: String,
}

// Balance yanıt yapısının tanımı
#[derive(Serialize, Deserialize, Debug)]
struct BalanceResponse {
    balances: Vec<Balance>,
}

// Transaction yanıt yapısının tanımı
#[derive(Serialize, Deserialize, Debug)]
struct TransactionsResponse {
    records: Vec<Transaction>,
}

// Hesap bilgilerini almak için fonksiyon
async fn fetch_account_info(account_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://horizon-testnet.stellar.org/accounts/{}",
        account_id
    );
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    println!("Response body for account info: {}", body);

    let account_info: serde_json::Value = serde_json::from_str(&body)?;
    println!("Account Info: {:?}", account_info);

    Ok(())
}

// Balance (bakiyeler) bilgilerini almak için fonksiyon
async fn fetch_balance() -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://horizon-testnet.stellar.org/accounts/{}",
        STELLAR_PUBLIC_KEY
    );
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    println!("Response body for balance: {}", body);

    let balance_response: BalanceResponse = serde_json::from_str(&body)?;
    println!("Balances: {:?}", balance_response.balances);

    Ok(())
}

// Transaction (işlemler) bilgilerini almak için fonksiyon
async fn fetch_transactions() -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://horizon-testnet.stellar.org/accounts/{}/transactions",
        STELLAR_PUBLIC_KEY
    );
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    println!("Response body for transactions: {}", body);

    let transactions_response: TransactionsResponse = serde_json::from_str(&body)?;
    println!("Transactions: {:?}", transactions_response.records);

    Ok(())
}

// Ödeme oluşturmak için fonksiyon
async fn create_payment(destination: &str, amount: f64) -> Result<(), Box<dyn Error>> {
    let url = "https://horizon-testnet.stellar.org/transactions";
    let client = Client::new();
    let payment = Payment {
        destination: destination.to_string(),
        amount,
        asset: "XLM".to_string(), // Örnek olarak XLM; gerçek kullanımda doğru varlık kodunu kullanın
    };
    let response = client.post(url).json(&payment).send().await?;
    let body = response.text().await?;

    println!("Response body for payment: {}", body);

    // Yanıtı kontrol edin ve gerekli işlemleri yapın
    Ok(())
}

// Ödeme geçmişini almak için fonksiyon
async fn fetch_payment_history(account_id: &str) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://horizon-testnet.stellar.org/accounts/{}/payments",
        account_id
    );
    let client = Client::new();
    let response = client.get(&url).send().await?;
    let body = response.text().await?;

    println!("Response body for payment history: {}", body);

    let payment_history: serde_json::Value = serde_json::from_str(&body)?;
    println!("Payment History: {:?}", payment_history);

    Ok(())
}

// Ana fonksiyon
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Hesap bilgilerini al
    fetch_account_info(STELLAR_PUBLIC_KEY).await?;

    // Bakiye bilgilerini al
    fetch_balance().await?;

    // İşlemleri al
    fetch_transactions().await?;

    // Örnek ödeme yap
    create_payment(
        "GBAA3FJH6NYDGFIMAB5C4YX4R6PYKFX5CDFFEQZPIA2QBN5CSQHFOYQP",
        100.0,
    )
    .await?;

    // Ödeme geçmişini al
    fetch_payment_history(STELLAR_PUBLIC_KEY).await?;

    Ok(())
}
