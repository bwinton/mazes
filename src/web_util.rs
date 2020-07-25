use quicksilver::{Result};
use stdweb::web::document;

pub fn get_args() -> Result<String> {
    let mut rv = "backtrack".to_owned();
    if let Some(location) = document().location() {
        match location.hash() {
            Ok(hash) => {
                if !hash.is_empty() {
                    rv = hash[1..].to_owned();
                }        
            }
            Err(_) => {}
        };
    }
    Ok(rv)
}
