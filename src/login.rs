use std::{fs, io::{self, stdout, Write}};
use crate::forgot::huid_recovery;

pub fn login() -> bool {
    // Reading login detail
    let _login_dir = "login.txt";
    let login_detail = fs::read_to_string(_login_dir)
        .expect("failed to read login details");
    let _split_login_detail: Vec<&str> = login_detail.split(&[':', '['][..])
        .collect();
    let username_login = _split_login_detail[0];
    let password_login = _split_login_detail[1];
    
    if login_username() == username_login && login_password() == password_login {
        return true;
    }
    else {
        if huid_recovery() {
            return true;
        }
        else {
            return false;
        }
    }
    
}
fn login_username() -> String {
    // Username Part
    let mut _username = String::new();
    print!("Enter Username for login: ");
    stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut _username)
        .expect("failed to read _username login");
    let username = _username.trim();
    return username.to_owned();
}

fn login_password() -> String {
    // Password Part
    let mut _password = String::new();
    print!("Enter Password for login: ");
    stdout()
        .flush()
        .unwrap(); 
    io::stdin()
        .read_line(&mut _password)
        .expect("failed to read _username login");
    let password = _password.trim();
    return password.to_owned();
}