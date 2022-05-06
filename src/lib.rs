
pub use array2d::Array2D;
pub use std::usize;

pub const WORD_LEN: usize = 5;
pub const GROUP_COUNT: usize = 243; // 3^5

pub type Word = [u8; WORD_LEN];
pub type WordIndex = usize;
pub type Group = u8;

pub fn string_for_group(group: &Group) -> String {
    let mut c = *group;
    let mut num_list = Vec::new();
    for _i in 0..WORD_LEN {
        let v = c % 3;
        println!("{}", &v);
        num_list.insert(0, v);
        c = c / 3;
    }
    println!("{}:{:?}", &group, &num_list);
    let mut s = String::new();
    for num in num_list {
        s.push_str(&format!("{}", &num));
    }
    s
}

// For building table. 
// First represents the output of guessing a when the right answer is b.
// Second represents the output of guessing b when the right answer is a.
pub fn groups_for_words(a: &Word, b: &Word) -> (Group, Group) {
    let mut checkword = [0; WORD_LEN];
    let mut claimword = [0; WORD_LEN];

    // Which letters are in the right place?
    for i in 0..WORD_LEN {
        if b[i] == a[i] {
            checkword[i] = 2;
            claimword[i] = 2;
        }
    }

    // Check for letters that are in the wrong place
    for i in 0..WORD_LEN {
        if checkword[i] == 0 {
            let looking_for = a[i];
            for j in 0..WORD_LEN {
                // Found and not already claimed?
                if looking_for == b[j] && claimword[j] == 0 {
                    checkword[i] = 1;
                    claimword[j] = 1;
                }
            }
        }
    }

    // Convert to numbers
    let mut groupa = 0;
    let mut groupb = 0;
    for i in 0..WORD_LEN {
        groupa = 3 * groupa + checkword[i];
        groupb = 3 * groupb + claimword[i];
    }

    // Return the numbers
    (groupa, groupb)
}

// Given the table of groups, the indexes of the possible answers, return the index of the answers
// that will give you the most entropy
pub fn entropy_list(groups: &Array2D<u8>, conforming: &Vec<WordIndex>) -> Vec<(WordIndex, f32)> {
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



#[cfg(test)]
mod tests {
    use super::*;
    // For testing
    fn groupstr_for_strings(a: &str, b: &str) -> (String, String) {
        let bufa = a.as_bytes();
        let mut worda = [0 as u8; WORD_LEN];
        worda.copy_from_slice(&bufa);

        let bufb = b.as_bytes();
        let mut wordb = [0 as u8; WORD_LEN];
        wordb.copy_from_slice(&bufb);

        let (groupa, groupb) = groups_for_words(&worda, &wordb);
        let stra = string_for_group(&groupa);
        let strb = string_for_group(&groupb);
        (stra, strb)
    }
    #[test]
    fn grouping() {
        let (stra, strb) = groupstr_for_strings(&"stirs", &"wrist");
        assert!(stra == "11210");
        assert!(strb == "01211");
    }
}