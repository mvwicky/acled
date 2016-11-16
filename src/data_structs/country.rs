use std::collections::BTreeMap;
use std::f64;

use super::country_page_data::CountryPageData;
use super::event::Event;
use super::event_trunc::{EventTrunc, EventTruncRenderable};
use super::year::Year;
use super::year_page_data::YearPageData;

/// `Country` contains full country data
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
    /// Create a blank `Country` from just the name
    pub fn from_name(t_name: String) -> Country {
        Country {
            events: Vec::new(),
            link: t_name.clone().replace(" ", ""),
            name: t_name,
            num_events: 0i32,
            num_fatalities: 0i32,
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
            link: self.link.clone(),
            total_eve: t_vec.len() as i32,
            years: t_vec,
        }
    }
    pub fn to_year_data(&self, inp_year: i32) -> YearPageData {
        let mut t_eve: Vec<EventTruncRenderable> = Vec::new();
        for elem in &self.events {
            if elem.year == inp_year {
                t_eve.push(EventTruncRenderable::from_event(elem));
            }
        }
        YearPageData {
            eve_vec: t_eve,
            name: self.name.clone(),
            year: inp_year,
        }
    }
}
