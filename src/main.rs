extern crate csv;
#[macro_use] extern crate nickel;

use std::env;
use std::collections::HashMap;

use nickel::{Nickel, HttpRouter, StaticFilesHandler};

fn main() {
	let mut p = env::current_dir().unwrap();
	p.push("data");
	p.push("csv");
	p.push("ACLED_Data_clean.csv");
	println!("{}", p.display());
	println!("{}", p.is_file());
    if !p.is_file() {
        println!("File does not exist! {}", p.display());
        std::process::exit(-1);
    }

	let mut rdr = csv::Reader::from_file(p.as_path()).unwrap();
	
	println!("HEADERS");
	println!("{:?}", rdr.headers().unwrap());
	println!("");

    let mut cnt = 0;
	for row in rdr.records() {
		let row = row.unwrap();
        let fat = row[24].parse::<i32>().unwrap();
        if fat >= 1 {
              cnt += 1;
        }
        break;
	}
    println!("Incidents with more than one fatality: {}", cnt);

    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("images/"));
    server.utilize(StaticFilesHandler::new("src/dart"));
    server.utilize(StaticFilesHandler::new("styles/"));

    server.get("/", middleware! { |_, response| 
    	let mut data = HashMap::new();

    	data.insert("name", "User");
    	
    	return response.render("views/main.tpl", &data);
    });

    server.listen("127.0.0.1:6767");
}
