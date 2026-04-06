use regex::Regex;
use std::collections::BTreeMap;

pub struct SimpleTokenizerV1 {
    str_to_int: BTreeMap<String, usize>,
    int_to_str: BTreeMap<usize, String>,
    split_re: Regex,
    replace_re: Regex,
}
impl SimpleTokenizerV1 {
    pub fn new(vocab: BTreeMap<String, usize>) -> Self {
        let mut int_to_str = BTreeMap::new();
        for (s, &i) in &vocab {
            int_to_str.insert(i, s.clone());
        }

        let split_re = Regex::new(r#"[,.:;?_!"()\']|--|\s"#).unwrap();
        let replace_re = Regex::new(r#"\s+([,.:;?!"()'])"#).unwrap();
        Self {
            str_to_int: vocab,
            int_to_str,
            split_re,
            replace_re,
        }
    }

    pub fn encode(&self, text: &str) -> Vec<usize> {
        let mut split_result: Vec<&str> = Vec::new();
        let mut last_end = 0;

        for mat in self.split_re.find_iter(text) {
            split_result.push(&text[last_end..mat.start()]);
            split_result.push(mat.as_str());
            last_end = mat.end();
        }

        split_result.push(&text[last_end..]);

        let preprocessed: Vec<&str> = split_result
            .into_iter()
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
            .collect();

        let ids: Vec<usize> = preprocessed
            .into_iter()
            .map(|s| {
                *self
                    .str_to_int
                    .get(s)
                    .expect("Token not found in vocabulary")
            })
            .collect();

        ids
    }

    pub fn encode_v2(&self, text: &str) -> Vec<usize> {
        let mut split_result: Vec<&str> = Vec::new();
        let mut last_end = 0;

        for mat in self.split_re.find_iter(text) {
            split_result.push(&text[last_end..mat.start()]);
            split_result.push(mat.as_str());
            last_end = mat.end();
        }

        split_result.push(&text[last_end..]);

        let ids: Vec<usize> = split_result
            .into_iter()
            .map(|item| item.trim())
            .filter(|item| !item.is_empty())
            .map(|item| {
                let token = if self.str_to_int.contains_key(item) {
                    item
                } else {
                    "<|unk|>"
                };
                *self
                    .str_to_int
                    .get(token)
                    .expect("Critical error: Vocabulary must contain '<|unk|>' token")
            })
            .collect();

        ids
    }

    pub fn decode(&self, ids: &[usize]) -> String {
        let words: Vec<&str> = ids
            .iter()
            .map(|i| {
                self.int_to_str
                    .get(i)
                    .expect("Id not found in vocabulary")
                    .as_str()
            })
            .collect();

        let joined_text = words.join(" ");

        let decoded_text = self.replace_re.replace_all(&joined_text, "$1");
        decoded_text.into_owned()
    }
}

#[cfg(test)]
mod test {
    use crate::{
        simple_tokenizerv1::{self, SimpleTokenizerV1},
        working_with_text_data::{read_text_from_file, text_tokenizer, token_to_token_ids},
    };

    #[test]
    fn test_simple_tokenizer_v1() {
        let contents = read_text_from_file().unwrap();
        let mut tokens = text_tokenizer(&contents).unwrap();
        tokens.push("<|endoftext|>".to_owned());
        tokens.push("<|unk|>".to_owned());
        let preprocessed: Vec<String> = tokens
            .into_iter()
            .map(|token| token.trim().to_string())
            .filter(|token| !token.is_empty())
            .collect();

        let vocab = token_to_token_ids(preprocessed);

        let tokenizer = SimpleTokenizerV1::new(vocab);

        let text =
            r#""It's the last he painted, you know," Mrs. Gisburn said with pardonable pride."#;

        let ids = tokenizer.encode(text);
        println!("{:?}", ids);
        let decoded_text = tokenizer.decode(&ids);
        println!("{}", decoded_text);

        println!("================");

        let text1 = "Hello, do you like tea?";
        let text2 = "In the sunlit terraces of the palace.";

        let text = format!("{} <|endoftext|> {}", text1, text2);
        println!("{}", text);

        let ids = tokenizer.encode_v2(&text);
        println!("{:?}", ids);
        let decoded = tokenizer.decode(&ids);
        println!("{}", decoded);
    }
}
