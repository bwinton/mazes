use crate::util::Args;
use macroquad::collections::storage;
use sapp_jsutils::JsObject;

struct Reset(bool);

pub struct Web {}

impl Web {
    pub fn new() -> Self {
        storage::store(Reset(false));
        Self {}
    }
}

extern "C" {
    fn get_search() -> JsObject;
    fn get_value(key: JsObject) -> JsObject;
    fn get_checked(key: JsObject) -> bool;
}

#[allow(unused_mut)]
fn web_get_search() -> String {
    let mut algorithm = String::new();
    let js_object = unsafe { get_search() };
    js_object.to_string(&mut algorithm);
    algorithm
}

fn web_get_value(key: &str) -> String {
    let mut value = String::new();
    let key = JsObject::string(&(key.to_owned() + " :checked"));
    let js_object = unsafe { get_value(key) };
    js_object.to_string(&mut value);
    value
}

fn web_get_checked(key: &str) -> bool {
    let key = JsObject::string(&(key.to_owned() + ":checked"));
    let rv = unsafe { get_checked(key) };
    rv
}

#[no_mangle]
extern "C" fn send_reset() {
    storage::store(Reset(true));
}

impl Args for Web {
    fn get_algorithm(&self) -> String {
        let mut algorithm = web_get_search();
        if algorithm.is_empty() {
            algorithm = "?parallel".to_string();
        }
        algorithm[1..].to_owned()
    }

    fn get_variant(&self) -> String {
        let algorithm = self.get_algorithm();
        let variant = match algorithm.as_str() {
            "parallel" => web_get_value("#parallel"),
            "aldousbroder" => {
                if web_get_checked("#aldousbroder") {
                    "fast".to_owned()
                } else {
                    "slow".to_owned()
                }
            }
            "wilson" => {
                if web_get_checked("#wilson") {
                    "slow".to_owned()
                } else {
                    "fast".to_owned()
                }
            }
            "growingtree" => web_get_value("#growingtree"),
            "bintree" => {
                let random = if web_get_checked("#bintree-random") {
                    "random"
                } else {
                    "ordered"
                };
                let element = web_get_value("#bintree-bias");
                format!("{}:{}", random, element).to_owned()
            }
            "sidewinder" => {
                if web_get_checked("#sidewinder") {
                    "hard".to_owned()
                } else {
                    "easy".to_owned()
                }
            }
            "hexparallel" => web_get_value("#hexparallel"),
            "penrose" => web_get_value("#penrose"),

            _ => "unused".to_owned(),
        };
        variant
    }

    fn needs_reset(&self) -> bool {
        let rv: bool = storage::get::<Reset>().unwrap().0;
        storage::store(Reset(false));
        rv
    }
}
