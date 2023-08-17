use std::{io::{stdout, Write, self}, fs::File};

pub fn registration() -> bool {

    // DB Create Part
    let _dir = "login.txt";
    let mut _create_db = File::create(_dir)
        .expect("Failed to create login.txt");
    _create_db.write_all(username().as_bytes())
        .expect("Failed to write username for register");
    _create_db.write_all(":".as_bytes())
        .expect("Failed to write splitter.");
    _create_db.write_all(password().as_bytes())
        .expect("Failed to write password for register");
    // HUID Register Part (todo)
    _create_db.write_all("[".as_bytes())
        .expect("Failed to write splitter.");
    _create_db.write_all(huid().as_bytes())
        .expect("Failed to write huid for register");
    _create_db.write_all("]".as_bytes())
        .expect("Failed to write splitter.");
    return true;
}

fn username() -> String {
    // Username register Part
    let mut _register_username = String::new();
    print!("Enter username for registration: ");
    stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut _register_username)
        .expect("Failed to read register username");
    let register_username = _register_username
        .trim();
    if register_username.is_empty() {
        println!("Username is empty");
        return register_username.to_owned();
    }
    return register_username.to_owned();
}

fn password() -> String {
    // Password register Part
    let mut _register_password = String::new();
    print!("Enter password for registration: ");
    stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut _register_password)
        .expect("Failed to read register password");
    let register_password = _register_password
        .trim();
    if register_password.is_empty() {
        println!("Password is empty");
        return register_password.to_owned();
    }
    return register_password.to_owned();
}

fn huid() -> String {
    // HUID register Part
    let mut _huid_register = String::new();
    print!("Enter HUID for registration: ");
    stdout()
        .flush()
        .unwrap();
    io::stdin()
        .read_line(&mut _huid_register)
        .expect("failed to read _huid_register");
    let huid_register = _huid_register
        .trim();
    if huid_register.is_empty() {
        println!("HUID is empty");
        return huid_register.to_owned();
    }
    return huid_register.to_owned();
}