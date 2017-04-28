extern crate mustache;
extern crate serde_json;

use std::env;
use std::fs::OpenOptions;

use serde_json::Value;
use mustache::Data;

fn to_mustache_data(value: &Value) -> Data {
    match *value {
        Value::Array(ref vec) => {
            Data::VecVal(vec.into_iter().map(| item | to_mustache_data(&item)).collect())
        },
        Value::Object(ref map) => {
            Data::Map(map.iter().map(|(key, val)| (key.clone(), to_mustache_data(&val))).collect())
        }
        Value::String(ref string) => Data::StrVal(string.clone()),
        _ => Data::StrVal(format!("{}", value)),
    }
}

fn main() {
    let mut args = env::args();
    args.next().expect("Could not get name of executable");

    let template_path = args.next().expect("You need to specify template to compile");
    let output_path = args.next().unwrap_or("out.html".to_owned());

    let values = if let Some(path) = args.next() {
        let json_file = OpenOptions::new()
            .read(true)
            .open(path)
            .expect("Unable to open JSON configuration");
        to_mustache_data(&serde_json::from_reader(&json_file).expect("Invalid JSON format on specified file."))
    } else {
        Data::VecVal(vec![])
    };

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&output_path)
        .expect("Unable to open or create output file");

    let template = mustache::compile_path(&template_path).unwrap();
    template.render_data(&mut output_file, &values).unwrap();
}
