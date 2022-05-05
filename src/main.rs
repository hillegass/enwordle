use array2d::Array2D;
use std::cmp::min;
use std::fs::File;
use std::io::*;
use std::str;
use std::usize;

const WORD_LEN: usize = 5;
const WORD_COUNT: usize = 12947;
const GROUP_COUNT: usize = 243; // 3^5
type Word = [u8; WORD_LEN];
type WordIndex = usize;
type Group = u8;

// If the right answer were target and you input trying, the group
// is a number that represents the feedback from Wordle.
fn group_for_try_target(trying: &Word, target: &Word) -> Group {
    let mut checkword = [0; WORD_LEN];

    // Which letters are in the right place?
    for i in 0..WORD_LEN {
        checkword[i] = 2 * (trying[i] == target[i]) as u8;
    }

    // Check for letters that are in the wrong place
    for i in 0..WORD_LEN {
        if checkword[i] == 0 {
            let looking_for = trying[i];
            let search_result = target.iter().position(|&r| r == looking_for);
            checkword[i] = if search_result.is_none() { 0 } else { 1 }
        }
    }

    // Convert to a number
    let mut group = 0;
    for i in 0..WORD_LEN {
        group = 3 * group + checkword[i];
    }

    // Return the number
    group
}

// Given the table of groups, the indexes of the possible answers, return the index of the answers
// that will give you the most entropy
fn entropy_list(groups: &Array2D<u8>, conforming: &Vec<WordIndex>) -> Vec<(WordIndex, f32)> {
    let list_len = conforming.len();
    let mut entropy_list = Vec::with_capacity(list_len);

    // Go through all the possibilities
    for current_word_index in conforming {
        // Figure out how many remaining possibilities would be in each group
        let mut group_histogram = [0; GROUP_COUNT];
        for other_word_index in conforming {
            let group = groups[(*current_word_index, *other_word_index)];
            group_histogram[group as usize] += 1;
        }

        // Use that to compute the entropy
        let mut entropy = 0.0;
        for group in 0..GROUP_COUNT {
            let group_count = group_histogram[group];
            if group_count > 0 {
                let p = group_count as f32 / list_len as f32;
                entropy -= p * p.log2();
            }
        }
        // Save it in the list
        entropy_list.push((*current_word_index, entropy));
    }
    // Sort the list by entropy (descending order)
    entropy_list.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    // Return the list
    entropy_list
}

fn main() {
    // Read in the word list
    let mut words: Vec<Word> = Vec::with_capacity(WORD_COUNT);
    let mut word: Word = [0 as u8; WORD_LEN];
    let mut f = BufReader::new(File::open("words.txt").expect("open failed"));
    while f.read_exact(&mut word).is_ok() {
        words.push(word);
        f.seek(SeekFrom::Current(1)).expect("seek failed");
    }
    let word_count = words.len();
    println!("Read {} words. Making table...", word_count);

    // Make a table that holds the feedback from Wordle for every possible pair of words
    let mut groups = Array2D::filled_with(0 as u8, word_count, word_count);
    for i in 0..word_count {
        for j in 0..word_count {
            groups[(i, j)] = group_for_try_target(&words[i], &words[j]);
        }
    }

    // At first, all the words are possible answers
    let mut conforming = Vec::new();
    for i in 0..word_count {
        conforming.push(i);
    }

    // Keep going until the user quits
    loop {
        let possible_picks = entropy_list(&groups, &conforming);
        println!("{} possibilities", possible_picks.len());

        let rows_to_print = min(9, possible_picks.len());
        for i in 0..rows_to_print {
            let (word_index, entropy) = possible_picks[i];
            let word = str::from_utf8(&words[word_index]).unwrap();
            println!("\t{} {}: {:.2} bits", i + 1, word, entropy);
        }
        if possible_picks.len() < 2 {
            println!("No more possibilities");
            return;
        }
        let mut new_word_index = usize::max_value();
        while new_word_index == usize::max_value() {
            println!("What word did you pick? (0 to quit)");
            let mut input_string = String::new();
            stdin()
                .read_line(&mut input_string)
                .ok()
                .expect("Failed to read line");
            let trimmed_string = input_string.trim();

            let number_parse_result = trimmed_string.parse::<usize>();

            if let Ok(number) = number_parse_result {
                if number == 0 {
                    return;
                }
                if number <= possible_picks.len() {
                    new_word_index = possible_picks[number - 1].0;
                }
            } else {
                if trimmed_string.len() != WORD_LEN {
                    // FIXME: This should check the input better
                    println!("Wrong number of letters. Try again.");
                    continue;
                }
                let buf = trimmed_string.as_bytes();
                let mut maybe_word = [0 as u8; WORD_LEN];
                maybe_word.copy_from_slice(&buf);
                let search_result = words.iter().position(|&r| r == maybe_word);
                if search_result.is_none() {
                    println!("That's not a word. Try again.");
                    continue;
                }
                new_word_index = search_result.unwrap();
            }
        }
        let mut new_word = [0 as u8; WORD_LEN];
        new_word.copy_from_slice(&words[new_word_index]);

        let mut input_group: Group = (GROUP_COUNT - 1) as Group;
        while input_group == (GROUP_COUNT - 1) as Group {
            println!("What did wordle return? (0 = not present, 1 = wrong place, 2 = right place)");
            let mut input_string = String::new();
            stdin()
                .read_line(&mut input_string)
                .ok()
                .expect("Failed to read line");
            let trimmed_string = input_string.trim();
            if trimmed_string.len() != WORD_LEN {
                // FIXME: This should check the input better
                println!("That's not a complete response. Try again.");
                continue;
            }
            let buf = trimmed_string.as_bytes();
            input_group = 0;
            for i in 0..WORD_LEN {
                let value = buf[i as usize] - ('0' as u8);
                input_group = 3 * input_group + value;
            }
            if input_group == (GROUP_COUNT - 1) as Group {
                println!("Congratulations!");
                return;
            }
        }

        // Go throught all the possibilities and remove the ones that don't match
        let mut new_conforming = Vec::new();
        for word_index in conforming {
            if groups[(new_word_index, word_index)] == input_group {
                new_conforming.push(word_index);
            }
        }
        if new_conforming.len() == 0 {
            println!("No words match that response.");
            return;
        }

        conforming = new_conforming;
    }
}
