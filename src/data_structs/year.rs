/*
    Year: contains year stats (country indep.)
*/

#[derive(Debug, Clone, RustcEncodable)]
pub struct Year {
    pub year: i32,
    pub events: i32,
    pub fatalities: i32,
    pub epd: String,
    pub fpe: String,
}

impl Year {
    pub fn new(t_year: i32) -> Year {
        Year {
            year: t_year,
            events: 0,
            fatalities: 0,
            epd: "".to_string(),
            fpe: "".to_string(),
        }
    }
}
