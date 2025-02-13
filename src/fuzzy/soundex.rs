pub(crate) fn phoneme_encode(word: &str) -> String {
    let acc = word
        .chars()
        .fold(String::with_capacity(word.len()), |mut acc, letter| 
            if acc.is_empty() {
                acc.push(letter.to_ascii_uppercase());
                acc
            } else {
                let length = acc.len();
                let previous = if length > 1 {
                    acc.chars().nth(length - 1).unwrap()
                } else {
                    ' ' // Dummy value
                };

                match (previous, letter) {
                    ('1', 'b' | 'f' | 'p' | 'v') => return acc,
                    ('2', 'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z') => return acc,
                    ('3', 'd' | 't') => return acc,
                    ('4', 'l') => return acc,
                    ('5', 'm' | 'n') => return acc,
                    ('6', 'r') => return acc,
                    (_, 'b' | 'f' | 'p' | 'v') => acc.push(char::from_digit(1, 10).unwrap()),
                    (_, 'c' | 'g' | 'j' | 'k' | 'q' | 's' | 'x' | 'z') => acc.push(char::from_digit(2, 10).unwrap()),
                    (_, 'd' | 't') => acc.push(char::from_digit(3, 10).unwrap()),
                    (_, 'l') => acc.push(char::from_digit(4, 10).unwrap()),
                    (_, 'm' | 'n') => acc.push(char::from_digit(5, 10).unwrap()),
                    (_, 'r') => acc.push(char::from_digit(6, 10).unwrap()),
                    _ => acc.push(letter.to_ascii_uppercase()),
                }

                acc
            });

    let mut numeric = acc
        .chars()
        .skip(1)
        .fold(String::with_capacity(acc.len()), |mut acc, letter| {
            let previous = if acc.len() > 1 {
                acc.chars().nth(acc.len() - 1).unwrap()
            } else {
                ' ' // Dummy value
            };

            let previous_previous = if acc.len() > 2 {
                acc.chars().nth(acc.len() - 2).unwrap()
            } else {
                ' ' // Dummy value
            };

            match (previous_previous, previous, letter) {
                (_, 'A' | 'E' | 'I' | 'O' | 'U' | 'Y', _) => acc.push(letter),
                (prev, 'H' | 'W', current) if prev == current => return acc,
                (prev, 'H' | 'W', current) if prev != current => acc.push(letter),
                _ => acc.push(letter),
            }
            
            acc
        })
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>();

    numeric = match numeric.len() {
        0 => "000".to_string(),
        1 => format!("{}00", numeric),
        2 => format!("{}0", numeric),
        _ => numeric[..3].to_string(),
    };

    format!("{}{}",
        acc.chars().nth(0).unwrap(), // First character
        numeric // Numeric characters
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phoneme_encode() {
        assert_eq!(phoneme_encode("stewart"), phoneme_encode("stuart")); // Similar sounding
        assert_eq!(phoneme_encode("allricht"), "A462");
        assert_eq!(phoneme_encode("hanselmann"), "H524");
        assert_eq!(phoneme_encode("roses"), "R220"); // Vowel letter separator
        assert_eq!(phoneme_encode("carwruth"), "C630"); // H or W letter separator
        assert_eq!(phoneme_encode("gutierrez"), "G362"); // Double consonant
        assert_eq!(phoneme_encode("campbell"), "C514"); // Same type of consonant
        assert_eq!(phoneme_encode("zita"), "Z300"); // Single consonant
        assert_eq!(phoneme_encode("schafer"), phoneme_encode("shaeffer")); // Similar sounding
    }
}