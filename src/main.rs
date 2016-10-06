extern crate csv;
#[macro_use] extern crate nickel;
extern crate rustc_serialize;
extern crate acled;


use std::env;
use std::collections::HashMap;

use nickel::{Nickel, HttpRouter, Mountable, StaticFilesHandler};

use acled::data_structs::{Event, Country, CountryPageData, MainPageData};


fn contains_name(inp_vec: &Vec<Country>, inp_name: String) -> bool {
    if inp_vec.is_empty() || inp_name.is_empty() {
        false
    }
    else {
        for elem in inp_vec {
            if elem.name == inp_name {
                return true;
            }
        }
        false
    }
}

fn get_country_by_name(inp_vec: &Vec<Country>, inp_name: String) -> Option<Country> {
	if inp_vec.is_empty() || inp_name.is_empty() {
		None
	}
	else {
		for elem in inp_vec {
			if elem.name == inp_name {
                let n_elem = elem.clone();
				return Some(n_elem);
			}
		}
		None
	}
}

fn get_country_by_link(inp_vec: &Vec<Country>, inp_link: String) -> Option<Country> {
    if inp_vec.is_empty() || inp_link.is_empty() {
        None
    }
    else {
        for elem in inp_vec {
            if elem.link == inp_link {
                let n_elem = elem.clone();
                return Some(n_elem);
            }
        }
        None
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
    for head in rdr.headers().unwrap() {
        println!("{:?}", head);
    }
	println!("");

    
    let country_index: usize = 14;
    let fatality_index: usize = 24;

    let mut country_vec: Vec<Country> = Vec::new();
    let mut ind: usize = 0;
    let mut tot_eve: i32 = 0;
	for row in rdr.records() {
        tot_eve += 1;
        let row = row.unwrap();

        let fatalities = row[fatality_index].parse::<i32>().unwrap();
        let country = row[country_index].parse::<String>().unwrap();
        
        if country_vec.is_empty() {
            let link = country.replace(" ", "");
            let n_ctry = Country::new(link, country.clone(), 1, fatalities);
            country_vec.push(n_ctry.clone());
        }
        if !contains_name(&country_vec, country.clone()) {
            let link = country.replace(" ", "");
            let n_ctry = Country::new(link, country.clone(), 1, fatalities);
            country_vec.push(n_ctry.clone());
            ind += 1;
        }
        else {
            country_vec[ind].num_events += 1;
            country_vec[ind].num_fatalities += fatalities;
        }
	}
    for elem in country_vec.clone() {
        println!("{:?}", elem);
    }
    println!("Total Events: {}", tot_eve);

    let mut server = Nickel::new();

    server.mount("/static/styles/", StaticFilesHandler::new("styles/"));
    server.mount("/static/images/", StaticFilesHandler::new("images/"));
    server.mount("/dart/", StaticFilesHandler::new("src/dart"));
   
    let c_vec = country_vec.clone();
    // Main Page
    server.get("/", middleware! { |_, response| 
        let main_page = MainPageData { countries: c_vec.clone() }; 
    	return response.render("views/main.tpl", &main_page);
    });

    server.get("/country/:country_link", middleware! { |request, response| 
        let c_link = request.param("country_link").unwrap();
        let ctry = get_country_by_link(&country_vec, c_link.to_string());
        println!("{:?}", ctry);
        if ctry.is_none() {
            let c_struct = CountryPageData {
                found: false,
                name: "Not Found".to_string(),
                events: 0,
                fatalities:0
            };
            return response.render("views/country.tpl", &c_struct);
        }
        else {
            let ctry = ctry.unwrap();
            let c_struct = CountryPageData { 
                found: true,
                name: ctry.name,
                events: ctry.num_events,
                fatalities: ctry.num_fatalities, 
            };
            return response.render("views/country.tpl", &c_struct);
        }
    });

    server.listen("127.0.0.1:6767");
}
