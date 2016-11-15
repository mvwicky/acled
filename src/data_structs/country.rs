/*
	Country: contains full country data 
*/

use std::collections::BTreeMap;
use std::f64;

use super::country_page_data::CountryPageData;
use super::event::Event;
use super::year::Year;

#[derive(Debug, Clone, RustcEncodable)]
pub struct Country {
    pub events: Vec<Event>,
    pub link: String,
    pub name: String,
    pub num_events: i32,
    pub num_fatalities: i32,
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