mod token {
    trait Tokenizer {
        type Error;

        fn tokenizer(program: String) -> Result<Vec<String>, Self::Error>;
    }
}
