use std::borrow::BorrowMut;
use std::process::exit;
use std::io;
use std::vec;


mod renderer;
use renderer::print_screen_staging;

mod command_hooks;
use command_hooks::{decrypt, verify};

fn main() {

    let mut PROGRAM_STATE = renderer::ScreenState::new();

    /* Get message */
    PROGRAM_STATE.head = Some(vec![String::from("Enter or paste your message and write ':q' when your finished.")]);
    //let head = vec![String::from("Enter or paste your message and write ':q' when your finished.")];
    renderer::print_screen_staging2(&PROGRAM_STATE);

    let mut message: Vec<String> = Vec::new();
    loop {
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        if &buffer == ":q\n" {
            break;
        }

        PROGRAM_STATE.push(buffer);
        //message.push(buffer);   // dont push buffer.trim() because '\n' is implemented in OpenPGP standard
        renderer::print_screen_staging2(&PROGRAM_STATE);        
        //print_screen_staging(&head, Some(&message), None);
    }
    /* ---------- */


    /* Get operation */
    let head = vec![String::from("Operations: d - decrypt, v - verify"), String::from("You can chain operations - 'dv' => decrypt and verify, 'ce' => clearsign and encrypt.")];
    let pseudo_tail: Vec<String> = Vec::new();
    
    print_screen_staging(&head, Some(&message), Some(&pseudo_tail));

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

    print_screen_staging(&head, Some(&message), Some(&tail));

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