use tiktoken_rs::r50k_base;

#[cfg(test)]
mod test {

    use crate::working_with_text_data::read_text_from_file;

    use super::*;

    #[test]
    fn test_tiktoken() {
        let tokenizer = r50k_base().unwrap();

        let content = read_text_from_file().unwrap();

        let enc_text = tokenizer.encode_ordinary(&content);
        println!("{:?}", enc_text.len());

        let enc_sample = &enc_text[50..];
        let context_size = 4;
        let x = &enc_sample[..context_size];
        let y = &enc_sample[1..context_size + 1];

        println!("x: {:?}, y: {:?}", x, y);
        for i in 1..=context_size {
            let context = &enc_sample[..i];
            let desired = enc_sample[i];

            println!("{:?} ----> {:?}", context, desired);
            let decoded_context = tokenizer.decode(context.to_vec()).unwrap();
            let decoded_desired = tokenizer.decode(vec![desired]).unwrap();
            println!("{} ----> {}", decoded_context, decoded_desired,);
        }
    }
}
