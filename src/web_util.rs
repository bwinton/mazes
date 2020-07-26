use quicksilver::Result;
use stdweb::web::document;

pub fn get_args() -> Result<String> {
    let mut rv = "backtrack".to_owned();
    if let Some(location) = document().location() {
        match location.search() {
            Ok(search) => {
                if !search.is_empty() {
                    rv = search[1..].to_owned();
                }
            }
            Err(_) => {}
        };
    }
    Ok(rv)
}
