use std::collections::HashMap;
use std::iter::Iterator;
use std::iter::FromIterator;

fn is_valid_passphrase(passphrase: &str) -> bool {
    let mut words_seen = HashMap::new();
    for word in passphrase.split_ascii_whitespace() {
        if words_seen.contains_key(word) {
            return false;
        }

        words_seen.insert(word, 1);
    }
    true
}

fn is_valid_passphrase_anagrams(passphrase: &str) -> bool {
    let mut seen = HashMap::new();
    for word in passphrase.split_ascii_whitespace() {
        let mut sorted: Vec<char> = word.chars().collect();
        sorted.sort_by(|a, b| b.cmp(a));

        let new_word = String::from_iter(sorted);

        if seen.contains_key(&new_word) {
            return false;
        }

        seen.insert(new_word, 1);
    }

    true
}

#[aoc(day4, part1)]
pub fn day4_part1(input: &str) -> i32 {
    let mut n_valid = 0;
    for line in input.lines() {
        if is_valid_passphrase(line) {
            n_valid += 1;
        }
    }

    n_valid
}

#[aoc(day4, part2)]
pub fn day4_part2(input: &str) -> i32 {
    input.lines()
    .map(|l| {
        if is_valid_passphrase_anagrams(l) {
            1
        } else {
            0
        }
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::{is_valid_passphrase, is_valid_passphrase_anagrams};

    #[test]
    fn example1() {
        assert!(is_valid_passphrase("aa bb cc dd ee"));
    }

    #[test]
    fn example2() {
        assert!(!is_valid_passphrase("aa bb cc dd aa"));
    }

    #[test]
    fn example3() {
        assert!(is_valid_passphrase("aa bb cc dd aaa"));
    }

    #[test]
    fn example4() {
        assert!(is_valid_passphrase_anagrams("abcde fghij"));
    }

    #[test]
    fn example5() {
        assert!(!is_valid_passphrase_anagrams("abcde xyz ecdab"));
    }

    #[test]
    fn example6() {
        assert!(is_valid_passphrase_anagrams("a ab abc abd abf abj"));
    }

    #[test]
    fn example7() {
        assert!(is_valid_passphrase_anagrams("iiii oiii ooii oooi oooo"));
    }

    #[test]
    fn example8() {
        assert!(!is_valid_passphrase_anagrams("oiii ioii iioi iiio"));
    }
}