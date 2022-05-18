use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let mut words_and_scores: Vec<(String, usize)> = vec![];
    let raw_word_list = make_vec_from_filenames(&[PathBuf::from("raw.txt")]);
    for word in raw_word_list {
        words_and_scores.push((word.to_string(), number_of_clicks(word)));
    }
    words_and_scores.sort_by(|a, b| a.1.cmp(&b.1));
    println!("{:?}", words_and_scores[2]);
    println!("{:?}", words_and_scores[18000]);

    let mut f = File::create("output.txt").expect("Unable to create file");
    for word_and_score in words_and_scores {
        writeln!(f, "{}", word_and_score.0).expect("Unable to write word to file");
    }

    println!(
        "Distance between q and e is {}",
        measure_distance_between_chars('q', 'e')
    );
    println!(
        "Distance between m and g is {}",
        measure_distance_between_chars('m', 'g')
    );
    println!(
        "Distance between 'wax' is {}",
        measure_distance_of_word("wax".to_string())
    );
}

fn make_map() -> HashMap<char, (usize, usize)> {
    let keyboard_map = HashMap::from([
        ('q', (0, 0)),
        ('w', (1, 0)),
        ('e', (2, 0)),
        ('r', (3, 0)),
        ('t', (4, 0)),
        ('y', (5, 0)),
        ('u', (6, 0)),
        ('i', (7, 0)),
        ('o', (8, 0)),
        ('p', (9, 0)),
        ('a', (0, 1)),
        ('s', (1, 1)),
        ('d', (2, 1)),
        ('f', (3, 1)),
        ('g', (4, 1)),
        ('h', (5, 1)),
        ('j', (6, 1)),
        ('k', (7, 1)),
        ('l', (8, 1)),
        ('z', (0, 2)),
        ('x', (1, 2)),
        ('c', (2, 2)),
        ('v', (3, 2)),
        ('b', (4, 2)),
        ('n', (5, 2)),
        ('m', (6, 2)),
    ]);

    return keyboard_map;
}

fn measure_distance_between_chars(char1: char, char2: char) -> usize {
    let map = make_map();
    let coordinates1 = map[&char1];
    let coordinates2 = map[&char2];
    ((coordinates1.0 as isize - coordinates2.0 as isize).abs()
        + (coordinates1.1 as isize - coordinates2.1 as isize).abs())
    .try_into()
    .unwrap()
}

fn measure_distance_of_word(word: String) -> usize {
    let mut distance = 0;
    let char_vec: Vec<char> = word.chars().collect();
    for c in char_vec.windows(2) {
        distance += measure_distance_between_chars(c[0], c[1]);
    }
    distance
}

fn number_of_clicks(word: String) -> usize {
    word.chars().count() + measure_distance_of_word(word)
}
/// Takes a slice of `PathBuf`s representing the word list(s)
/// that the user has inputted to the program. Then iterates
/// through each file and addes each line to Vec<String>. (Blank
/// lines and duplicate links will be handled elsewhere.)
pub fn make_vec_from_filenames(filenames: &[PathBuf]) -> Vec<String> {
    let mut word_list: Vec<String> = [].to_vec();
    for filename in filenames {
        let f = match File::open(filename) {
            Ok(file) => file,
            Err(e) => panic!("Error opening file {:?}: {}", filename, e),
        };
        let file = BufReader::new(&f);
        for line in file.lines() {
            let l = match line {
                Ok(l) => l,
                Err(e) => {
                    eprintln!(
                        "Error reading a line from file {:?}: {}\nWill continue reading file.",
                        filename, e
                    );
                    continue;
                }
            };
            word_list.push(l);
        }
    }
    word_list
}
