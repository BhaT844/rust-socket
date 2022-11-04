use std::{thread, process};
use std::net::{TcpListener, Shutdown, TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

fn main() {
    const PORT: &str = "3333";
    let addr = format!("0.0.0.0:{}", PORT);

    let listener = TcpListener::bind(addr).unwrap_or_else(|err| {
        eprintln!("서버 생성 실패: {}", err);
        process::exit(1);
    });

    println!("{}번 포트에서 서버가 열렸습니다.", PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) =>  {
                println!("{} 연결됨", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(e) => {
                println!("연결실패: {}", e);
            },
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0_u8; 50];

    while match stream.read(&mut buffer) {
        Ok(size) => {
            if &buffer[0..size] == b"/quit" {
                println!("{}님이 연결을 종료하였습니다.", stream.peer_addr().unwrap());
                false
            } else {
                println!("{}: {}", stream.peer_addr().unwrap(), from_utf8(&buffer).unwrap());

                for i in buffer.iter_mut() {
                    if
                        65    <=    *i && *i    <=    90
                    {
                        *i += 32;
                    }
    
                    else if
                        97    <=    *i && *i    <=    122
                    {
                        *i -= 32;
                    }
                }

                stream.write(&buffer[0..size]).unwrap();
                buffer = [0_u8; 50];
                true
            }
        },
        Err(_) => {
            println!("오류가 발생하여 연결을 종료합니다: {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        },
    } {}
}