use std::{fs, io::{stdout, Write, self}};

pub fn huid_recovery() -> bool {
    let _login_dir = "login.txt";
    let login_detail = fs::read_to_string(_login_dir)
        .expect("failed to read login details");
    let _split_login_detail: Vec<&str> = login_detail.split("[")
        .collect();
    let _huid = _split_login_detail[1];
    let huid = _huid.replace("]", "");


    // Forgot Part
    let mut _huid_input = String::new();
    print!("Enter your HUID: ");
    stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut _huid_input)
        .expect("failed to read _huid_input");
    let huid_input = _huid_input.trim();
    if huid_input == huid {
        return true;
    }
    else {
        println!("Wrong HUID!!!");
        return false;
    }
}