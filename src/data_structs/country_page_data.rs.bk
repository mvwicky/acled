/*
	CountryPageData: data actually sent to page
*/

use super::country::Country;
use super::year::Year;

#[derive(Debug, Clone, RustcEncodable)]
pub struct CountryPageData {
    pub found: bool,
    pub name: String,
    pub years: Vec<Year>,
}

impl CountryPageData {
    pub fn new(t_name: String) -> CountryPageData {
        CountryPageData {
            found: true,
            name: t_name,
            years: Vec::new(),
        }
    }
    pub fn from_country(ctry: Country) -> CountryPageData {
    	CountryPageData::new(ctry.name)
    }
}