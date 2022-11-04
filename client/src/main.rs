use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::process;
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("성공적으로 서버에 연결되었습니다");

            loop {
                let mut message = String::new();
                let     message = input(&mut message);

                stream.write(message).unwrap_or_else(|err| {
                    println!("데이터 전송 실패: {}", err);
                    process::exit(1);
                });
                
                if message == b"/quit" {
                    break;
                }

                let mut buffer = [0_u8; 50];
                match stream.read(&mut buffer) {
                    Ok(_) => {
                        let text = from_utf8(&buffer).unwrap();
                        println!("수신: {}", text);
                    },
                    Err(e) => {
                        println!("데이터 수신 실패: {}", e);
                    },
                }
            }
        },
        Err(e) => {
            println!("연결에 실패하였습니다: {}", e);
        },
    }
}

fn input<'a>(msg: &'a mut String) -> &'a [u8] {
    io::stdin().read_line(msg).unwrap();
    msg.trim().as_bytes()
}