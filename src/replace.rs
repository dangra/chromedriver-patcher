use memchr::memmem;
use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;

pub fn by(haystack: &mut [u8], needle: &[u8], replaceby: &[u8]) {
    let matches: Vec<usize> = memmem::find_iter(&haystack[..], needle).collect();
    let times = matches.len();
    for m in matches {
        let replaceat = &mut haystack[m..m + needle.len()];
        replaceat.copy_from_slice(replaceby);
    }
    eprintln!(
        "Replaced {} time(s) '{}' by '{}'",
        times,
        String::from_utf8_lossy(needle),
        String::from_utf8_lossy(replaceby),
    )
}

pub fn by_random(haystack: &mut [u8], needle: &[u8]) {
    let replaceby = gen_alphanum(needle.len());
    by(haystack, needle, &replaceby);
}

fn gen_alphanum(len: usize) -> Vec<u8> {
    let mut rng = thread_rng();
    let mut sampled = Alphanumeric {}.sample_string(&mut rng, len).into_bytes();
    // Identifiers can't start with a number in most cases
    if b"012345789".contains(&sampled[0]) {
        sampled[0] = b'_';
    }
    return sampled;
}
