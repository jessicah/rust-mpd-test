extern crate mpd;

use mpd::Client;
use mpd::search::{Query, Term, Window};

fn main() {
    let mut connection = Client::connect("172.18.176.1:6601").unwrap();

	let mut query = Query::new();
	let query = query.and(Term::Any, "");
	let mut index = 0;
	let mut files: Vec<String> = vec![];
	loop {
		let search = connection.search(query, Window::from((index, index+1)));
		println!("{:?}", search);

		match search {
			Ok(_) => (),
			Err(e) => {
				println!("{:?}", e);
				println!("Read a total of {} items", index);
				break
			}
		}

		index += 1;
	}
}
