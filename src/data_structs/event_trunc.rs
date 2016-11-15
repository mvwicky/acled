use chrono::NaiveDate;

use super::field::Field;
use super::event::Event;

/// `EventTrunc`
///  contains certain fields from a row
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
    pub fn from_csv_row(row: &Vec<String>) -> EventTrunc {
        let date_str: String =
            row[Field::EventDate as usize].parse::<String>().unwrap().trim().to_string();
        let t_date: NaiveDate = NaiveDate::parse_from_str(&date_str, "%d/%m/%Y").unwrap();
        let t_year: i32 = row[Field::Year as usize].parse::<i32>().unwrap();
        let t_lat: f64 = row[Field::Latitude as usize].parse::<f64>().unwrap();
        let t_lon: f64 = row[Field::Longitude as usize].parse::<f64>().unwrap();
        let t_fat: i32 = row[Field::Fatalities as usize].parse::<i32>().unwrap();
        EventTrunc {
            event_date: t_date,
            year: t_year,
            event_type: row[Field::EventType as usize].clone().trim().to_string(),
            actor_1: row[Field::Actor1 as usize].clone().trim().to_string(),
            ally_actor_1: row[Field::AllyActor1 as usize].clone().trim().to_string(),
            actor_2: row[Field::Actor2 as usize].clone().trim().to_string(),
            ally_actor_2: row[Field::AllyActor2 as usize].clone().trim().to_string(),
            country: row[Field::Country as usize].clone().trim().to_string(),
            admin_1: row[Field::Admin1 as usize].clone().trim().to_string(),
            admin_2: row[Field::Admin2 as usize].clone().trim().to_string(),
            admin_3: row[Field::Admin3 as usize].clone().trim().to_string(),
            location: row[Field::Location as usize].clone().trim().to_string(),
            latitude: t_lat,
            longitude: t_lon,
            source: row[Field::Source as usize].clone().trim().to_string(),
            notes: row[Field::Notes as usize].clone().trim().to_string(),
            fatalities: t_fat,
        }
    }
}
