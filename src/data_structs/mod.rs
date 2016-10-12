use std::collections::{HashMap, BTreeMap};
use std::f64;

use chrono::NaiveDate;

#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Event {
    pub gwno: String,
    pub event_id_cnty: String,
    pub event_id_no_cnty: String,
    pub event_date: NaiveDate,
    pub year: i32,
    pub time_precision: i32,
    pub event_type: String,
    pub actor_1: String,
    pub ally_actor_1: String,
    pub inter_1: String,
    pub actor_2: String,
    pub ally_actor_2: String,
    pub inter_2: String,
    pub interaction: String,
    pub country: String,
    pub admin_1: String,
    pub admin_2: String,
    pub admin_3: String,
    pub location: String,
    pub latitude: f64,
    pub longitude: f64,
    pub geo_precision: i32,
    pub source: String,
    pub notes: String,
    pub fatalities: i32,
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct EventTrunc {
    pub event_date: NaiveDate,
    pub year: i32,
    pub event_type: String,
    pub actor_1: String,
    pub ally_actor_1: String,
    pub actor_2: String,
    pub ally_actor_2: String,
    pub country: String,
    pub admin_1: String,
    pub admin_2: String,
    pub admin_3: String,
    pub location: String,
    pub latitude: f64,
    pub longitude: f64,
    pub source: String,
    pub notes: String,
    pub fatalities: i32,
}


#[derive(Debug, Clone, RustcEncodable)]
pub struct Country {
    pub events: Vec<Event>,
    pub link: String,
    pub name: String,
    pub num_events: i32,
    pub num_fatalities: i32,
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct CountryPageData {
    pub found: bool,
    pub name: String,
    pub years: Vec<Year>,
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct CountryNameLink {
    pub name: String,
    pub link: String,
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct MainPageData {
    pub countries: Vec<CountryNameLink>,
}


// struct to contain all country objects
// i.e., contains all data basically
// searching for countries (by name/link) will be implemented on this
#[derive(Clone, RustcEncodable)]
pub struct AllCountries {
    pub countries: Vec<Country>,
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct Year {
    pub year: i32,
    pub events: i32,
    pub fatalities: i32,
    pub epd: String,
    pub fpe: String,
}


impl Event {
    pub fn from_row(row: &Vec<String>) -> Event {
        let date_str: String = row[3].parse::<String>().unwrap();
        let t_date: NaiveDate = NaiveDate::parse_from_str(&date_str, "%d/%m/%Y").unwrap();
        let t_year: i32 = row[4].parse::<i32>().unwrap();
        let t_tpres: i32 = row[5].parse::<i32>().unwrap();
        let t_lat: f64 = row[19].parse::<f64>().unwrap();
        let t_lon: f64 = row[20].parse::<f64>().unwrap();
        let t_gpres: i32 = row[21].parse::<i32>().unwrap();
        let t_fat: i32 = row[24].parse::<i32>().unwrap();
        Event {
            gwno: row[0].clone(),
            event_id_cnty: row[1].clone(),
            event_id_no_cnty: row[2].clone(),
            event_date: t_date,
            year: t_year,
            time_precision: t_tpres,
            event_type: row[6].clone(),
            actor_1: row[7].clone(),
            ally_actor_1: row[8].clone(),
            inter_1: row[9].clone(),
            actor_2: row[10].clone(),
            ally_actor_2: row[11].clone(),
            inter_2: row[12].clone(),
            interaction: row[13].clone(),
            country: row[14].clone(),
            admin_1: row[15].clone(),
            admin_2: row[16].clone(),
            admin_3: row[17].clone(),
            location: row[18].clone(),
            latitude: t_lat,
            longitude: t_lon,
            geo_precision: t_gpres,
            source: row[22].clone(),
            notes: row[23].clone(),
            fatalities: t_fat,
        }
    }
}

impl EventTrunc {
    pub fn from_event(event: &Event) -> EventTrunc {
        EventTrunc {
            event_date: event.event_date.clone(),
            year: event.year.clone(),
            event_type: event.event_type.clone(),
            actor_1: event.actor_1.clone(),
            ally_actor_1: event.ally_actor_1.clone(),
            actor_2: event.actor_2.clone(),
            ally_actor_2: event.ally_actor_2.clone(),
            country: event.country.clone(),
            admin_1: event.admin_1.clone(),
            admin_2: event.admin_2.clone(),
            admin_3: event.admin_3.clone(),
            location: event.location.clone(),
            latitude: event.latitude.clone(),
            longitude: event.longitude.clone(),
            source: event.source.clone(),
            notes: event.notes.clone(),
            fatalities: event.fatalities.clone(),
        }
    }
    pub fn from_row(row: &Vec<String>) -> EventTrunc {
        let date_str: String = row[3].parse::<String>().unwrap().trim().to_string();
        let t_date: NaiveDate = NaiveDate::parse_from_str(&date_str, "%d/%m/%Y").unwrap();
        let t_year: i32 = row[4].parse::<i32>().unwrap();
        let t_lat: f64 = row[19].parse::<f64>().unwrap();
        let t_lon: f64 = row[20].parse::<f64>().unwrap();
        let t_fat: i32 = row[24].parse::<i32>().unwrap();
        EventTrunc {
            event_date: t_date,
            year: t_year,
            event_type: row[6].clone().trim().to_string(),
            actor_1: row[7].clone().trim().to_string(),
            ally_actor_1: row[8].clone().trim().to_string(),
            actor_2: row[10].clone().trim().to_string(),
            ally_actor_2: row[11].clone().trim().to_string(),
            country: row[14].clone().trim().to_string(),
            admin_1: row[15].clone().trim().to_string(),
            admin_2: row[16].clone().trim().to_string(),
            admin_3: row[17].clone().trim().to_string(),
            location: row[18].clone().trim().to_string(),
            latitude: t_lat,
            longitude: t_lon,
            source: row[22].clone().trim().to_string(),
            notes: row[23].clone().trim().to_string(),
            fatalities: t_fat,
        }
    }
}


impl Country {
    pub fn new(t_link: String, t_name: String, num_eve: i32, num_fat: i32) -> Country {
        Country {
            events: Vec::new(),
            link: t_link,
            name: t_name,
            num_events: num_eve,
            num_fatalities: num_fat,
        }
    }
    pub fn to_page_data(&self) -> CountryPageData {
        let mut t_years: BTreeMap<i32, Year> = BTreeMap::new();
        for event in &self.events {
            let c_year = t_years.entry(event.year).or_insert(Year::new(event.year));
            c_year.events += 1;
            c_year.fatalities += event.fatalities;
        }
        let mut year_vec: Vec<Year> = Vec::new();
        for elem in t_years.values() {
            year_vec.push(elem.clone());
        }
        for elem in &mut year_vec {
            let f_eve: f64 = f64::from(elem.events);
            let f_fat: f64 = f64::from(elem.fatalities);
            let t_epd: f64 = f_eve / 365.0f64;
            let t_fpe: f64 = f_fat / f_eve;
            elem.epd = format!("{:.2}", t_epd);
            elem.fpe = format!("{:.2}", t_fpe);

        }
        let mut t_vec = year_vec.clone();
        t_vec.reverse();
        CountryPageData {
            found: true,
            name: self.name.clone(),
            years: t_vec,
        }

    }
}

impl CountryPageData {
    pub fn new(t_name: String, eve: i32, fat: i32) -> CountryPageData {
        CountryPageData {
            found: true,
            name: t_name,
            years: Vec::new(),
        }
    }
}

impl CountryNameLink {
    pub fn new(t_name: String) -> CountryNameLink {
        let t_link: String = t_name.replace(" ", "");
        CountryNameLink {
            name: t_name,
            link: t_link,
        }
    }
}

impl Year {
    pub fn new(t_year: i32) -> Year {
        Year {
            year: t_year,
            events: 0,
            fatalities: 0,
            epd: "".to_string(),
            fpe: "".to_string(),
        }
    }
}
