fn main() {
    // TODO if wanted get string from user as a cli
    let old_string = "This donkey went to town and came home.";
    let mut output_words: Vec<&str> = Vec::new();
    // for each word in string:
    for word in old_string.split(' ') {
        // check first char for vowel
        // if vowel move to end and add ay, else add hay to end
        // add word to output string
        output_words.push(word);
    }
    let output_string = output_words.join(" ");
    // print output string
    println!("{output_string}")
}
