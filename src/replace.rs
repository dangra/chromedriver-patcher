use memchr::memmem;
use rand::distr::{Alphanumeric, SampleString};

/// Replace all occurrences of `needle` with `replacement` in `haystack`
///
/// Returns the number of replacements performed
pub fn by(haystack: &mut [u8], needle: &[u8], replacement: &[u8]) -> usize {
    // Validate replacement length
    assert!(!needle.is_empty(), "Needle must be non-empty");
    assert_eq!(
        needle.len(),
        replacement.len(),
        "Replacement must be the same length as the needle"
    );

    // Find all occurrences
    let matches: Vec<usize> = memmem::find_iter(haystack, needle).collect();
    let replacement_count = matches.len();

    // Perform replacements
    for position in matches {
        let replace_slice = &mut haystack[position..position + needle.len()];
        replace_slice.copy_from_slice(replacement);
    }

    if replacement_count > 0 {
        log_replacement(replacement_count, needle, replacement);
    }

    replacement_count
}

/// Replace all occurrences of `needle` with random alphanumeric strings of the same length
///
/// Returns the number of replacements performed
pub fn by_random(haystack: &mut [u8], needle: &[u8]) -> usize {
    let replacement = generate_safe_identifier(needle.len());
    by(haystack, needle, &replacement)
}

/// Generate a random alphanumeric string that is safe to use as an identifier
fn generate_safe_identifier(length: usize) -> Vec<u8> {
    let mut rng = rand::rng();
    let mut sampled = Alphanumeric.sample_string(&mut rng, length).into_bytes();

    // Ensure the first character is not a digit (safer for identifiers)
    if sampled[0].is_ascii_digit() {
        sampled[0] = b'_';
    }

    sampled
}

/// Log information about the replacement operation
fn log_replacement(count: usize, needle: &[u8], replacement: &[u8]) {
    let needle_str = String::from_utf8_lossy(needle);
    let replacement_str = String::from_utf8_lossy(replacement);

    eprintln!(
        "Replaced {} occurrence{} of '{}' with '{}'",
        count,
        if count == 1 { "" } else { "s" },
        needle_str,
        replacement_str,
    );
}
