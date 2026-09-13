// https://en.wikipedia.org/wiki/The_Twelve_Days_of_Christmas_(song)
//
// Just doing the first 4 days as it is pretty repetitive.

const COMPONENTS: [(&str, &str); 4] = [
    ("first", "A partridge in a pear tree."),
    ("second", "Two turtle doves"),
    ("third", "Three French hens"),
    ("fourth", "Four calling birds"),
];

fn main() {
    println!("{}", make_text());
}

fn make_text() -> String {
    let mut accum = String::new();
    let mut result = String::new();
    for element in COMPONENTS {
        let (count, gift) = element;
        let day = format!("On the {count} day of Christmas my true love sent to me\n");
        if !result.is_empty() {
            result.push_str("\n")
        }
        let mut accum_was_empty = true;
        if accum.is_empty() {
            accum.push_str(gift)
        } else {
            accum.insert_str(0, &format!("{gift},\n"));
            accum_was_empty = false;
        }
        result.push_str(&day);
        result.push_str(&accum);
        if accum_was_empty {
            accum = accum.replacen("A ", "And a ", 1);
        }
    }
    return result
}

#[cfg(test)]
mod tests {
    const TEXT: &str = "On the first day of Christmas my true love sent to me
A partridge in a pear tree.
On the second day of Christmas my true love sent to me
Two turtle doves,
And a partridge in a pear tree.
On the third day of Christmas my true love sent to me
Three French hens,
Two turtle doves,
And a partridge in a pear tree.
On the fourth day of Christmas my true love sent to me
Four calling birds,
Three French hens,
Two turtle doves,
And a partridge in a pear tree.";
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_make_text() {
        assert_eq!(TEXT, make_text());
    }
}
