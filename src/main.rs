use std::env;
use std::fs;

mod bufexec;
mod replace;

fn main() {
    let binpath = match env::args().skip(1).next() {
        Some(path) => path,
        None => return,
    };
    println!("Patching binary: {}", binpath);
    println!("{:?}", env::args().skip(2).collect::<Vec<String>>());
    let mut haystack = fs::read(binpath).expect("Shomething went wrong reading the file");

    replace::by_random(&mut haystack, b"$cdc_asdjflasutopfhvcZLmcfl_");
    replace::by(
        &mut haystack,
        b"addScriptToEvaluateOnNewDocument",
        b"addScriptToEvaluateOnNewDoOOPSnt",
    );
    bufexec::bufexec(&haystack, env::args().skip(1))
}
