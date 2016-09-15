extern crate csv;
#[macro_use] extern crate nickel;
extern crate rustc_serialize;


use std::env;
use std::collections::HashMap;

use nickel::{Nickel, HttpRouter, StaticFilesHandler};


#[derive(Debug, Clone, RustcEncodable)]
struct Country {
    name: String,
    num_events: i32,
    num_fatalities: i32,
}
impl Country {
    fn new(n: String, num_eve: i32, num_fat: i32) -> Country {
        Country {
            name: n,
            num_events: num_eve,
            num_fatalities: num_fat,
        }
    }
}

#[derive(Debug, Clone, RustcEncodable)]
struct CountryPageData {
    found: bool,
    name: String,
    events: i32,
    fatalities: i32,
}
impl CountryPageData {
    fn new(n: String, eve: i32, fat: i32) -> CountryPageData {
        CountryPageData {
            found: true,
            name: n,
            events: eve,
            fatalities: fat,
        }
    }
}


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

    let mut fatality_count = 0;
    let mut country_vec: Vec<String> = Vec::new();
    let mut country_str: Vec<Country> = Vec::new();
    let mut ind = 0;
	for row in rdr.records() {
        let row = row.unwrap();

        let fatalities = row[24].parse::<i32>().unwrap();
        if fatalities > 0 {
            fatality_count += 1;
        }

        let country = row[14].parse::<String>().unwrap();
        if !country_vec.contains(&country) {
            country_vec.push(country.clone());
            ind = country_vec.len() - 1;
            let n_ctry = Country::new(country.clone(), 0, 0);
            country_str.push(n_ctry.clone());
        }
        
        country_str[ind].num_fatalities += fatalities;
        country_str[ind].num_events += 1;
        
        
	}
    println!("Incidents with more than one fatality: {}", fatality_count);
    println!("{:?}", country_str);

    let mut server = Nickel::new();

    server.utilize(StaticFilesHandler::new("images/"));
    server.utilize(StaticFilesHandler::new("src/dart"));
    server.utilize(StaticFilesHandler::new("styles/"));

    server.get("/", middleware! { |_, response| 
    	let mut data = HashMap::new();
        // let c_vec = country_vec.clone();
        let c_vec = country_str.clone();
        let mut countries: Vec<HashMap<String, String>> = Vec::new();
        for ctry in c_vec {
            let mut c_map: HashMap<String, String> = HashMap::new();
            
            //let fatalities = country_map[&ctry].clone().to_string();
            
            //c_map.insert("name".to_string(), ctry);
            //c_map.insert("fatalities".to_string(), fatalities);

            c_map.insert("name".to_string(), ctry.name.clone());
            c_map.insert("fatalities".to_string(), ctry.num_fatalities.clone().to_string());
            c_map.insert("events".to_string(), ctry.num_events.clone().to_string());
            countries.push(c_map);
        }
        data.insert("countries", countries);
    
        let mut title_vec: Vec<HashMap<String, String>> = Vec::new();
        let mut title_map: HashMap<String, String> = HashMap::new();
        title_map.insert("t_string".to_string(), "ACLED".to_string());
        title_vec.push(title_map);
        data.insert("title", title_vec);
        println!("{:?}", data);	
    	return response.render("views/main.tpl", &data);
    });

    server.get("/country/:country_name", middleware! { |request, response| 
        let mut data = HashMap::new();
        let c_name = request.param("country_name").unwrap();
        println!("{}", c_name);
        if !country_vec.contains(&c_name.to_string()) {
            data.insert("found", "");
            return response.render("views/country.tpl", &data);
        }
        else {
            let c_struct = CountryPageData { 
                found: false,
                name: c_name.to_string(),
                events: 0,
                fatalities: 0
            };
            return response.render("views/country.tpl", &c_struct);
        }
    });

    server.listen("127.0.0.1:6767");
}
