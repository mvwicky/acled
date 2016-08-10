extern crate csv;

use std::env;

fn main() {
	let mut p = env::current_dir().unwrap();
	p.push("data");
	p.push("ACLED_Data.csv");
	println!("{}", p.display());
	println!("{}", p.exists());
	println!("{}", p.is_file());

	let p_p = p.as_path();
	println!("{}", p_p.display());

	let mut rdr = csv::Reader::from_file(p_p).unwrap();
	
	println!("HEADERS");
	println!("{:?}", rdr.headers().unwrap());
	println!("");

	for row in rdr.records() {
		let row = row.unwrap();
		println!("{:?}", row[1]);
	}
}
