use super::event_trunc::EventTruncRenderable;

/// `YearPageData`
/// data sent to the year page
/// basically a list of events
#[derive(Debug, Clone, RustcEncodable)]
pub struct YearPageData {
    pub eve_vec: Vec<EventTruncRenderable>,
    pub name: String,
    pub year: i32,
}
