use reqwest;
use serde_json::Value;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Huobi API'sinden verileri çek
    let url = "https://api.huobi.pro/v1/common/symbols";
    let resp = reqwest::get(url).await?.json::<Value>().await?;

    // USDT spot piyasasındaki coinleri filtrele ve düzenle
    let mut markets: Vec<String> = resp["data"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|&s| s["quote-currency"].as_str().unwrap() == "usdt")
        .map(|s| {
            format!(
                "HUOBI:{}USDT",
                s["base-currency"].as_str().unwrap().to_uppercase()
            )
        })
        .collect();

    // Özel sıralama fonksiyonu
    markets.sort_by(|a, b| {
        let a_parts: Vec<&str> = a.split(':').collect();
        let b_parts: Vec<&str> = b.split(':').collect();
        let a_coin = &a_parts[1][..a_parts[1].len() - 4];
        let b_coin = &b_parts[1][..b_parts[1].len() - 4];

        if a_coin.chars().next().unwrap().is_numeric()
            && b_coin.chars().next().unwrap().is_numeric()
        {
            b_coin.cmp(a_coin)
        } else if a_coin.chars().next().unwrap().is_numeric() {
            std::cmp::Ordering::Less
        } else if b_coin.chars().next().unwrap().is_numeric() {
            std::cmp::Ordering::Greater
        } else {
            a_coin.cmp(b_coin)
        }
    });

    // Sonuçları dosyaya yaz
    let mut file = File::create("huobi_usdt_markets.txt")?;
    for market in markets {
        writeln!(file, "{}", market)?;
    }

    println!("Veriler başarıyla 'huobi_usdt_markets.txt' dosyasına yazıldı.");
    Ok(())
}
