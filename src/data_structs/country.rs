#[derive(Debug, Clone, RustcEncodable)]
pub struct Country {
    link: String,
    name: String,
    num_events: i32,
    num_fatalities: i32,
}
