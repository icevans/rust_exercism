fn uppercase_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn beer_noun_phrase(n: u32) -> String {
    match n {
        0 => "no more bottles of beer".to_string(),
        1 => "1 bottle of beer".to_string(),
        n => format!("{n} bottles of beer"),
    }
}

pub fn verse(n: u32) -> String {
    let mut first_line = format!(
        "{} on the wall, {}.\n",
        beer_noun_phrase(n),
        beer_noun_phrase(n)
    );

    let second_line: String;

    if n > 0 {
        second_line = format!(
            "Take {} down and pass it around, {} on the wall.\n",
            if n == 1 { "it" } else { "one" },
            beer_noun_phrase(n - 1)
        );
    } else {
        second_line =
            "Go to the store and buy some more, 99 bottles of beer on the wall.".to_string();
    }

    first_line.push_str(&second_line);
    uppercase_first_letter(&first_line)
}

pub fn sing(start: u32, end: u32) -> String {
    let mut song = "".to_string();

    for n in (end..start + 1).rev() {
        song.push_str(&verse(n));
        song.push_str("\n");
    }

    song
}
