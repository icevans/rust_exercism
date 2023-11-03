use std::io::Cursor;
use unicode_reader::Graphemes;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::from("");

    for grapheme in Graphemes::from(Cursor::new(input)) {
        let grapheme = grapheme.unwrap();
        reversed.insert_str(0, &grapheme);
    }

    reversed
}
