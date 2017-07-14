use std::iter;

type ShiftRules = [u8; 256];

/*
pub fn brute_search(needle: &[u8], haystack: &[u8]) -> Vec<i32> {
    haystack
        .windows(needle.len())
        .enumerate()
        .filter(|&(_,x)| x == needle)
        .map(|(x,_)| (x + needle.len() - 1) as i32)
        .collect()
}
*/

pub fn fast_search(needle: &[u8], haystack: &[u8]) -> Vec<i32> {
    let len = needle.len();
    if len == 0 || len > 256 {
        return vec![];
    }

    // Boyer-Moore
    let rules = build_shift_rules(needle);

    let xs = iter::repeat(()).scan(0, |index, _| {
        let next = find(&rules, needle, haystack, *index);
        if let Some(i) = next {
            *index = i + 1;
        }
        next
    });

    xs.map(|x| x as i32).collect()
}

fn build_shift_rules(term: &[u8]) -> ShiftRules {
    let term_len = term.len();
    let mut rules: ShiftRules = [term_len as u8; 256];

    if term_len > 256 {
        // Don't even try
        return rules;
    }

    for (i, &b) in term.iter().enumerate() {
        let shift = term_len - 1 - i;
        rules[b as usize] = shift as u8;
    }

    rules
}

fn find(
    rules: &ShiftRules,
    needle: &[u8],
    haystack: &[u8],
    start_index: usize
) -> Option<usize> {
    let n = needle.len();

    let mut index = n - 1 + start_index;

    while let Some(&b) = haystack.get(index) {
        let shift = rules[b as usize];

        if shift != 0 {
            index += shift as usize;
        } else {
            if check_match(needle, haystack, index) {
                return Some(index);
            }
            index += 1;
        }
    }

    None
}

fn check_match(needle: &[u8], haystack: &[u8], end_index: usize) -> bool {
    let start = end_index - needle.len() + 1;
    let end = end_index + 1;
    let substring = &haystack[start .. end];

    substring == needle
}
