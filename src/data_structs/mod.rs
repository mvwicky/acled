#[derive(Debug, Clone, RustcDecodable, RustcEncodable)]
pub struct Event {
   gwno: String,
   event_id_cnty: String,
   event_id_no_cnty: String,
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
