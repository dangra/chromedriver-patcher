use std::env;
use std::fs;

mod os;
mod replace;

fn main() {
    let mut args = env::args();
    let arg0 = args.next().unwrap();
    let path = match args.next() {
        Some(v) => v,
        None => {
            println!("{} [COMMAND] [ARGS]...", arg0); 
            return
        },
    };
    // Read executable from file
    let mut binary = fs::read(&path).unwrap_or_else(|_| panic!("Failed to read {}", path));
    // Patch the executable
    eprintln!("Patching: {}", path);
    replace::by_random(&mut binary, b"$cdc_asdjflasutopfhvcZLmcfl_");
    replace::by_random(&mut binary, b"addScriptToEvaluateOnNewDocument");
    // Replace current proxess with the patched executable
    os::bufexec(&binary, env::args().skip(1))
}
