use std::env::args;
use termcolor::{StandardStream, ColorChoice, ColorSpec, Color, WriteColor};
use std::io::Write;

#[tokio::main]
async fn main() {
	let input_word = args().nth(1).expect("No word given!");
	let mut cout = StandardStream::stdout(ColorChoice::Always);
	let word = vocab::main(&input_word).await;
	// print definitions and examples
	for (i, definition_block) in word.definition_blocks.iter().enumerate() {
		cout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
		writeln!(cout, "{}. {}", i + 1, definition_block.definition).unwrap();
		cout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
		writeln!(cout, "Examples:").unwrap();
		for example in definition_block.examples.iter() {
			write!(cout, " {}", example).unwrap();
		}
		writeln!(cout).unwrap();
	}
	// print verbs
	cout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(220, 0, 0)))).unwrap();
	write!(cout, "verbs:").unwrap();
	for verb in word.verbs {
		write!(cout, " {}", verb).unwrap();
	}
	writeln!(cout).unwrap();

	// print nouns
	cout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();	
	write!(cout, "nouns:").unwrap();
	for noun in word.nouns {
		write!(cout, " {}", noun).unwrap();
	}
	writeln!(cout).unwrap();

	// print adjectives
	cout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(220, 0, 0)))).unwrap();
	write!(cout, "adjectives:").unwrap();
	for adjective in word.adjectives {
		write!(cout, " {}", adjective).unwrap();
	}
	writeln!(cout).unwrap();

	// print adverbs
	cout.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
	write!(cout, "adverbs:").unwrap();
	for adverb in word.adverbs {
		write!(cout, " {}", adverb).unwrap();
	}
	writeln!(cout).unwrap();
}