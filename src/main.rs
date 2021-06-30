/*
 * 1v Read chromedriver
 * 6v Replace CDC token
 * 2v Create FD memfd
 * 3v copy ChromeDriver
 * 4. Exec FD
 *
 * 5. Pass arguments
 */

use std::env;
use std::ffi::CString;
use std::fs;
use std::io::Write;
use std::os::unix::io::AsRawFd as _;

use nix;
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use regex::bytes::Regex;

use memfd;

const CHROMEDRIVERBIN: &str = "/usr/bin/chromedriver";
const DOCKEY: &str = r#"cdc_asdjflasutopfhvcZLmcfl_"#;

fn gen_alphanum(len: usize) -> String {
    let mut rng = thread_rng();
    return Alphanumeric {}.sample_string(&mut rng, len);
}

// Exec a binary directly from memory without writing to the filesystem
fn bufexec(buf: &[u8]) -> () {
    // Create destination fd in memory
    let opts = memfd::MemfdOptions::default()
        .close_on_exec(true)
        .allow_sealing(true);
    let mfd = opts.create("chromedriver").expect("Failed to create MemFD");
    mfd.as_file().write_all(buf).expect("Error writing to file");
    fs::write("destination", buf).expect("");

    let cargs: Vec<CString> = env::args().map(|s| CString::new(s).unwrap()).collect();
    let cvars: Vec<CString> = env::vars()
        .map(|(k, v)| CString::new(format!("{}={}", k, v)).unwrap())
        .collect();
    nix::unistd::fexecve(mfd.as_raw_fd(), &cargs, &cvars).expect("Failed to Exec patched binary");
}

fn main() {
    //let args: Vec<CString> = env::args().map(|s| {CString::new(s).unwrap()}).collect();
    let randkey = gen_alphanum(DOCKEY.len());
    println!("ChromeDriver: Replacing '{}' by '{}'", DOCKEY, randkey);
    let re = Regex::new(DOCKEY).expect("Failed to compile regular expression");

    let srcbuf = fs::read(CHROMEDRIVERBIN).expect("Shomething went wrong reading the file");
    let buf = re.replace(srcbuf.as_slice(), randkey.as_bytes());

    bufexec(&buf)
}
