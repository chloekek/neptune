use np_init::mount_proc;
use np_init::mount_sys;
use np_unix::Signalfd;
use np_unix::Sigset;
use np_unix::sigprocmask;
use std::io::Result;
use std::process::Command;

fn main() -> Result<()>
{
    let mut sigset = Sigset::sigemptyset()?;
    sigset.sigaddset(libc::SIGCHLD)?;

    sigprocmask(libc::SIG_BLOCK, &sigset)?;

    let signalfd = Signalfd::signalfd(&sigset)?;

    mount_proc()?;
    mount_sys()?;

    let shell = Command::new("/bin/np-shelld").spawn()?;
    println!("{:?}", shell.id());

    let bash = Command::new("/bin/bash").spawn()?;
    println!("{:?}", bash.id());

    loop {
        let siginfo = signalfd.read()?;
        println!("{:?}", siginfo.ssi_pid);
    }
}
