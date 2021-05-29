use std::io::{self, Error, ErrorKind};
use std::process::{Command, ExitStatus, Output};
use crate::util_msg::{print_debug, print_error, MessageType, print_message};

pub fn run_command_or_exit(cmd: &str, args: &[&str]) -> Output {
    let mut c = Command::new(cmd);
    c.args(args);
    crate::util_msg::when(MessageType::DEBUG, || {
        print_debug(&format!("Run command: {:?}", c));
    });
    let output = c.output();
    match output {
        Err(e) => {
            print_error(&format!("Run command: {:?}, failed: {}", c, e));
            crate::util_runtime::invoke_callbacks();
            std::process::exit(-1);
        }
        Ok(output) => {
            if !output.status.success() {
                print_output(MessageType::ERROR, &output);
            }
            output
        }
    }
}

pub fn print_output(message_type: MessageType, output: &Output) {
    crate::util_msg::when(message_type, || {
        print_message(message_type, &format!(r##"Run command failed, code: {:?}
-----std out---------------------------------------------------------------
{}
-----std err---------------------------------------------------------------
{}
---------------------------------------------------------------------------"##,
                                             output.status.code(),
                                             String::from_utf8_lossy(&output.stdout),
                                             String::from_utf8_lossy(&output.stderr)));
    });
}

pub fn run_command_and_wait(cmd: &mut Command) -> io::Result<ExitStatus> {
    cmd.spawn()?.wait()
}

pub fn extract_package_and_wait(dir: &str, file_name: &str) -> io::Result<ExitStatus> {
    let mut cmd: Command;
    if file_name.ends_with(".zip") {
        cmd = Command::new("unzip");
    } else if file_name.ends_with(".tar.gz") {
        cmd = Command::new("tar");
        cmd.arg("-xzvf");
    } else {
        let m: &str = &format!("Unknown file type: {}", file_name);
        return Err(Error::new(ErrorKind::Other, m));
    }
    cmd.arg(file_name).current_dir(dir);
    run_command_and_wait(&mut cmd)
}

