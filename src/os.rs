pub use self::os::bufexec;

#[cfg(not(target_os = "linux"))]
mod os {
    use std::fs;

    pub fn bufexec(buf: &[u8], args: impl Iterator<Item = String>) -> () {
        fs::write("destination", buf).expect("");
        eprintln!("{:?}", args.collect::<Vec<String>>());
        
    }
}

#[cfg(target_os = "linux")]
mod os {
    use memfd;
    use nix;
    use std::env;
    use std::ffi::CString;
    use std::io::Write;
    use std::os::unix::io::AsRawFd as _;

    // Exec a binary directly from memory without writing to the filesystem
    pub fn bufexec(buf: &[u8], args: impl Iterator<Item = String>) -> () {
        // Create destination fd in memory
        let opts = memfd::MemfdOptions::default().close_on_exec(true);
        let mfd = opts.create("chromedriver").expect("Failed to create MemFD");
        // Write buffer to fd
        mfd.as_file().write_all(buf).expect("Error writing to file");
        // Prepare args, envs and exec fd
        let cargs: Vec<CString> = args.map(|s| CString::new(s).unwrap()).collect();
        let cvars: Vec<CString> = env::vars()
            .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
            .collect();
        nix::unistd::fexecve(mfd.as_raw_fd(), &cargs, &cvars)
            .expect("Failed to Exec patched binary");
    }
}
