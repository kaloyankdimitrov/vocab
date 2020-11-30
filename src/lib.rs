mod fetch;
mod parsers;
use std::vec::Vec;

#[derive(Debug)]
pub struct Word {
	pub definition_blocks: Vec<parsers::DefinitionBlock>,
	pub verbs: Vec<String>,
	pub nouns: Vec<String>,
	pub adjectives: Vec<String>,
	pub adverbs: Vec<String>
}

impl Word {
	fn new() -> Word {
		Word {
			definition_blocks: Vec::new(),
			verbs: Vec::new(),
			nouns: Vec::new(),
			adjectives: Vec::new(),
			adverbs: Vec::new(),
		}
	}
} 

pub async fn main(input_word: &str) -> Word {
	let mut word = Word::new();
	let (t1, t2, t3, t4, t5) = fetch::main(input_word).await;	
	word.definition_blocks = parsers::cambridge(&t1);
	word.verbs = parsers::wordhippo(&t2);
	word.nouns = parsers::wordhippo(&t3);
	word.adjectives = parsers::wordhippo(&t4);
	word.adverbs = parsers::wordhippo(&t5);
	word
}
