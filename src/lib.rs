#[macro_use] extern crate lazy_static;
extern crate regex;
extern crate ansi_term;

use regex::Regex;

#[test]
fn test_split_red() {
    use ansi_term::Colour::Red;
    let red_string = Red.paint("a red string").to_string();
    red_string.chars().for_each(|c| {
        println!("{:?}", c);
    });
    println!("Smeeeeegle {}", red_string);
}


#[cfg(test)]
mod tests {
}
