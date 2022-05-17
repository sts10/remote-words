use std::collections::HashMap;
fn main() {
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
