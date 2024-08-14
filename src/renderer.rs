const ESC: char = 27 as char;
use crate::command_hooks::CommandOutput;


pub enum ScreenPart {
    Head,
    Body,
    Tail
}
use ScreenPart::{Head, Body, Tail};


pub struct ApplicationState {
    head: Vec<String>,
    pub body: Vec<String>,
    tail: Vec<String>,
    max_len: usize
}

impl ApplicationState {
    pub fn new() -> Self {
        Self { head: Vec::new(), body: Vec::new(), tail: Vec::new(), max_len: 0 }
    }
    
    pub fn push(&mut self, part: ScreenPart, line: String) {

        // keep track of longest line length in print stack
        let line_length = line.len();
        if line_length > self.max_len {
            self.max_len = line_length;
        }

        match part {
            Head => { self.head.push(line) }
            Body => { self.body.push(line) }
            Tail => { self.tail.push(line) }
        }
    }

    pub fn append(&mut self, part: ScreenPart, lines: &mut Vec<String>) {

        for line in lines.iter() {
            let line_length = line.len();
            if line_length > self.max_len {
                self.max_len = line_length;
            }
        }
        
        match part {
            Head => { self.head.append(lines) }
            Body => { self.body.append(lines) }
            Tail => { self.tail.append(lines) }
        }

    }

    pub fn clear(&mut self, part: ScreenPart) {

        match part {
            Head => { self.head = Vec::new() }
            Body => { self.body = Vec::new() }
            Tail => { self.tail = Vec::new() }
        }

    }

    pub fn print_screen(&self) {

        // clear screen
        print!("{ESC}c");

        // print head
        for line in self.head.iter() {
            print!("{}", format!("{}{}\n", " ".repeat((self.max_len - line.len()) / 2), line));
        }
        if !self.head.is_empty() {
            println!("\n{}\n", "=".repeat(self.max_len));
        }

        // print body
        for line in self.body.iter() {
            print!("{}", line);
        }

        //print tail
        if !self.tail.is_empty() {
            println!("\n{}\n", "=".repeat(self.max_len));
        }
        for line in self.tail.iter() {
            print!("{}", format!("{}{}", " ".repeat((self.max_len - line.len()) / 2), line));
        }

    }

    // consumes CommandOutput Object
    pub fn update_screen(&mut self, mut command: CommandOutput) {

        self.push(Tail, command.status);
        
        // might be wrong
        if let Some(content) = &mut command.content {
            content.pop();
            self.clear(Body);
            self.append(Body, content)
        }

    }

}