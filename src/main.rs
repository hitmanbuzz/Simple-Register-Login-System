mod register;
mod login;
mod forgot;
mod menu;

use std::io;

use menu::menu;

fn main() {
    menu();
    
    // This is to make windows screen idle so that it doesn't auto close after the program is ran
    let mut idle = String::new();
    io::stdin()
        .read_line(&mut idle)
        .expect("Failed to keep windows idle");
}
