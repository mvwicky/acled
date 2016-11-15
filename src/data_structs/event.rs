// Event struct: contains a row
//

use chrono::NaiveDate;

use super::field::Field;

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

impl Event {
    pub fn from_csv_row(row: &Vec<String>) -> Event {
        let date_str: String =
            row[Field::EventDate as usize].parse::<String>().unwrap().trim().to_string();
        let t_date: NaiveDate = NaiveDate::parse_from_str(&date_str, "%d/%m/%Y").unwrap();
        let t_year: i32 = row[Field::Year as usize].parse::<i32>().unwrap();
        let t_tpres: i32 = row[Field::TimePrecision as usize].parse::<i32>().unwrap();
        let t_lat: f64 = row[Field::Latitude as usize].parse::<f64>().unwrap();
        let t_lon: f64 = row[Field::Longitude as usize].parse::<f64>().unwrap();
        let t_gpres: i32 = row[Field::GeoPrecision as usize].parse::<i32>().unwrap();
        let t_fat: i32 = row[Field::Fatalities as usize].parse::<i32>().unwrap();
        Event {
            gwno: row[Field::Gwno as usize].clone().trim().to_string(),
            event_id_cnty: row[Field::EventIdCnty as usize].clone().trim().to_string(),
            event_id_no_cnty: row[Field::EventIdNoCnty as usize].clone().trim().to_string(),
            event_date: t_date,
            year: t_year,
            time_precision: t_tpres,
            event_type: row[Field::EventType as usize].clone().trim().to_string(),
            actor_1: row[Field::Actor1 as usize].clone().trim().to_string(),
            ally_actor_1: row[Field::AllyActor1 as usize].clone().trim().to_string(),
            inter_1: row[Field::Inter1 as usize].clone().trim().to_string(),
            actor_2: row[Field::Actor2 as usize].clone().trim().to_string(),
            ally_actor_2: row[Field::AllyActor2 as usize].clone().trim().to_string(),
            inter_2: row[Field::Inter2 as usize].clone().trim().to_string(),
            interaction: row[Field::Interaction as usize].clone().trim().to_string(),
            country: row[Field::Country as usize].clone().trim().to_string(),
            admin_1: row[Field::Admin1 as usize].clone().trim().to_string(),
            admin_2: row[Field::Admin2 as usize].clone().trim().to_string(),
            admin_3: row[Field::Admin3 as usize].clone().trim().to_string(),
            location: row[Field::Location as usize].clone().trim().to_string(),
            latitude: t_lat,
            longitude: t_lon,
            geo_precision: t_gpres,
            source: row[Field::Source as usize].clone().trim().to_string(),
            notes: row[Field::Notes as usize].clone().trim().to_string(),
            fatalities: t_fat,
        }
    }
}
