/*
	CountryNameLink: hold the name and link to a country
*/

#[derive(Debug, Clone, RustcEncodable)]
pub struct CountryNameLink {
    pub name: String,
    pub link: String,
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