use std::str;
use std::env;
use std::ascii::AsciiExt;

mod search;

const UTF_DATA: &'static [u8] = include_bytes!("utf-data.txt");

fn take_line(m: &[u8], i: usize) -> &[u8] {
    let mut start_find = i;
    while start_find > 0 {
        start_find -= 1;
        if m[start_find] == b'\n' {
            start_find += 1;
            break;
        }
    }

    let mut end_find = i + 1;
    while let Some(&b) = m.get(end_find) {
        if b == b'\n' { break; }
        end_find += 1;
    }

    &m[start_find .. end_find]
}

fn transform_arg(a: &str) -> String {
    if a.len() == 1 {
        return format!("\n{} ", a);
    }

    a.chars().map(|c| match c {
        '-' | '_' => ' ',
        'a' ... 'z' => c.to_ascii_uppercase(),
        _ => c
    }).collect()
}

fn contains(needle: &[u8], haystack: &[u8]) -> bool {
    haystack.windows(needle.len()).any(|x| x == needle)
}

fn main() {
    let argv = env::args().skip(1);

    let args: Vec<_> = argv
        .map(|a| transform_arg(&a))
        .collect();

    if let Some((head, tail)) = args.split_first() {
        let indices = search::fast_search(head.as_bytes(), UTF_DATA);

        let lines = indices.into_iter()
            .map(|i| take_line(UTF_DATA, i as usize))
            .filter(|l| 
                tail.iter().all(|k| contains(k.as_bytes(), l)))
            .flat_map(|x| str::from_utf8(x));

        let mut last = "";
        for line in lines {
            if line != last {
                println!("{}", line);
                last = line;
            }
        }
    }
}
