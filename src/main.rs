use array2d::Array2D;
use std::cmp::min;
use std::fs::File;
use std::io::*;
use std::str;
use std::usize;
//use enwordle::{WORD_LEN, GROUP_COUNT, Word, Group, groups_for_words, entropy_list};
use enwordle::*;

const WORD_COUNT: usize = 12947;


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
        groups[(i, i)] = (GROUP_COUNT - 1) as Group;
        for j in i+1..word_count {
            let (groupa, groupb) = groups_for_words(&words[i], &words[j]);
            groups[(i, j)] = groupa;
            groups[(j, i)] = groupb;
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
            println!("What did wordle return? (0=not present, 1=wrong place, 2=right place)");
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
