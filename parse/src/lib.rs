use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

/// Parses input from the file into a vector of strings
/// splits on newline characters
pub fn line(filename: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();

    // preparing file to be read
    let path = Path::new(filename);
    let file = File::open(path).map_err(|err| format!("{filename}: {err}"))?;
    let mut reader = BufReader::new(file);

    // reading each line of file and adding to vec
    let mut line = String::new();
    loop {
        let length = reader.read_line(&mut line)?;
        if length == 0 {
            return Ok(result);
        }
        // removes new line characters from end of string
        let l = line.trim_end_matches("\r\n").trim_end_matches('\n');
        result.push(l.to_string());
        line = String::new();
    }
}

/// Parses input from the file into a vector of strings
/// splits on sentence-end punctuation, i.e. ".", "?", "!"
pub fn sentence(filename: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();

    // calls the line function to get a vector of strings split on new lines
    let lines = line(filename)?;

    // loops over the lines to split on punctuation and add sentences to the vec
    for line in lines {
        let chars = line.chars();
        let mut sentence = String::new();
        for char in chars {
            if char == '.' || char == '?' || char == '!' {
                if !sentence.is_empty() {
                    sentence.push(char);
                    let mod_sentence = sentence.trim();
                    sentence = mod_sentence.to_string();
                    result.push(sentence);
                    sentence = String::new();
                }
            } else {
                sentence.push(char);
            }
        }
        if !sentence.is_empty() {
            result.push(sentence);
        }
    }
    Ok(result)
}

/// Parses input from the file into a vector of strings
/// splits on spaces
pub fn word(filename: &str) -> Result<Vec<String>> {
    let mut result = Vec::new();

    // calls the line function to get a vector of strings split on new lines
    let lines = line(filename)?;

    // loops over the lines to split on spaces and add words
    for line in lines {
        let chars = line.chars();
        let mut word = String::new();
        for char in chars {
            if char == ' ' {
                if !word.is_empty() {
                    result.push(word);
                }
                word = String::new();
            } else if char != '.' && char != '?' && char != '!' {
                word.push(char);
            }
        }
        if !word.is_empty() {
            result.push(word);
        }
    }
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line() {
        let result = line("SampleText.txt").unwrap();
        assert_eq!(result, ["line 1", "line? 2.", "line 3!", "lin.e 4?", "line !5"]);
    }

    #[test]
    fn test_sentence() {
        let result = sentence("SampleText.txt").unwrap();
        assert_eq!(result, ["line 1", "line?", "2.", "line 3!", "lin.", "e 4?", "line !", "5"]);
    }

    #[test]
    fn test_word() {
        let result = word("SampleText.txt").unwrap();
        assert_eq!(result, ["line", "1", "line", "2", "line", "3", "line", "4", "line", "5"]);
    }
}
