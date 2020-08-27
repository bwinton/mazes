use quicksilver::Result;
// use quicksilver::log;
use crate::stdweb::unstable::TryInto;
use crate::util::Args;
use stdweb::web::{document, html_element::OptionElement, IParentNode};

pub struct Web {}

impl Web {
    pub fn new() -> Self {
        Self {}
    }
}

impl Args for Web {
    fn get_args(&self) -> Result<String> {
        let mut algorithm = "parallel".to_owned();
        if let Some(location) = document().location() {
            match location.search() {
                Ok(search) => {
                    if !search.is_empty() {
                        algorithm = search[1..].to_owned();
                    }
                }
                Err(_) => {}
            };
        };
        Ok(algorithm)
    }

    fn get_variant(&self) -> String {
        let mut algorithm = "parallel".to_owned();
        if let Some(location) = document().location() {
            match location.search() {
                Ok(search) => {
                    if !search.is_empty() {
                        algorithm = search[1..].to_owned();
                    }
                }
                Err(_) => {}
            };
        };
        let variant = match algorithm.as_str() {
            "parallel" => {
                let element = document()
                    .query_selector("#parallel :checked")
                    .unwrap()
                    .unwrap();
                let element: OptionElement = element.try_into().unwrap();
                element.value()
            }
            "aldousbroder" => {
                let element = document().query_selector("#aldousbroder:checked").unwrap();
                if element.is_some() {
                    "fast".to_owned()
                } else {
                    "slow".to_owned()
                }
            }
            "wilson" => {
                let element = document().query_selector("#wilson:checked").unwrap();
                if element.is_some() {
                    "slow".to_owned()
                } else {
                    "fast".to_owned()
                }
            }
            "growingtree" => {
                let element = document()
                    .query_selector("#growingtree :checked")
                    .unwrap()
                    .unwrap();
                let element: OptionElement = element.try_into().unwrap();
                element.value()
            }
            _ => "unused".to_owned(),
        };
        variant
    }
}
