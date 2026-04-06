# ch02 — Working with Text Data

Rust implementation of Chapter 2 from [LLMs-from-scratch](https://github.com/rasbt/LLMs-from-scratch/blob/main/ch02).

## Modules

| Module                   | Description                                                                                                            |
| ------------------------ | ---------------------------------------------------------------------------------------------------------------------- |
| `working_with_text_data` | Download, read, and regex-tokenize `the-verdict.txt`; build a vocabulary mapping                                       |
| `simple_tokenizerv1`     | `SimpleTokenizerV1` — encode/decode text with a custom vocabulary, including `<\|unk\|>` and `<\|endoftext\|>` support |
| `bytepair_encoding`      | BPE tokenization via `tiktoken-rs` (`r50k_base`), sliding-window context/target pairs                                  |

## Topics Covered

- Downloading and reading text files (`the-verdict.txt`)
- Tokenizing text using regex-based splitting
- Building token-to-ID vocabularies (`BTreeMap`)
- Encoding and decoding with a custom tokenizer (`SimpleTokenizerV1`)
- Handling unknown tokens (`<|unk|>`) and special tokens (`<|endoftext|>`)
- Byte-Pair Encoding (BPE) with `tiktoken-rs`
- Creating input/target pairs with a sliding context window

## Usage

```bash
# Run all ch02 tests
cargo test -p ch02

# Run specific module tests
cargo test -p ch02 -- working_with_text_data
cargo test -p ch02 -- simple_tokenizerv1
cargo test -p ch02 -- bytepair_encoding

# Run a single test with output
cargo test -p ch02 -- bytepair_encoding::test::test_tiktoken --nocapture
```

## Dependencies

| Crate         | Purpose                                   |
| ------------- | ----------------------------------------- |
| `reqwest`     | HTTP client for downloading text files    |
| `regex`       | Regex-based text tokenization             |
| `tiktoken-rs` | OpenAI BPE tokenizer bindings (r50k_base) |

## Reference

- [Chapter 2 (Python)](https://github.com/rasbt/LLMs-from-scratch/blob/main/ch02)
