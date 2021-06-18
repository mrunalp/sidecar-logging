use std::fs:: OpenOptions;
use std::path::Path;
use std::os::unix::io::IntoRawFd;
use std::ffi::CString;
use nix::unistd;
use nix::sys::stat;
use anyhow::Result;
use readable_perms::{Permissions, ChmodExt};

fn main() -> Result<()> {
    const STDOUT: i32 = 1;

    let fifo_path = Path::new("/var/log/shared/stdout.pipe");
    if !fifo_path.exists() {
        unistd::mkfifo(fifo_path, stat::Mode::S_IRWXO)?;
        fifo_path.chmod(Permissions::from_mask(0o777))?;
    }

    println!("Opening fifo for duping");
    let fd = OpenOptions::new()
        .read(false)
        .write(true)
        .open(fifo_path)?;
    let raw_fd = fd.into_raw_fd();
    unistd::dup2(raw_fd, STDOUT)?;

    println!("Dup done, exec'ing..");
    let path = CString::new("/usr/bin/testlog")?;
    let args:  Vec<CString> = Vec::new();
    unistd::execvp(&path, &args)?;
    Ok(())
}