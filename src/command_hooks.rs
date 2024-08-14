use std::borrow::Cow;
use std::process::Command;
use std::process::Stdio;


pub struct CommandRx<'a> {
    success: bool,
    stdout: Cow<'a, String>,
    stderr: Cow<'a, String>,


    pub status: String,
    pub content: Option<Vec<String>>
}

impl<'a> CommandRx<'a> {

    fn new(input: &Vec<String>, args: &[&str]) -> Self {

        let cmd_in = Command::new("echo")
            .arg(input.join(""))
            .stdout(Stdio::piped())
            .spawn()
            .expect("command_hooks: fn new - error executing 'echo' command.");

        let args_iter = args.into_iter();
        let cmd_out = Command::new(args_iter.next().unwrap());
        for arg in args_iter {
            cmd_out.arg(arg);
        }
        cmd_out.stdin(cmd_in.stdout.unwrap());
        let output = cmd_out.output().expect("msg");

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let success = output.status.success();

        Self {  }
    }

    pub fn decrypt(message: &Vec<String>) -> Self {

        let gpg_input = Command::new("echo")
            .arg(message.join(""))
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        let command_output = match Command::new("gpg")
            .arg("-d")
            .stdin(gpg_input.stdout.unwrap())
            .output() {
                Ok(out) => { out },
                Err(_) => { return Self { success: false, status: "[TODO] Command Error!".into(), content: None } },
            };

        let stderr = String::from_utf8_lossy(&command_output.stderr);
        let stdout = String::from_utf8_lossy(&command_output.stdout);
    

        // Handle cases
        // **
        if command_output.status.success() {
            let output: Vec<String> = stdout.split('\n').map(|str| format!("{}\n", str)).collect();
            return Self { success: true, status: "[d] Sucessfully decrypted message!\n".into(), content: Some(output) }
        }

        if stderr.starts_with("gpg: no valid OpenPGP data found") {
            return Self { success: false, status: "[d] Message is not in a valid OpenPGP format!\n".into(), content: None }
        }

        Self { success: false, status: "[d] You are not a vaild recipient!\n".into(), content: None }

    }

    pub fn verify(message: &Vec<String>) -> Self  {

        // Get data
        let gpg_input = Command::new("echo")
                .arg(message.join(""))
                .stdout(Stdio::piped())
                .spawn()
                .expect("Code Error: verify()");
    
        let command_output = Command::new("gpg")
                .arg("--verify")
                .stdin(gpg_input.stdout.unwrap())
                .output()
                .expect("Code Error: verify()");
    
        let stderr = String::from_utf8_lossy(&command_output.stderr);
    
    
        // Handle cases
        if command_output.status.success() {
            let start = stderr.find("gpg: Good signature from").expect("Code Error: verify()");
            let mut output = &stderr[start + 25 ..];
    
            let end = output.find("]").expect("Code Error: verify()");
            output = &output[.. end + 1];
    
            return Self { success: true, status: format!("[v] Signed by {}\n", output).into(), content: None }
        }
    
        if stderr.starts_with("gpg: no valid OpenPGP data found") {
            return Self { success: false, status: "[v] Signature is not in a valid OpenPGP format!\n".into(), content: None }
        }
    
        Self { success: false, status: "[v] You are not a vaild recipient!\n".into(), content: None }
    
    }

}