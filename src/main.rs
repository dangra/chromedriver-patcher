use std::env;
use std::fs;

mod os;
mod replace;

fn main() {
    let mut args = env::args();
    let cmdname = args.next().unwrap();
    let binpath = match args.next() {
        Some(path) => path,
        None => {
            println!("{} [COMMAND] [ARGS]...", cmdname); 
            return
        },
    };
    let mut haystack = fs::read(&binpath).expect(&format!("Failed to read {}", binpath));

    println!("Patching: {}", binpath);
    replace::by_random(&mut haystack, b"$cdc_asdjflasutopfhvcZLmcfl_");
    replace::by_random(&mut haystack, b"addScriptToEvaluateOnNewDocument");
    os::bufexec(&haystack, env::args().skip(1))
}
