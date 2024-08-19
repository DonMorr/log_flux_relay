use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum FilterType {
    WholeMatch,   // Matches the entire string
    PartialMatch, // Matches a substring within the string
    Regex,        // Matches using a regular expression
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Filter {
    pub name: String,
    pub filter_type: FilterType,
    pub value: String,
}

impl Filter {
    // Constructor for Filter
    pub fn new(name: &str, filter_type: FilterType, value: &str) -> Self {
        Self {
            name: name.to_string(),
            filter_type,
            value: value.to_string(),
        }
    }

    // Method to check if a value matches the filter
    pub fn matches(&self, input: &str) -> bool {
        match self.filter_type {
            FilterType::WholeMatch => self.value == input,
            FilterType::PartialMatch => input.contains(&self.value),
            FilterType::Regex => {
                // Implement regex matching here
                // Example (requires `regex` crate): 
                // let re = regex::Regex::new(&self.value).unwrap();
                // re.is_match(input)
                unimplemented!("Regex matching not implemented yet")
            },
        }
    }
}
