use std::collections::HashSet;

/// Strips the given prefixes from the input string
pub fn strip_prefixes<'a>(input: &'a str, prefixes: &[&str]) -> Option<&'a str> {
    prefixes
        .iter()
        .find(|&&prefix| input.starts_with(prefix))
        .and_then(|&prefix| input.strip_prefix(prefix))
}

/// Strips the given suffixes from the input string
pub fn strip_suffixes<'a>(input: &'a str, suffixes: &[&str]) -> Option<&'a str> {
    suffixes
        .iter()
        .find(|&&prefix| input.ends_with(prefix))
        .and_then(|&prefix| input.strip_suffix(prefix))
}

/// Trims duplicate lines from the given strings
pub fn trim_duplicate_lines(strings: Vec<String>) -> Vec<String> {
    let mut unique_lines: HashSet<&str> = HashSet::new();
    let mut result: Vec<String> = Vec::new();

    for string in strings.iter() {
        let unique_lines_in_string: Vec<_> = string
            .lines()
            .filter(|&line| {
                // Always include empty lines
                if line.trim().is_empty() {
                    true
                } else {
                    // Include non-empty line only if it's unique
                    unique_lines.insert(line)
                }
            })
            .collect();
        let unique_lines = unique_lines_in_string.join("\n");
        let unique_lines = replace_multiple_empty_lines(&unique_lines);
        result.push(unique_lines);
    }

    result
}

/// Replaces sequential empty lines with a single empty line
fn replace_multiple_empty_lines(input: &str) -> String {
    let mut result = String::new();
    let mut was_previous_line_empty = false;

    for line in input.lines() {
        if line.trim().is_empty() {
            if !was_previous_line_empty {
                // Add a single empty line
                result.push('\n');
                was_previous_line_empty = true;
            }
        } else {
            result.push_str(line);
            // Add the non-empty line
            result.push('\n');
            was_previous_line_empty = false;
        }
    }

    result
}
