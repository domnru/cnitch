use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::SplitN;

fn main() {
    let secret: String = std::env::var("SECRET").unwrap_or("".to_owned());
    let port: String = std::env::var("SERVER_PORT").unwrap_or("8080".to_owned());
    let listener: TcpListener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();

    println!("Listening on port {}", port);

    for stream in listener.incoming() {
        let stream: TcpStream = stream.unwrap();
        handle_connection(stream, &secret);
    }
}

fn handle_connection(mut stream: TcpStream, secret: &str) {
    let mut buffer: [u8; 1024] = [0; 1024];
    stream.read(&mut buffer).unwrap();
    let post = b"POST / HTTP/1.1\r\n";
    if buffer.starts_with(post) {
        use std::str;
        let req: &str = str::from_utf8(&buffer).unwrap();
        let parts: SplitN<&str> = req.splitn(2, "\r\n\r\n");
        let body: &str = parts.last().unwrap().trim().trim_end_matches(char::from(0));

        let mut response: String = "HTTP/1.1 200 OK\r\n".to_owned();

        if secret != "" && secret == body {
            let content: String = read_file();
            let content: Vec<&str> = content.split("\n").filter(|e| !e.is_empty()).collect();
            let res: String = format!("{}\r\n\r\n{:?}", response, content);
            response = res.to_owned();
        } else {
            if let Err(e) = append_to_file(body) {
                eprintln!("{}", e);
                response = "HTTP/1.1 500 Internal Server Error\r\n".to_owned();
            }
        }

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn append_to_file(content: &str) -> Result<(), std::io::Error> {
    use chrono::prelude::Utc;

    let mut file: File = OpenOptions::new().create(true).append(true).open("data")?;
    let now = Utc::now();

    writeln!(file, "{{\"data\" : \"{}\", \"date\" : \"{}\"}},", content, now)?;

    Ok(())
}

fn read_file() -> String {
    let mut data_file: File = File::open("data").unwrap();
    let mut file_content: String = String::new();
    data_file.read_to_string(&mut file_content).unwrap();

    return file_content;
}
