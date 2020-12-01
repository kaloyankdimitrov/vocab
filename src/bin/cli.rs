use std::env::args;
use termcolor::{StandardStream, ColorChoice, ColorSpec, Color, WriteColor};
use std::io::Write;

#[tokio::main]
async fn main() {
	let mut cout = StandardStream::stdout(ColorChoice::Always);
	let input_word = match args().nth(1) {
		Some(word) => word,
		None => {
			// print red error
			cout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(220, 0, 0)))).unwrap();
			writeln!(cout, "No word given!").unwrap();
			return
		}
	};
	let word = match vocab::main(&input_word).await {
		Ok(word) => word,
		Err(_) => {
			// print red error
			cout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(220, 0, 0)))).unwrap();
			writeln!(cout, "Could not fetch online dictionaries!").unwrap();
			return
		}
	};
	// print definitions and examples
	for (i, definition_block) in word.definition_blocks.iter().enumerate() {
		cout.set_color(ColorSpec::new().set_fg(Some(Color::Green))).unwrap();
		writeln!(cout, "{}. {}", i + 1, definition_block.definition).unwrap();
		cout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan))).unwrap();
		if definition_block.examples.len() > 0 {
			writeln!(cout, "Examples:").unwrap();
			for example in definition_block.examples.iter() {
				writeln!(cout, " {}", example).unwrap();
			}
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