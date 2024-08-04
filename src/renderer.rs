pub fn print_screen_staging(head: &Vec<String>, message: Option<&Vec<String>>, tail: Option<&Vec<String>>) {
    
    print!("{esc}c", esc = 27 as char);
    
    let mut longest_line_length: usize = 0;

    for line in head {
        if line.len() < longest_line_length { continue; }
        longest_line_length = line.len();
    }
    if let Some(message_unwrapped) = message {
        for line in message_unwrapped {
            if line.len() < longest_line_length { continue; }
            longest_line_length = line.len();
        }
    }
    if let Some(tail_unwrapped) = tail {
        for line in tail_unwrapped {
            if line.len() < longest_line_length { continue; }
            longest_line_length = line.len();
        }
    }

    for line in head {
        print!("{}", format!("{}{}\n", " ".repeat((longest_line_length - line.len()) / 2), line));
    }
    println!("\n{}\n", "=".repeat(longest_line_length));

    if let Some(message_unwrapped) = message {
        for line in message_unwrapped {
            print!("{}", line);
        }
    }

    if let Some(tail_unwrapped) = tail {
        println!("\n{}\n", "=".repeat(longest_line_length));

        for line in tail_unwrapped {
            print!("{}", format!("{}{}", " ".repeat((longest_line_length - line.len()) / 2), line));
        }
    }

}

pub fn print_screen_staging2(head: &Vec<String>, message: Option<&Vec<String>>, tail: Option<&Vec<String>>) {
    
    print!("{esc}c", esc = 27 as char);
    
    let mut longest_line_length: usize = 0;

    for line in head {
        if line.len() < longest_line_length { continue; }
        longest_line_length = line.len();
    }
    if let Some(message_unwrapped) = message {
        for line in message_unwrapped {
            if line.len() < longest_line_length { continue; }
            longest_line_length = line.len();
        }
    }
    if let Some(tail_unwrapped) = tail {
        for line in tail_unwrapped {
            if line.len() < longest_line_length { continue; }
            longest_line_length = line.len();
        }
    }

    for line in head {
        print!("{}", format!("{}{}\n", " ".repeat((longest_line_length - line.len()) / 2), line));
    }
    println!("\n{}\n", "=".repeat(longest_line_length));

    if let Some(message_unwrapped) = message {
        for line in message_unwrapped {
            print!("{}", line);
        }
    }

    if let Some(tail_unwrapped) = tail {
        println!("\n{}\n", "=".repeat(longest_line_length));

        for line in tail_unwrapped {
            print!("{}", format!("{}{}", " ".repeat((longest_line_length - line.len()) / 2), line));
        }
    }

}