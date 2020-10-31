use select::document::Document;
use select::predicate::Class;
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

pub fn cambridge(document: &str, cout: &mut StandardStream) {
	cout.reset().unwrap();
	let document = Document::from(document);
	for (i, block) in document.select(Class("def-block")).enumerate() {
		let block_document = Document::from(&block.html()[..]);
		for definition in block_document.select(Class("def")) {
			cout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
			writeln!(cout, "{}. {}", i + 1, definition.text()).unwrap();
			cout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))).unwrap();
			writeln!(cout, "Examples: ").unwrap();
			for (i, example) in block_document.select(Class("examp")).enumerate() {
				let example_text = match i {
					0 => example.text(),
					_ => example.text()[1..].to_string(),
				};
				writeln!(cout, "	{}", example_text).unwrap();
			}
		}
	}
}

pub fn wordhippo(document: &str, cout: &mut StandardStream, form_type: &str) {
	cout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
	let document = Document::from(document);
	write!(cout, "{}: ", match form_type {
		"(n.)" => "noun(s)",
		"(v.)" => "verb(s)",
		"(adj.)" => "adjective(s)",
		"(adv.)" => "adverb(s)",
		_ => panic!("Incorrect word form!")
	}).unwrap();
	for word in document.select(Class("defv2wordtype")) {
		write!(cout, "{}{} ", word.text(), form_type).unwrap();
	}
	writeln!(cout, "").unwrap();
}