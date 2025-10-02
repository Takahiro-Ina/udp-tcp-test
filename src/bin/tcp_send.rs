use dotenvy::dotenv;
use mysql::prelude::*;
use mysql::*;
use std::env;
use std::error::Error;
use std::io::Write;
use std::net::TcpStream;

#[derive(Debug)]
struct Trade {
    ticker: String,
    price: f64,
    quantity: u32,
}
fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = Pool::new(database_url.as_str())?;
    let mut conn = pool.get_conn()?;

    println!("データベース接続成功");

    conn.exec_drop(
        "CREATE TABLE IF NOT EXISTS trades (
            id INT AUTO_INCREMENT PRIMARY KEY,
            ticker VARCHAR(10) NOT NULL,
            price DECIMAL(10, 2) NOT NULL,
            quantity INT NOT NULL
        )",
        (),
    )?;

    let trade_to_insert = Trade {
        ticker: "APPLE".to_string(),
        price: 123.45,
        quantity: 100,
    };

    conn.exec_drop(
        "INSERT INTO trades (ticker, price, quantity) VALUES (?, ?, ?)",
        (
            trade_to_insert.ticker,
            trade_to_insert.price,
            trade_to_insert.quantity,
        ),
    )?;

    let last_id: u64 = conn.last_insert_id();

    println!("新しいデータを挿入 ID: {}", last_id);

    let mut stream = TcpStream::connect("127.0.0.1:8081")?;

    let last_id_bytes = last_id.to_be_bytes();
    stream.write_all(&last_id_bytes)?;

    println!("TCPでIDを送信: {}", last_id);

    Ok(())
}
