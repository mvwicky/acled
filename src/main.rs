extern crate chrono;
extern crate csv;
#[macro_use]
extern crate nickel;
extern crate rustc_serialize;
extern crate acled;
extern crate postgres;


use std::env;
use std::process::Command;
// use std::path::{Path, PathBuf};

use nickel::{Nickel, HttpRouter, Mountable, StaticFilesHandler, QueryString};

use acled::{AllCountries, Event, EventTrunc, Country, CountryPageData, MainPageData,
            CountryNameLink, YearPageData};

use postgres::{Connection, TlsMode};
use postgres::params::{UserInfo, ConnectParams, ConnectTarget};
// use postgres::types::ToSql;

use acled::AcledServer;

fn main() {
    // Some server testing
    let mut test_server = AcledServer::new();
    test_server.init();
    let test_db_name = "acled_scratch";

    let test_params = ConnectParams {
        target: ConnectTarget::Tcp("localhost".to_owned()),
        port: None,
        user: Some(UserInfo {
            user: "michael".to_owned(),
            password: None,
        }),
        database: Some(test_db_name.to_owned()),
        options: vec![],
    };
    let test_conn = Connection::connect(test_params, TlsMode::None).unwrap();
    test_server.add_conn(test_conn);


    std::process::exit(0);

    // Compile SASS (SCSS) to CSS
    println!("Compiling SASS");
    let (styles_dir, main_scss, main_css) = ("styles", "main.scss", "main.css");
    let sass_cmd = format!("{styles}/{scss}:{styles}/{css}",
                           styles = styles_dir,
                           scss = main_scss,
                           css = main_css);
    println!("{cmd}", cmd = sass_cmd);
    Command::new("sass").arg(&sass_cmd).status().unwrap();

    // Make sure data file exists
    let mut p = env::current_dir().unwrap();
    p.push("data");
    p.push("csv");
    p.push("ACLED_Data_clean.csv");
    if !p.is_file() {
        println!("File does not exist ({})", p.display());
        std::process::exit(-1);
    }
    else {
        println!("File found ({})", p.display());
    }

    let rebuild_db = false;

    // Connect to acled db
    let host = "localhost";
    let uname = "michael";
    let db_name = "acled";

    println!("Opening database connection");
    let params = ConnectParams {
        target: ConnectTarget::Tcp(host.to_owned()),
        port: None,
        user: Some(UserInfo {
            user: uname.to_owned(),
            password: None,
        }),
        database: Some(db_name.to_owned()),
        options: vec![],
    };
    let conn = Connection::connect(params, TlsMode::None).unwrap();
    println!("Connection opened");

    if rebuild_db {
        conn.execute("DROP TABLE IF EXISTS event", &[]).unwrap();
        conn.execute("DROP TABLE IF EXISTS event_trunc", &[]).unwrap();
    }

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

    let add_event_trunc =
        conn.prepare("INSERT INTO event_trunc (event_date, year, event_type, \
                      actor_1,ally_actor_1, actor_2, ally_actor_2, country,admin_1, \
                      admin_2, admin_3, location,latitude, longitude, source, \
                      notes,fatalities) VALUES ($1, $2, $3, $4,$5, $6, $7, $8,$9, $10, \
                      $11, $12,$13, $14, $15, $16,$17)")
            .unwrap();

    let get_event_trunc_country =
        conn.prepare("SELECT * FROM event_trunc WHERE country = $1")
            .unwrap();

    let get_event_trunc_country_year =
        conn.prepare("SELECT * FROM event_trunc WHERE country = $1 AND year = $2")
            .unwrap();


    conn.execute("CREATE TABLE IF NOT EXISTS ctry_pair (\
        id   SERIAL PRIMARY KEY,\
        name VARCHAR NOT NULL,\
        link VARCHAR NOT NULL\
        )",
                 &[])
        .unwrap();

    let add_ctry_pair = conn.prepare("INSERT INTO ctry_pair (name, link) VALUES ($1, $2)")
        .unwrap();
    let get_all_ctry_pair = conn.prepare("SELECT * FROM  ctry_pair ORDER BY name")
        .unwrap();

    let mut cur_rows: i64 = 0;
    for cnt in &conn.query("SELECT count(*) from event_trunc", &[]).unwrap() {
        cur_rows = cnt.get(0);
    }

    let mut rdr = csv::Reader::from_file(p.as_path()).unwrap();

    println!("");

    let country_index: usize = 14;
    let fatality_index: usize = 24;

    let mut all_ctries = AllCountries::new();
    let mut tot_eve: i32 = 0;
    let mut rows_aff = 0;
    for row in rdr.records() {
        tot_eve += 1;
        let row = row.unwrap();

        let cur_eve = EventTrunc::from_csv_row(&row);
        if cur_rows == 0 {
            let r = add_event_trunc.execute(&[&cur_eve.event_date,
                           &cur_eve.year,
                           &cur_eve.event_type,
                           &cur_eve.actor_1,
                           &cur_eve.ally_actor_1,
                           &cur_eve.actor_2,
                           &cur_eve.ally_actor_2,
                           &cur_eve.country,
                           &cur_eve.admin_1,
                           &cur_eve.admin_2,
                           &cur_eve.admin_3,
                           &cur_eve.location,
                           &cur_eve.latitude,
                           &cur_eve.longitude,
                           &cur_eve.source,
                           &cur_eve.notes,
                           &cur_eve.fatalities])
                .unwrap();
            rows_aff += r;
            println!("{}", rows_aff);
        }

        let country = row[country_index].parse::<String>().unwrap();
        let fatalities = row[fatality_index].parse::<i32>().unwrap();

        all_ctries.push_new_if_not(country);

        let ac_ind = all_ctries.countries.len() - 1;
        all_ctries.countries[ac_ind].num_events += 1;
        all_ctries.countries[ac_ind].num_fatalities += fatalities;
        all_ctries.countries[ac_ind].events.push(Event::from_row(&row));
    }
    println!("Rows Affected: {}", rows_aff);
    println!("Total Events: {}", tot_eve);

    let mut nl_vec: Vec<CountryNameLink> = Vec::new();
    for elem in &get_all_ctry_pair.query(&[]).unwrap() {
        let cnv = CountryNameLink {
            name: elem.get(1),
            link: elem.get(2),
        };
        nl_vec.push(cnv);
    }

    let mut server = Nickel::new();

    server.mount("/static/styles/", StaticFilesHandler::new("styles/"));
    server.mount("/static/images/", StaticFilesHandler::new("images/"));
    server.mount("/dart/", StaticFilesHandler::new("src/dart"));
    server.mount("/js/", StaticFilesHandler::new("src/js"));

    // Main Page
    let main_page = MainPageData { countries: nl_vec };
    server.get("/",
               middleware! { |request, response| 
        println!("{}", request.path_without_query().unwrap());
    	return response.render("views/main.tpl", &main_page);
    });
    // Page for a country
    let t_all_ctries = all_ctries.clone();
    server.get("/country/:country_link",
               middleware! { |request, response| 
        println!("{}", request.path_without_query().unwrap());
        let link = request.param("country_link").unwrap_or("");
        let ctry = t_all_ctries.get_by_link(link.to_string());
        match ctry {
            Some(c) => {
                let data = c.to_page_data();
                return response.render("views/country.tpl", &data);
            },
            None => {
                let data = CountryPageData::new("Not Found".to_string());
                return response.render("views/country.tpl", &data);
            },
        }
    });
    // Page for a year for a country
    let t_all_ctries = all_ctries.clone();
    server.get("/year/:country_link/:year",
               middleware!{ |request, response|
        println!("{}", request.path_without_query().unwrap());
        if request.query().get("sort").is_some() {
            println!("got sort");
        }
        let link = request.param("country_link").unwrap();
        let year = request.param("year").unwrap().parse::<i32>().unwrap();
        let ctry = t_all_ctries.get_by_link(link.to_string());
        match ctry {
            Some(c) => {
                let data = c.to_year_data(year);
                return response.render("views/year.tpl", &data);
            }
            None => {
                let data = YearPageData {
                    eve_vec: Vec::new(),
                    name: "".to_owned(),
                    year: 0i32,
                };
                return response.render("views/year.tpl", &data);
            }
        }

    });
    // return JSON data for a country (for xjax)
    // server.get("/country/data/:country_link", middleware! {|request, response|
    //
    // });
    //




    server.listen("127.0.0.1:6767");
}
