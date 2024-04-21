use std::{io::Read, net::{TcpListener, TcpStream}};
use std::io::prelude::*;

mod http_custom;
use http_custom::{http_status_code_resolve, HttpStatusCode};

use crate::http_custom::{http_method_resolve, HttpMethod};

fn main() {

    let address = "127.0.0.1:8000";

    let listener = TcpListener::bind(&address).unwrap();
    println!("Running on {}", address);

    let mut is_run: bool = false;

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        if is_run == false {
            println!("Stream: Active");
            is_run = true;
        }
        
        handler_connection(stream);
    }

}


fn handler_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("\n<<Stream recibido>>\n");

    println!("{}", String::from_utf8_lossy(&buffer[..]));

    let get = http_method_resolve(HttpMethod::GET);

    if buffer.starts_with(get) {
        println!("\n\n<<{}>>\n\n", http_status_code_resolve(HttpStatusCode::RP_200));

        http_send( 
            stream, 
            build_content_view( 
                String::from("index.html"),
                HttpStatusCode::RP_200
            ) 
        );
    
    } else {
        println!("\n\n<<{}>>\n\n", http_status_code_resolve(HttpStatusCode::RP_404));

        http_send(
            stream, 
            build_content_view(
                String::from("not_found.html"),
                HttpStatusCode::RP_404
            )
        );

    }
    
}

fn build_content_view(path: String, status_code: HttpStatusCode) -> String {
    let content = std::fs::read_to_string(path).unwrap();
    return format!(
        "HTTP/1.1 {}\r\n Content-Length: {}\r\n\r\n{}", 
        http_status_code_resolve(status_code),
        content.len(), 
        content
    );
}

fn http_send(mut stream: TcpStream, response: String){
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
