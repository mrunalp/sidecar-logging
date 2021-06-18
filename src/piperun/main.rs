use std::env;
use std::fs:: OpenOptions;
use std::os::unix::io::IntoRawFd;
use std::path::Path;
use std::ffi::CString;
use nix::unistd;
use nix::sys::stat;
use anyhow::Result;
use readable_perms::{Permissions, ChmodExt};

fn main() -> Result<()> {
    const STDOUT: i32 = 1;

    let args: Vec<CString> = env::args().map(|a| CString::new(a).unwrap()).collect();

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
    unistd::dup2(fd.into_raw_fd(), STDOUT)?;

    println!("Dup done, exec'ing..");
    unistd::execvp(&args[1], &args[2..])?;
    Ok(())
}