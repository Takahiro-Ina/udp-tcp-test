use std::io;
use std::net::UdpSocket;
use std::time::Duration;

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8080")?;
    println!("受信待ち...");

    socket.set_read_timeout(Some(Duration::from_secs(10)))?;

    let mut buf = [0; 1024];
    let mut received_count = 0;

    loop {
        match socket.recv_from(&mut buf) {
            Ok((size, _src)) => {
                received_count += 1;
                let message = String::from_utf8_lossy(&buf[..size]);
                println!("{}: 受信回数, メッセージ: {}", received_count, message);
            }

            Err(e) => {
                if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut {
                    println!("受信タイムアウト");
                    break;
                }
                eprintln!("受信エラー: {}", e);
            }
        }
    }

    println!("最終受信回数: {}", received_count);

    Ok(())
}
