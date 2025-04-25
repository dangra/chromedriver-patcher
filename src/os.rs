pub use self::_os::bufexec;

#[cfg(not(target_os = "linux"))]
mod _os {
    pub fn bufexec(buf: &[u8], args: impl Iterator<Item = String>) {
        std::fs::write("destination", buf).expect("");
        eprintln!("{:?}", args.collect::<Vec<String>>());
    }
}

#[cfg(target_os = "linux")]
mod _os {
    use std::env;
    use std::ffi::CString;
    use std::io::Write;
    use std::os::unix::io::AsRawFd as _;

    // Exec a binary directly from memory without writing to the filesystem
    pub fn bufexec(buf: &[u8], args: impl Iterator<Item = String>) {
        // Create destination fd in memory
        let mfd = memfd::MemfdOptions::default()
            .close_on_exec(true)
            .create("chromedriver")
            .expect("Failed to create MemFD");

        // Write buffer to fd
        mfd.as_file()
            .write_all(buf)
            .expect("Error writing to memory fd");

        // Prepare args, envs and exec fd
        let cargs = args.map(|s| CString::new(s).unwrap()).collect::<Vec<_>>();
        let cvars = env::vars()
            .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
            .collect::<Vec<_>>();

        nix::unistd::fexecve(mfd.as_raw_fd(), &cargs, &cvars)
            .expect("Failed to Exec patched binary");
    }
}
