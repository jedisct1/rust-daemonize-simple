use std::env::set_current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct Daemonize {
    pub chdir: Option<PathBuf>,
    pub pid_file: Option<PathBuf>,
    pub stdin_file: Option<PathBuf>,
    pub stdout_file: Option<PathBuf>,
    pub stderr_file: Option<PathBuf>,
    pub chroot: bool,
    pub append: bool,
}

impl Daemonize {
    unsafe fn _doit(self) -> Result<(), &'static str> {
        if let Some(chdir) = &self.chdir {
            set_current_dir(chdir).map_err(|_| "chdir() failed")?;
        }

        if let Some(pid_file) = self.pid_file {
            let pid = match libc::getpid() {
                -1 => return Err("getpid() failed"),
                pid => pid,
            };
            let pid_str = format!("{}", pid);
            File::create(pid_file)
                .map_err(|_| "Creating the PID file failed")?
                .write_all(pid_str.as_bytes())
                .map_err(|_| "Writing to the PID file failed")?;
        }

        Ok(())
    }

    pub fn doit(self) -> Result<(), &'static str> {
        unsafe { self._doit() }
    }
}
