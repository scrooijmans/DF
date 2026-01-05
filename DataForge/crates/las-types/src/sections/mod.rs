// This is the module definition file - similar to a header file in C++
// These lines make other modules accessible/public
pub mod version;
pub mod well;
pub mod curve;
pub mod parameter;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// This is similar to a C++ struct/class definition
// #[derive(Debug, Clone)] are attributes (like C++ attributes) that automatically 
// implement Debug formatting and Clone (copying) functionality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HeaderItem {
    // pub makes these fields public (like public: in C++)
    // Without pub they would be private by default
    pub mnemonic: String,  // String is owned string type (like std::string)
    pub unit: String,
    pub value: String,
    pub description: String,
}

// impl is like implementing methods for a class in C++
// This block contains all associated functions and methods for HeaderItem
impl HeaderItem {
    // This is similar to a constructor in C++
    // In Rust, constructors are just regular static methods by convention named 'new'
    // &str is like const char* or string_view in C++ - a reference to string data
    pub fn new(mnemonic: &str, unit: &str, value: &str, description: &str) -> Self {
        HeaderItem {
            // to_string() converts &str to owned String (like std::string)
            mnemonic: mnemonic.to_string(),
            unit: unit.to_string(),
            value: value.to_string(),
            description: description.to_string(),
        }
    }
}

// This creates a type alias (like typedef in C++)
// SectionItems is now an alias for HashMap<String, HeaderItem>
// Similar to: typedef std::map<std::string, HeaderItem> SectionItems; in C++
pub type SectionItems = HashMap<String, HeaderItem>; 