use select::document::Document;
use select::predicate::Class;
use std::{vec::Vec, string::String};

pub struct DefinitionBlock {
	pub definition: String,
	pub examples: Vec<String>,
}

impl DefinitionBlock {
	fn new() -> DefinitionBlock {
		DefinitionBlock {
			definition: String::new(),
			examples: Vec::new(),
		}
	}
}

pub fn cambridge(document: &str) -> Vec<DefinitionBlock> {
	let document = Document::from(document);
	let mut definitions: Vec<DefinitionBlock> = Vec::new();
	for block in document.select(Class("def-block")) {
		let block_document = Document::from(&block.html()[..]);
		if let Some(definition) = block_document.select(Class("def")).next() {
			definitions.push(DefinitionBlock::new());
			definitions.last_mut().unwrap().definition = definition.text();
			for (ei, example) in block_document.select(Class("examp")).enumerate() {
				let example_text = match ei {
					0 => example.text(),
					_ => example.text()[1..].to_string(),
				};
				definitions.last_mut().unwrap().examples.push(example_text);
			}
		} 
	}
	definitions
}

pub fn wordhippo(document: &str) -> Vec<String> {
	let mut words: Vec<String> = Vec::new(); 
	let document = Document::from(document);
	for word in document.select(Class("defv2wordtype")) {
		words.push(word.text());
	}
	words
}