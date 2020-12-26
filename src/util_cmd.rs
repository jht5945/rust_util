use std::io::{self, Error, ErrorKind};
use std::process::{Command, ExitStatus};

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

