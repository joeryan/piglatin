use regex::Regex;
fn main() {
    // TODO if wanted get string from user as a cli
    let old_string = "This donkey went to town and came home";
    let mut output_words: Vec<String> = Vec::new();
    // for each word in string:
    for word in old_string.split(' ') {
        // check first char for vowel
        let pig_word = make_pig_word(word);
        // if vowel move to end and add ay, else add hay to end
        // add word to output string
        output_words.push(pig_word);
    }
    let output_string = output_words.join(" ");
    // print output string
    println!("{old_string}");
    println!("{output_string}")
}

fn make_pig_word(word: &str) -> String {
    let vowels = "aAeEiIoOuU";
    let mut pig_word = String::from(word);
    let first_char = pig_word.remove(0);
    if vowels.contains(first_char) {
        let mut tmp_word = String::from(word);
        tmp_word.push_str("-hay");
        pig_word = tmp_word;
    } else {
        let mut pig_suffix = String::from("-");
        pig_suffix.push(first_char);
        pig_suffix.push_str("ay");
        println!("suffix: {pig_suffix}");
        pig_word.push_str(&pig_suffix);
    }
    return pig_word;
}
