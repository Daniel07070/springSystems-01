fn most_frequent_word(text: &str) -> (String, usize) {
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut diff_words: Vec<&str> = Vec::new(); //vector to store different words once
    let mut counts: Vec<usize> = Vec::new();

    for word in &words{
        let mut found = false;
        for (i, dword) in diff_words.iter().enumerate(){
            if dword == word{ ///checks if word already exists
                let count_ref: &mut usize = &mut counts[i];
                *count_ref += 1; // derefenrence and increment count_ref
                found = true; 
                break;
            }
        }
        if !found{ //if word was not found then add it by pushing
            diff_words.push(word);
            counts.push(1); // add 1 to that words count
        }
    }
    let mut max_count = 0;
    let mut max_word = "";
    for (i, &count) in counts.iter().enumerate(){
        if count > max_count{ 
            max_count = count; // changes max_count to new count if true
            max_word = diff_words[i]
        }
    }
    (max_word.to_string(), max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}