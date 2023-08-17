use std::io::{stdout, Write, self};
use crate::register::registration;
use crate::login::login;

pub fn menu() {
    print!("
[1] Register
[2] Login

[#] Choose: ");
    stdout()
        .flush()
        .unwrap();
    let mut _menu_option = String::new();
    io::stdin()
        .read_line(&mut _menu_option)
        .expect("Failed To read menu option");
    let menu_option = _menu_option.trim();


    // Back To Menu
    fn backmenu() {
        println!("\n");
        print!("Press `y` to go back to main menu: ");
        stdout()
            .flush()
            .unwrap();
        let mut backmenu = String::new();
        io::stdin()
            .read_line(&mut backmenu)
            .expect("Failed to read `backMenu`");
        let backmenu = backmenu.trim();

        if backmenu == "y" || backmenu == "y".to_uppercase() {
            menu();
        }
        else {
            
        }
    }

    // Choose option
    if menu_option == "1" {
        if registration() {
            println!("Registration Done!!!");
            backmenu();
        }
    }
    else if menu_option == "2" {
        if login() {
            println!("Login Successfull");
            backmenu();
        }
        else {
            println!("Login Failed Due to Wrong Login Details Provided");
        }
    }
    else {
        println!("Wrong Menu Option!!!");
    }
}