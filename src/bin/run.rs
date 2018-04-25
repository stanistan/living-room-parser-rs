extern crate living_room_parser;
extern crate serde_json;

fn main() {
    let input: String = std::env::args().skip(1).take(1).collect();
    let parsed = living_room_parser::parse(&input)
        .expect("failed to parse");

    let json = serde_json::to_string_pretty(&parsed)
        .expect("failed to convert to json");

    println!("{}", json);
}
