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
    pub events: i32,
    pub fatalities: i32,
    pub years: Vec<Year>,
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct MainPageData {
    pub countries: Vec<Country>,
}

#[derive(Debug, Clone, RustcEncodable)]
pub struct Year {
    pub eve: i32,
    pub fat: i32,
    pub year: i32,
}

impl Event {
    pub fn from_row(row: &Vec<String>) -> Event {
        let date_str: String = row[3].parse::<String>().unwrap();
        let t_date: NaiveDate = NaiveDate::parse_from_str(&date_str, "%d/%m/%Y").unwrap();
        let t_year: i32 = row[4].parse::<i32>().unwrap();
        let t_tpres: i32 = row[5].parse::<i32>().unwrap();
        let t_lat: f32 = row[19].parse::<f32>().unwrap();
        let t_lon: f32 = row[20].parse::<f32>().unwrap();
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
        let t_years: Vec<Years> = Vec::new();
        
    }
}

impl CountryPageData {
    pub fn new(t_name: String, eve: i32, fat: i32) -> CountryPageData {
        CountryPageData{
            found: true,
            name: t_name,
            events: eve,
            fatalities: fat,
            years: Vec::new(),
        }
    }
}
