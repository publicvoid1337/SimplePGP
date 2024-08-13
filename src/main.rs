use std::process::exit;
use std::io;


mod renderer;
use renderer::{ScreenState, ScreenPart};

mod command_hooks;
use command_hooks::{decrypt, verify};

fn main() {

    let mut screen = ScreenState::new();

    /* Get message */
    screen.push(ScreenPart::Head, "Enter or paste your message and write ':q' when your finished.".to_string());
    screen.print_screen();

    let mut message: Vec<String> = Vec::new();
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        if &buffer == ":q\n" {
            break;
        }

        screen.push(ScreenPart::Body, buffer);   // dont push buffer.trim() because '\n' is implemented in OpenPGP standard
        screen.print_screen();
    }
    /* ---------- */


    /* Get operation */
    screen.clear(ScreenPart::Head);
    screen.push(ScreenPart::Head, "Operations: d - decrypt, v - verify".to_string());
    screen.push(ScreenPart::Head, "You can chain operations - 'dv' => decrypt and verify, 'ce' => clearsign and encrypt.".to_string());  
    screen.push(ScreenPart::Tail, "".to_string());  
    screen.print_screen();

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    let mut tail: Vec<String> = Vec::new();

    for char in buffer.trim().chars() {
        match char {
           'd' => {
                get_result(&mut message, decrypt, &mut tail);
            },

            'v' => {
                get_result(&mut message, verify, &mut tail);
            },

            'q' => { exit(0) },

            _ => { exit(1) }
        }
    }

    screen.print_screen();

}

/* Housekeeping */
fn get_result(message: &mut Vec<String>, function: fn(&Vec<String>) -> Result<(String, Option<Vec<String>>), String>, tail: &mut Vec<String>) {

    match function(message) {
        Ok(res) => {
            if res.1.is_some() {
                *message = res.1.unwrap();
            }
            (*tail).push(res.0);
        },

        Err(err) => {
            (*tail).push(err);
        }
    }
}