use dotenvy::dotenv;
use mysql::prelude::*;
use mysql::*;
use std::env;
use std::error::Error;
use std::io::Read;
use std::net::{TcpListener, TcpStream};

#[derive(Debug, FromRow)]
struct Trade {
    id: u64,
    ticker: String,
    price: f64,
    quantity: u32,
}

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;
    let pool = Pool::new(database_url.as_str())?;

    println!("データベース接続成功");

    let listener = TcpListener::bind("127.0.0.1:8081")?;
    println!("TCPサーバーが起動しました。ポート8081で待機中...");

    for stream_result in listener.incoming() {
        let mut stream: TcpStream = stream_result?;
        let mut buf = [0u8; 8];

        stream.read_exact(&mut buf)?;

        let record_id = u64::from_be_bytes(buf);
        println!("受信したID: {}", record_id);

        let mut conn = pool.get_conn()?;
        let selected_trade: Option<Trade> = conn.exec_first(
            "SELECT id, ticker, price, quantity FROM trades WHERE id = ?",
            (record_id,),
        )?;

        match selected_trade {
            Some(t) => {
                println!(
                    "ID:{}のデータを取得しました!tickerは{}, priceは{}, quantityは{}です.",
                    t.id, t.ticker, t.price, t.quantity
                );
            }
            None => {
                println!("ID:{}に対応するデータが見つかりません.", record_id);
            }
        }
    }

    Ok(())
}
