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
   pub latitude: f32,
   pub longitude: f32,
   pub geo_precision: i32,
   pub source: String,
   pub notes: String,
   pub fatalities: i32,
}


#[derive(Debug, Clone, RustcEncodable)]
pub struct Country {
    pub link: String,
    pub name: String,
    pub num_events: i32,
    pub num_fatalities: i32,
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct CountryPageData {
    pub found: bool,
    pub name: String,
    pub events: i32,
    pub fatalities: i32,
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct MainPageData {
    pub countries: Vec<Country>,
}

impl Event {
    pub fn from_row(row: Vec<String>) -> Event {
        
    }
}

impl Country {
    pub fn new(t_link: String, t_name: String, num_eve: i32, num_fat: i32) -> Country {
        Country {
            link: t_link,
            name: t_name,
            num_events: num_eve,
            num_fatalities: num_fat,
        }
    }
}

impl CountryPageData {
    pub fn new(t_name: String, eve: i32, fat: i32) -> CountryPageData {
        CountryPageData{
            found: true,
            name: t_name,
            events: eve,
            fatalities: fat,
        }
    }
}
