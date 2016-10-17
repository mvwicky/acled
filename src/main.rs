extern crate chrono;
extern crate csv;
#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate acled;
extern crate postgres;


use std::env;
use std::collections::HashMap;

use nickel::{Nickel, HttpRouter, Mountable, StaticFilesHandler};

use acled::data_structs::{Event, EventTrunc, Country, CountryPageData, MainPageData,
                          CountryNameLink};

use postgres::{Connection, TlsMode};
use postgres::types::ToSql;


fn contains_name(inp_vec: &Vec<Country>, inp_name: String) -> bool {
    if inp_vec.is_empty() || inp_name.is_empty() {
        false
    } else {
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
    } else {
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
    } else {
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
    let conn = Connection::connect("postgresql://michael@localhost/acled", TlsMode::None).unwrap();


    conn.execute("DROP TABLE IF EXISTS event", &[]).unwrap();
    // conn.execute("DROP TABLE IF EXISTS event_trunc", &[]).unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS event_trunc (\
                    id           SERIAL PRIMARY KEY,\
                    event_date   DATE,\
                    year         INT,\
                    event_type   VARCHAR NOT NULL,\
                    actor_1      VARCHAR NOT NULL,\
                    ally_actor_1 VARCHAR NOT NULL,\
                    actor_2      VARCHAR NOT NULL,\
                    ally_actor_2 VARCHAR NOT NULL,\
                    country      VARCHAR NOT NULL,\
                    admin_1      VARCHAR NOT NULL,\
                    admin_2      VARCHAR NOT NULL,\
                    admin_3      VARCHAR NOT NULL,\
                    location     VARCHAR NOT NULL,\
                    latitude     DOUBLE PRECISION,\
                    longitude    DOUBLE PRECISION,\
                    source       VARCHAR NOT NULL,\
                    notes        VARCHAR NOT NULL,\
                    fatalities   INT\
                  )",
                 &[])
        .unwrap();

    let mut cur_rows: i64 = 0;
    for cnt in &conn.query("SELECT count(*) from event_trunc", &[]).unwrap() {
        cur_rows = cnt.get(0);
    }

    let mut rdr = csv::Reader::from_file(p.as_path()).unwrap();

    println!("");


    let country_index: usize = 14;
    let fatality_index: usize = 24;

    let mut country_nl_vec: Vec<CountryNameLink> = Vec::new();
    let mut country_vec: Vec<Country> = Vec::new();
    let mut tot_eve: i32 = 0;
    let mut last_country: Option<String> = None;

    let add_event_trunc =
        conn.prepare("INSERT INTO event_trunc (event_date, year, event_type, \
                      actor_1,ally_actor_1, actor_2, ally_actor_2, country,admin_1, admin_2, \
                      admin_3, location,latitude, longitude, source, notes,fatalities) VALUES \
                      ($1, $2, $3, $4,$5, $6, $7, $8,$9, $10, $11, $12,$13, $14, $15, $16,$17)")
            .unwrap();

    let get_event_trunc_country = conn.prepare("SELECT * FROM event_trunc WHERE country = $1")
        .unwrap();

    let get_event_trunc_country_year =
        conn.prepare("SELECT * FROM event_trunc WHERE country = $1 AND year = $2").unwrap();

    let mut i = 0;
    for ret_row in &get_event_trunc_country.query(&[&"Somalia"]).unwrap() {
        i += 1;
    }
    println!("{}", i);

    let mut j = 0;
    for ret_row in &get_event_trunc_country_year.query(&[&"Somalia", &2015]).unwrap() {
        j += 1;
    }
    println!("{}", j);
    std::process::exit(0);

    let mut rows_aff = 0;
    for row in rdr.records() {
        tot_eve += 1;
        let row = row.unwrap();

        let e_t = EventTrunc::from_row(&row);
        if cur_rows == 0 {
            let r = add_event_trunc.execute(&[&e_t.event_date,
                           &e_t.year,
                           &e_t.event_type,
                           &e_t.actor_1,
                           &e_t.ally_actor_1,
                           &e_t.actor_2,
                           &e_t.ally_actor_2,
                           &e_t.country,
                           &e_t.admin_1,
                           &e_t.admin_2,
                           &e_t.admin_3,
                           &e_t.location,
                           &e_t.latitude,
                           &e_t.longitude,
                           &e_t.source,
                           &e_t.notes,
                           &e_t.fatalities])
                .unwrap();
            rows_aff += r;
            println!("{}", rows_aff);
        }

        let fatalities = row[fatality_index].parse::<i32>().unwrap();
        let country = row[country_index].parse::<String>().unwrap();

        if country_vec.is_empty() || last_country.is_none() {
            country_nl_vec.push(CountryNameLink::new(country.clone()));
        }
        if last_country.is_some() && last_country.unwrap() != country {
            country_nl_vec.push(CountryNameLink::new(country.clone()));
        }

        last_country = Some(country.clone());

        if country_vec.is_empty() || !contains_name(&country_vec, country.clone()) {
            let link = country.replace(" ", "");
            let n_ctry = Country::new(link, country.clone(), 0, 0);
            country_vec.push(n_ctry.clone());
        }
        let ind = country_vec.len() - 1;
        country_vec[ind].num_events += 1;
        country_vec[ind].num_fatalities += fatalities;
        country_vec[ind].events.push(Event::from_row(&row));
    }
    println!("Rows Affected: {}", rows_aff);
    println!("Total Events: {}", tot_eve);

    let mut server = Nickel::new();

    server.mount("/static/styles/", StaticFilesHandler::new("styles/"));
    server.mount("/static/images/", StaticFilesHandler::new("images/"));
    server.mount("/dart/", StaticFilesHandler::new("src/dart"));
    server.mount("/js/", StaticFilesHandler::new("src/js"));

    let c_vec = country_vec.clone();
    // Main Page
    let main_page = MainPageData { countries: country_nl_vec };
    server.get("/",
               middleware! { |request, response| 
        println!("{}", request.path_without_query().unwrap());
    	return response.render("views/main.tpl", &main_page);
    });
    // Page for a country
    server.get("/country/:country_link",
               middleware! { |request, response| 
        println!("{}", request.path_without_query().unwrap());
        let c_link = request.param("country_link").unwrap();
        let ctry = get_country_by_link(&country_vec, c_link.to_string());
        if ctry.is_none() {
            let c_struct = CountryPageData {
                found: false,
                name: "Not Found".to_string(),
                years: Vec::new(),
            };
            return response.render("views/country.tpl", &c_struct);
        }
        else {
            let ctry = ctry.unwrap();
            let pg_data = ctry.to_page_data();
            return response.render("views/country.tpl", &pg_data);
        }
    });
    // return JSON data for a country (for xjax)
    // server.get("/country/data/:country_link", middleware! {|request, response|
    //
    // });
    //

    // Page for a year for a country
    // server.get("/country/:country_link/:year", middleware!{|request, response|
    //
    // });
    //

    server.listen("127.0.0.1:6767");
}
