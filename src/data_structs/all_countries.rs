use super::country::Country;

/// `AllCountries`
/// basically wraps a vector holding country objects
/// allows for searching by name, link, etc
#[derive(Clone, RustcEncodable)]
pub struct AllCountries {
    pub countries: Vec<Country>,
}

impl AllCountries {
    pub fn new() -> AllCountries {
        AllCountries {
        	countries: Vec::new(),
        }
    }
    pub fn push(&mut self, inp_ctry: Country) {
        self.countries.push(inp_ctry);
    }
    /// Push a `Country` if it isn't currently in the vector
    /// returns `false` if not added, `true` otherwise
    /// only searches by name (should hash)
    pub fn push_if_not(&mut self, inp_ctry: Country) -> bool {
        if self.countries.is_empty() {
            self.push(inp_ctry);
            true
        }
        else {
            for elem in &self.countries {
                if elem.name == inp_ctry.name {
                    return false;
                }
            }
            self.push(inp_ctry);
            true
        }
    }
    /// Push a new `Country` if it isn't currently in the vector
    /// creates a new `Country` from the input name
    /// returns `true` if not added, `true` otherwise
    pub fn push_new_if_not(&mut self, inp_name: String) -> bool {
    	if self.countries.is_empty() {
    		self.push(Country::from_name(inp_name));
    		true
    	}
    	else {
    		for elem in &self.countries {
    			if elem.name == inp_name {
    				return false;
    			}
    		}
    		self.push(Country::from_name(inp_name));
    		true
    	}
    }
    pub fn contains_name(&self, inp_name: String) -> bool {
        if self.countries.is_empty() || inp_name.is_empty() {
            false
        }
        else {
            for elem in &self.countries {
                if elem.name == inp_name {
                    return true;
                }
            }
            false
        }
    }
    pub fn get_by_link(&self, inp_link: String) -> Option<Country> {
        if self.countries.is_empty() || inp_link.is_empty() {
            None
        }
        else {
            for elem in &self.countries {
                if elem.link == inp_link {
                    return Some(elem.clone());
                }
            }
            None
        }
    }
}
