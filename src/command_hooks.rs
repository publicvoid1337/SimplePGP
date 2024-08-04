use std::process::Command;
use std::process::Stdio;


pub struct RxMessage {
    success: bool,
    status: String,
    content: Option<Vec<String>>
}

pub fn decrypt(message: &Vec<String>) -> Result<(String, Option<Vec<String>>), String> {

    let command = Command::new("echo")
        .arg(message.join(""))
        .arg("|")
        .arg("gpg")
        .arg("-d")
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&command.stderr);
    let stdout = String::from_utf8_lossy(&command.stdout);
    

    // Handle cases
    if command.status.success() {
        let output: Vec<String> = stdout.split('\n').map(|str| format!("{}\n", str)).collect();
        return Ok((String::from("[d] Sucessfully decrypted message!\n"), Some(output)))
    }

    if stderr.starts_with("gpg: no valid OpenPGP data found") {
        return Err(String::from("[d] Message is not in a valid OpenPGP format!\n"))
    }
    Err(String::from("[d] You are not a vaild recipient!\n"))

}

pub fn verify(message: &Vec<String>) -> Result<(String, Option<Vec<String>>), String> {

    // Get data
    let message_pipe = Command::new("echo")
        .arg(message.join(""))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let raw_output = Command::new("gpg")
        .arg("--verify")
        .stdin(message_pipe.stdout.unwrap())
        .output()
        .unwrap();

    let stderr = String::from_utf8_lossy(&raw_output.stderr);


    // Handle cases
    if raw_output.status.success() {
        let start = stderr.find("gpg: Good signature from").expect("msg");
        let mut output = &stderr[start + 25 ..];

        let end = output.find("]").expect("msg");
        output = &output[.. end + 1];

        return Ok((format!("[v] Signed by {}\n", output), None))
    }

    if stderr.starts_with("gpg: no valid OpenPGP data found") {
        return Err(String::from("[v] Signature is not in a valid OpenPGP format!\n"))
    }
    Err(String::from("[v] You are not a vaild recipient!\n"))

}


pub fn staging_decrypt(message: &Vec<String>) -> RxMessage {

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
        Err(_) => { return RxMessage { success: false, status: "[TODO] Command Error!".into(), content: None } },
    };

    let stderr = String::from_utf8_lossy(&command_output.stderr);
    let stdout = String::from_utf8_lossy(&command_output.stdout);
    

    // Handle cases
    // **
    if command_output.status.success() {
        let output: Vec<String> = stdout.split('\n').map(|str| format!("{}\n", str)).collect();
        return RxMessage { success: true, status: "[d] Sucessfully decrypted message!\n".into(), content: Some(output) }
    }

    if stderr.starts_with("gpg: no valid OpenPGP data found") {
        return RxMessage { success: false, status: "[d] Message is not in a valid OpenPGP format!\n".into(), content: None }
    }
    RxMessage { success: false, status: "[d] You are not a vaild recipient!\n".into(), content: None }

}

pub fn staging_verify(message: &Vec<String>) -> RxMessage  {

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

        return RxMessage { success: true, status: format!("[v] Signed by {}\n", output).into(), content: None }
    }

    if stderr.starts_with("gpg: no valid OpenPGP data found") {
        return RxMessage { success: false, status: "[v] Signature is not in a valid OpenPGP format!\n".into(), content: None }
    }

    RxMessage { success: false, status: "[v] You are not a vaild recipient!\n".into(), content: None }

}