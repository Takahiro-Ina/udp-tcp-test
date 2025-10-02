use dotenvy::dotenv;
use mysql::prelude::*;
use mysql::*;
use std::env;
use std::error::Error;
use std::net::UdpSocket;

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

    let socket = UdpSocket::bind("127.0.0.1:8081")?;
    println!("UDP待機中... (127.0.0.1:8081)");

    let mut buf = [0u8; 1024];

    loop {
        let (len, src) = socket.recv_from(&mut buf)?;
        println!("受信: {} バイト from {}", len, src);

        if len != 8 {
            println!(
                "無効なデータ長: {} バイト. 8バイトのIDを期待しています.\n",
                len
            );
            continue;
        }

        let id_bytes: [u8; 8] = buf[..8].try_into()?;
        let record_id = u64::from_be_bytes(id_bytes);
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

    // Ok(())
}
