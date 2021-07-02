use std::env;
use std::fs;

mod os;
mod replace;

fn main() {
    let binpath = match env::args().skip(1).next() {
        Some(path) => path,
        None => return,
    };
    let mut haystack = fs::read(&binpath).expect("Shomething went wrong reading the file");

    println!("Patching: {}", binpath);
    replace::by_random(&mut haystack, b"$cdc_asdjflasutopfhvcZLmcfl_");
    replace::by_random(&mut haystack, b"addScriptToEvaluateOnNewDocument");
    os::bufexec(&haystack, env::args().skip(1))
}
