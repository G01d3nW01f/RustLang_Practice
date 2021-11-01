use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

const LOCAL: &str = "127.0.0.1:6000";
const MSG_SIZE: usize = 32;


fn banner_show(){
    
    let banner = "
    :::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
    ::::        :::          :::          :::::  :::::::::::: :::
    ::: ::::::::::: :::::::::::: ::::::::   ::::  :::::::::: ::::
    ::: ::::::::::: :::::::::::: ::::::::  :::::: ::::::::: :::::
    ::::: :::::::::          :::          :::::::: ::::::: ::::::
    ::::::     :::: :::::::::::: ::::::  :::::::::: ::::: :::::::
    :::::::::: :::: :::::::::::: :::::::  :::::::::: ::: ::::::::
    ::::        :::          ::: ::::::::  ::::::::::   :::::::::
    :::::::::::::::::::::::::::::::::::::::::::::::::::::::::::::
    ";
    println!("{}",banner);
}


fn sleep(){
    thread::sleep(::std::time::Duration::from_millis(100));
}

fn main(){

    banner_show();
    println!("[+]StartServer!!!!!Woohoooo");
    let server = TcpListener::bind(LOCAL).expect("[!]ListenerFailed!!!");
    server.set_nonblocking(true).expect("[!]Failed to initialize non-blocking!");
    
    let mut clients = vec![];
    let (tx,rx) = mpsc::channel::<String>();
    loop{
        if let Ok((mut socket,addr)) = server.accept(){
            println!("[+]Client {} Connected",addr);
            
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("[!]Failed_to_Clone_Client!!!!!"));
            
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                match socket.read_exact(&mut buff){
                    Ok(_) => {
                        let msg = buff.into_iter().take_while(|&x| x != 0).collect::<Vec<_>>();
                        let msg = String::from_utf8(msg).expect("[!]Invalid utf8");
                        
                        println!("{}: {:?}",addr,msg);
                        tx.send(msg).expect("[!]Failed send msg");
                    },
                    Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("[-]Closing connection with: {}",addr);
                        break;
                    }
                }
                sleep();
            });
        }
        if let Ok(msg) = rx.try_recv(){
            clients = clients.into_iter().filter_map(|mut client|{
                let mut buff = msg.clone().into_bytes();
                buff.resize(MSG_SIZE,0);
                
                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }
        sleep();
    }
}
