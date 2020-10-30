mod parsers;
use reqwest::get;
use std::env::args;
use tokio::join;

async fn fetch(url: &str) -> Result<String, reqwest::Error> {
	Ok(get(url).await?.text().await?)
}

async fn fetch_all(word: &str) -> (String, String, String, String, String) {
	let urls: [&str; 5] = [
		&format!("https://dictionary.cambridge.org/dictionary/english/{}", word)[..],
		&format!("https://www.wordhippo.com/what-is/the-noun-for/{}.html", word)[..],
		&format!("https://www.wordhippo.com/what-is/the-verb-for/{}.html", word)[..],
		&format!("https://www.wordhippo.com/what-is/the-adjective-for/{}.html", word)[..],
		&format!("https://www.wordhippo.com/what-is/the-adverb-for/{}.html", word)[..],
	];
	let (r1, r2, r3, r4, r5) = join!(
		fetch(urls[0]),
		fetch(urls[1]),
		fetch(urls[2]),
		fetch(urls[3]),
		fetch(urls[4]),
	);
	(r1.unwrap(), r2.unwrap(), r3.unwrap(), r4.unwrap(), r5.unwrap()) 
}

#[tokio::main]
async fn main() {
	let word = args().nth(1).expect("No word given!");
	let (t1, _t2, _t3, _t4, _t5) = fetch_all(&word).await;	
	parsers::cambridge(&t1);
}
