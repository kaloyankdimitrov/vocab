use select::document::Document;
use select::predicate::Class;
use std::{vec::Vec, string::String};

#[derive(Debug)]
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
	for (i, block) in document.select(Class("def-block")).enumerate() {
		definitions.push(DefinitionBlock::new());
		let block_document = Document::from(&block.html()[..]);
		for definition in block_document.select(Class("def")) {
			definitions[i].definition = definition.text();
			for (ei, example) in block_document.select(Class("examp")).enumerate() {
				let example_text = match ei {
					0 => example.text(),
					_ => example.text()[1..].to_string(),
				};
				definitions[i].examples.push(example_text);
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