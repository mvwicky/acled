use std::collections::{HashMap, BTreeMap};
use std::f64;
use std::path::{Path, PathBuf};
use std::process::Command;

use chrono::NaiveDate;
use postgres::{Connection, TlsMode};
use postgres::params::IntoConnectParams;
use nickel::{Nickel, HttpRouter, Router};

use super::data_structs::main_page_data::MainPageData;

pub struct AcledServer {
    conn: Option<Connection>,
    data_file: Option<PathBuf>,
    server: Nickel,
}

impl AcledServer {
    pub fn new() -> AcledServer {
        AcledServer {
            conn: None,
            data_file: None,
            server: Nickel::new(),
        }
    }
    pub fn init(&mut self) {
        /* compile sass  */
        println!("Compiling SASS");
    }
    pub fn start(&mut self) {
        self.server.get("/", middleware! { |request, response |
            let data: Vec<i32> = Vec::new();
            println!("{}", request.path_without_query().unwrap());
            return response.render("views/main.tpl", &data);
        });
    }
    fn main_page(&self) -> MainPageData {
        MainPageData { 
            countries: Vec::new(),
        }
    }
}

