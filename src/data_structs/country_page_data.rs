use super::country::Country;
use super::year::Year;

/// `CountryPageData`
/// data actually sent to page
#[derive(Debug, Clone, RustcEncodable)]
pub struct CountryPageData {
    pub found: bool,
    pub name: String,
    pub link: String,
    pub total_eve: i32,
    pub years: Vec<Year>,
}

impl CountryPageData {
    pub fn new(t_name: String) -> CountryPageData {
        CountryPageData {
            found: true,
            name: t_name.clone(),
            link: t_name.replace(" ", ""),
            total_eve: 0,
            years: Vec::new(),
        }
    }
    pub fn from_country(ctry: Country) -> CountryPageData {
        CountryPageData::new(ctry.name)
    }
}
