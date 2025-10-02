use std::net::UdpSocket;
use std::time::Duration;
use std::{io, thread};

fn main() -> io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let target = "127.0.0.1:8080";

    println!("めっちゃメッセージを送るよ");

    for _i in 0..10000000 {
        let message = b"ping";
        socket.send_to(message, target)?;

        thread::sleep(Duration::from_micros(1));
    }

    println!("送信完了");

    Ok(())
}
