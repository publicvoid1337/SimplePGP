use std::process::Command;
use std::process::Stdio;


struct CommandRxRaw {
    success: bool,
    stdout: String,
    stderr: String,
}

impl CommandRxRaw {

    fn new(input: &Vec<String>, args: &[&str]) -> Self {

        let cmd_in = Command::new("echo")
            .arg(input.join(""))
            .stdout(Stdio::piped())
            .spawn()
            .expect("command_hooks: fn new - error executing 'echo' command.");

        let mut args_iter = args.into_iter();
        let mut cmd_out = Command::new(args_iter.next().unwrap());
        for arg in args_iter {
            cmd_out.arg(arg);
        }
        cmd_out.stdin(cmd_in.stdout.unwrap());
        let output = cmd_out.output().expect("msg");

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let success = output.status.success();

        Self { success, stdout, stderr }
    }

}


pub struct CommandRx {
    pub status: String,
    pub content: Option<Vec<String>>
}

impl CommandRx {

    pub fn decrypt_staging(input: &Vec<String>) -> Self {

        let raw_output = CommandRxRaw::new(input, &["gpg", "-d"]);


        if raw_output.success {
            let output: Vec<String> = raw_output.stdout.split('\n').map(|str| format!("{}\n", str)).collect();
            return Self { status: "[d] Sucessfully decrypted message!\n".into(), content: Some(output) }
        }

        if raw_output.stderr.starts_with("gpg: no valid OpenPGP data found") {
            return Self { status: "[d] Message is not in a valid OpenPGP format!\n".into(), content: None }
        }

        Self { status: "[d] You are not a vaild recipient!\n".into(), content: None }

    }


    pub fn verify_staging(input: &Vec<String>) -> Self {

        let raw_output = CommandRxRaw::new(input, &["gpg", "--verify"]);


        if raw_output.success {
            let start = raw_output.stderr.find("gpg: Good signature from").expect("Code Error: verify()");
            let mut output = &raw_output.stderr[start + 25 ..];
    
            let end = output.find("]").expect("Code Error: verify()");
            output = &output[.. end + 1];
    
            return Self { status: format!("[v] Signed by {}\n", output).into(), content: None }
        }
    
        if raw_output.stderr.starts_with("gpg: no valid OpenPGP data found") {
            return Self { status: "[v] Signature is not in a valid OpenPGP format!\n".into(), content: None }
        }
    
        Self { status: "[v] You are not a vaild recipient!\n".into(), content: None }

    }

}