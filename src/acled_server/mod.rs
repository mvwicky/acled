use std::collections::{HashMap, BTreeMap};
use std::f64;
use std::path::PathBuf;
use std::process::Command;

use chrono::NaiveDate;
// use std::process::Command;

use postgres::Connection;
use postgres::params::IntoConnectParams;
use nickel::{Nickel, HttpRouter, Router};

use super::data_structs::main_page_data::MainPageData;

pub struct AcledServer {
    conn: Option<Connection>,
    data_file: Option<PathBuf>,
    scss_params: ScssParams,
    server: Nickel,
}

#[derive(Clone, Debug)]
struct ScssParams {
    dir: Option<String>,
    scss_file: Option<String>,
    css_file: Option<String>,
}

impl AcledServer {
    pub fn new() -> AcledServer {
        AcledServer {
            conn: None,
            data_file: None,
            scss_params: ScssParams::new(),
            server: Nickel::new(),
        }
    }
    pub fn add_conn(&mut self, new_conn: Connection) {
        self.conn = Some(new_conn);
    }
    pub fn add_data_file(&mut self, new_file: PathBuf) {
        self.data_file = Some(new_file);
    }
    pub fn init(&mut self) {
        // compile sass
        // try to get file names
        let temp_params = self.scss_params.clone();
        let styles_dir = match temp_params.dir {
            Some(d) => d,
            None => "styles".to_owned(),
        };
        let scss_file = match temp_params.scss_file {
            Some(f) => f,
            None => "main.scss".to_owned(),
        };
        let css_file = match temp_params.css_file {
            Some(f) => f,
            None => "main.css".to_owned(),
        };
        println!("Compiling SASS");
        let scss_cmd = format!("{dir}/{scss}:{dir}/{css}",
                               dir=styles_dir,
                               scss=scss_file,
                               css=css_file);
        println!("{}", scss_cmd);
        Command::new("sass").arg(&scss_cmd).status().unwrap();
    }
    pub fn start(&mut self) {
        self.init();
        self.server.get("/",
                        middleware! { |request, response |
            let data: Vec<i32> = Vec::new();
            println!("{}", request.path_without_query().unwrap());
            return response.render("views/main.tpl", &data);
        });

        self.server.listen("127.0.0.1:6767");
    }
    fn main_page(&self) -> MainPageData {
        if self.conn.is_none() {
            panic!("No connection");
        }
        MainPageData { countries: Vec::new() }
    }
}

impl ScssParams {
    fn new() -> ScssParams {
        ScssParams {
            dir: None,
            scss_file: None,
            css_file: None,
        }
    }
}
