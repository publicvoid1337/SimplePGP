use std::process::exit;
use std::io;


mod renderer;
use renderer::{ApplicationState, ScreenPart};

mod command_hooks;
use command_hooks::{decrypt, verify};




fn main() {

    let mut screen = ApplicationState::new();

    /* Get message */
    screen.push(ScreenPart::Head, "Enter or paste your message and write ':q' when your finished.".to_string());
    screen.print_screen();

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

    for char in buffer.trim().chars() {
        match char {
           'd' => {
                //get_result(&mut message, decrypt, &mut tail);
                let cmd = command_hooks::CommandOutput::decrypt(&screen.body);
                screen.update_screen(cmd);
            },

            'v' => {
                //get_result(&mut message, verify, &mut tail);
                let cmd = command_hooks::CommandOutput::verify(&screen.body);
                screen.update_screen(cmd);
            },

            'q' => { exit(0) },

            _ => { exit(1) }
        }
    }

    screen.print_screen();

}