use regex::Regex;
use reqwest::blocking::Client;
use std::char;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Duration;

pub fn write_text_to_file() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("the-verdict.txt");

    if !Path::new(path).exists() {
        let url = "https://raw.githubusercontent.com/rasbt/LLMs-from-scratch/main/ch02/01_main-chapter-code/the-verdict.txt";

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        let response = client.get(url).send()?;

        let response = response.error_for_status()?;

        let bytes = response.bytes()?;
        let mut file = File::create(path)?;
        file.write_all(&bytes)?;

        print!("File created successfully.");
    } else {
        println!("File already exists, skipping creation.");
    }
    Ok(())
}

pub fn read_text_from_file() -> Result<String, Box<dyn std::error::Error>> {
    let path = Path::new("the-verdict.txt");
    let contents = std::fs::read_to_string(path)?;
    let char_count = contents.chars().count();
    println!("The file contains {} characters.", char_count);

    let snippet: String = contents.chars().take(99).collect();
    println!("First 99 characters:\n{}", snippet);

    Ok(contents)
}

// The goal is to tokenize and  embed this text for an LLM
// Let's develop simple a tokenizer based on some simple sample text that we can then later apply to the text above.
// The following regular expression will split on whitespace and punctuation, but keep contractions together (e.g., "don't" will be treated as one token):

pub fn text_tokenizer(text: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let re = Regex::new(r#"([,.:;?_!"()\']|--|\s)"#)?;
    let mut result = Vec::new();

    let mut last_end = 0;

    for mat in re.find_iter(text) {
        result.push(text[last_end..mat.start()].to_string());

        result.push(mat.as_str().to_string());
        last_end = mat.end();
    }

    result.push(text[last_end..].to_string());
    let result = result
        .into_iter()
        .filter(|item| !item.trim().is_empty())
        .collect();

    Ok(result)
}

pub fn token_to_token_ids(tokens: Vec<String>) -> BTreeMap<String, usize> {
    let mut all_words = tokens.clone();
    all_words.sort_unstable();
    all_words.dedup();

    println!("Vocabulary size: {}", all_words.len());

    let mut vocab: BTreeMap<String, usize> = all_words
        .into_iter()
        .enumerate()
        .map(|(i, token)| (token, i))
        .collect();

    for (i, (token, integer)) in vocab.iter().enumerate() {
        println!("Token: {}, ID: {}", token, integer);

        if i == 50 {
            break;
        }
    }

    vocab
}

mod tests {
    use super::*;

    #[test]
    fn test_write_text_to_file() {
        let result = write_text_to_file();
        assert!(result.is_ok());
    }

    #[test]
    fn test_read_text_from_file() {
        let result = read_text_from_file();
        assert!(result.is_ok());
    }

    #[test]
    fn test_text_tokenizer() {
        let text = "Hello, world. Is this-- a test?";
        let tokens = text_tokenizer(text).unwrap();
        dbg!("Tokens: {:?}", tokens);
    }

    #[test]
    fn test_file_tokeniner() {
        let contents = read_text_from_file().unwrap();
        let tokens = text_tokenizer(&contents).unwrap();
        let preprocessed: Vec<String> = tokens
            .into_iter()
            .map(|token| token.trim().to_string())
            .filter(|token| !token.is_empty())
            .collect();

        let display_count = preprocessed.len();
        println!("Total tokens: {}", display_count);
        println!("{:?}", &preprocessed[..30]);
    }

    #[test]
    fn test_token_to_token_ids() {
        let contents = read_text_from_file().unwrap();
        let tokens = text_tokenizer(&contents).unwrap();
        let preprocessed: Vec<String> = tokens
            .into_iter()
            .map(|token| token.trim().to_string())
            .filter(|token| !token.is_empty())
            .collect();

        let vocab = token_to_token_ids(preprocessed);
        assert!(!vocab.is_empty());

    }
}
