use select::document::Document;
use select::predicate::Class;

pub fn cambridge(document: &str) {
	let document = Document::from(document);
	for (i, block) in document.select(Class("def-block")).enumerate() {
		let block_document = Document::from(&block.html()[..]);
		for definition in block_document.select(Class("def")) {
			println!("{}. {}", i + 1, definition.text());
			println!("Examples: ");
			for (i, example) in block_document.select(Class("examp")).enumerate() {
				let example_text = match i {
					0 => example.text(),
					_ => example.text()[1..].to_string(),
				};
				println!("	{}", example_text);
			}
		}
	}
}