#![feature(proc_macro)]

#[macro_use] extern crate stdweb;
extern crate living_room_parser;

use stdweb::js_export;

#[js_export]
fn parse(s: &str) -> Option<String> {
    match living_room_parser::parse_to_json_string(s) {
        Ok(json) => Some(json),
        _ => None
    }
}
