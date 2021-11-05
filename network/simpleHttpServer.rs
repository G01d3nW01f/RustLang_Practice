use std::io::{Read,Write};
use std::thread;
use std::net::{TcpListener,TcpStream};

fn handle_read(mut stream: &TcpStream){
    let mut buffer = [0u8 ;4096];
    match stream.read(&mut buffer){
        Ok(_) => {
            let req_str = String::from_utf8_lossy(&buffer);
            println!("{}",req_str);
            }
        Err(e) => println!("Failed: {}",e),
    }
}

fn handle_write(mut stream: TcpStream){
    let response = b"HTTP/1.1 200 OK\r\nContent-Type: text/html;chaset=UTF-8;\r\n\r\n<html><body>hello</body></html>\r\n";
    match stream.write(response){
        Ok(_) => println!("response sent"),
        Err(e) => println!("failed: {}",e),
    }
}


fn handle_client(stream: TcpStream){
    handle_read(&stream);
    handle_write(stream);
}

fn main(){
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Listening on {}",8080);
    
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                thread::spawn(||{
                    handle_client(stream)
                });
            }
            Err(e) => println!("Failed :{}",e),
        }
    }
}
