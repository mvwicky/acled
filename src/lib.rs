#[macro_use]
extern crate nickel;
#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate rustc_serialize;
extern crate postgres;

pub mod data_structs {
	pub mod field;
	pub mod event;
	pub mod event_trunc;
	pub mod country;
	pub mod country_page_data;
	pub mod country_name_link;
	pub mod main_page_data;
	pub mod all_countries;
	pub mod year;
}
pub mod acled_server;

pub use acled_server::AcledServer;

pub use data_structs::field::Field;
pub use data_structs::event::Event;
pub use data_structs::event_trunc::EventTrunc;
pub use data_structs::country::Country;
pub use data_structs::country_page_data::CountryPageData;
pub use data_structs::main_page_data::MainPageData;
pub use data_structs::country_name_link::CountryNameLink;