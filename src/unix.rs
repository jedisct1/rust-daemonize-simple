use std::env::set_current_dir;
use std::ffi::CString;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::mem;
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;

#[derive(Default, Debug, Clone)]
pub struct Daemonize {
    pub chdir: Option<PathBuf>,
    pub pid_file: Option<PathBuf>,
    pub stdin_file: Option<PathBuf>,
    pub stdout_file: Option<PathBuf>,
    pub stderr_file: Option<PathBuf>,
    pub umask: Option<libc::mode_t>,
    pub chroot: bool,
    pub append: bool,
}

impl Daemonize {
    unsafe fn _doit(self) -> Result<(), &'static str> {
        match libc::fork() {
            -1 => return Err("fork() failed"),
            0 => {}
            _ => {
                libc::_exit(0);
            }
        }
        libc::setsid();
        match libc::fork() {
            -1 => return Err("fork() failed"),
            0 => {}
            _ => {
                libc::_exit(0);
            }
        };

        if let Some(umask) = self.umask {
            libc::umask(umask);
        }
        if let Some(chdir) = &self.chdir {
            set_current_dir(chdir).map_err(|_| "chdir() failed")?;
        }

        let stdin_file = self.stdin_file.unwrap_or_else(|| "/dev/null".into());
        let fd = OpenOptions::new()
            .read(true)
            .open(&stdin_file)
            .map_err(|_| "Unable to open the stdin file")?;
        if libc::dup2(fd.as_raw_fd(), 0) == -1 {
            return Err("dup2(stdin) failed");
        }
        mem::forget(stdin_file);
        libc::close(fd.as_raw_fd());
        let stdout_file = self.stdout_file.unwrap_or_else(|| "/dev/null".into());
        let fd = OpenOptions::new()
            .create(true)
            .write(true)
            .append(self.append)
            .open(&stdout_file)
            .map_err(|_| "Unable to open the stdout file")?;
        if libc::dup2(fd.as_raw_fd(), 1) == -1 {
            return Err("dup2(stdout) failed");
        }
        mem::forget(stdout_file);
        libc::close(fd.as_raw_fd());
        let stderr_file = self.stderr_file.unwrap_or_else(|| "/dev/null".into());
        let fd = OpenOptions::new()
            .create(true)
            .write(true)
            .append(self.append)
            .open(&stderr_file)
            .map_err(|_| "Unable to open the stderr file")?;
        if libc::dup2(fd.as_raw_fd(), 2) == -1 {
            return Err("dup2(stderr) failed");
        }
        mem::forget(stderr_file);
        libc::close(fd.as_raw_fd());

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

        if let Some(chdir) = &self.chdir {
            if self.chroot {
                let chdir = CString::new(
                    chdir
                        .as_os_str()
                        .to_str()
                        .ok_or("Unexpected characters in chdir path")?,
                )
                .map_err(|_| "Unexpected chdir path")?;
                if libc::chroot(chdir.as_ptr()) != 0 {
                    return Err("chroot failed");
                }
                set_current_dir("/").map_err(|_| "chdir(\"/\") failed")?;
            } else {
                set_current_dir(chdir).map_err(|_| "chdir() failed")?;
            }
        }

        Ok(())
    }

    pub fn doit(self) -> Result<(), &'static str> {
        unsafe { self._doit() }
    }
}
