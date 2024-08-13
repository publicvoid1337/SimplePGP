const ESC: char = 27 as char;


pub enum ScreenPart {
    Head,
    Body,
    Tail
}
use ScreenPart::{Head, Body, Tail};


pub struct ScreenState {
    head: Option<Vec<String>>,
    body: Option<Vec<String>>,
    tail: Option<Vec<String>>,
    longest_line_length: usize
}

impl ScreenState {
    pub fn new() -> Self {
        Self { head: None, body: None, tail: None, longest_line_length: 0 }
    }
    
    pub fn push(&mut self, part: ScreenPart, line: String) {
        let line_length = line.len();
        if line_length > self.longest_line_length {
            self.longest_line_length = line_length;
        }

        match part {
            Head => { 
                if self.head.is_none() {
                    self.head = Some(Vec::new())
                }
                self.head.as_mut()
                         .expect("Screenstate::push - Head was not created!")
                         .push(line);
            }

            Body => {
                if self.body.is_none() {
                    self.body = Some(Vec::new())
                }
                self.body.as_mut()
                         .expect("Screenstate::push - Body was not created!")
                         .push(line);
            }

            Tail => {
                if self.tail.is_none() {
                    self.tail = Some(Vec::new())
                }
                self.tail.as_mut()
                         .expect("Screenstate::push - Tail was not created!")
                         .push(line);
            }
        }
    }

    pub fn print_screen(&self) {

        //clear screen
        print!("{ESC}c");

        if let Some(head) = &self.head {
            for line in head {
                print!("{}", format!("{}{}\n", " ".repeat((self.longest_line_length - line.len()) / 2), line));
            }

            println!("\n{}\n", "=".repeat(self.longest_line_length));
        }

        if let Some(body) = & self.body {
            for line in body {
                print!("{}", line);
            }
        }

        if let Some(tail) = &self.tail {
            println!("\n{}\n", "=".repeat(self.longest_line_length));

            for line in tail {
                print!("{}", format!("{}{}", " ".repeat((self.longest_line_length - line.len()) / 2), line));
            }
        }

    }
}