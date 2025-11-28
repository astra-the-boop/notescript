use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde_json::Value;

#[derive(Debug)]
enum Event {
    Note { pitch:String, duration:f32},
    Text(String),
    RepeatStart,
    RepeatEnd,
}


fn parser(path: &str) -> Value {
    Python::with_gil(|py| {
        let src = include_str!("parser.py");

        let module = PyModule::from_code(py, src, "parser.py", "parser")
            .expect("no load parser.py :(");

        let raw_json: String = module
            .getattr("parse").unwrap()
            .call1((path,)).unwrap()
            .extract().unwrap();

        serde_json::from_str(&raw_json).unwrap()
    })
}

fn fuckMyLife(json: &Value) -> Vec<Event> {
    let mut out = Vec::new();
    let arr = json.as_array().expect("parser must return list");

    for i in arr {
        let obj = i.as_object().unwrap();
        let e_ty = obj["type"].as_str().unwrap();

        match e_ty {
            "note" => out.push(Event::Note {
                pitch: obj["pitch"].as_str().unwrap().to_string(),
                duration: obj["duration"].as_f64().unwrap() as f32,
            }),
            "text" => out.push(Event::Text(
                obj["text"].as_str().unwrap().to_string(),
            )),
            "repeatStart" => out.push(Event::RepeatStart),
            "repeatEnd" => out.push(Event::RepeatEnd),
            other => panic!("unknown event: {}", other),
        }
    }

    out
}

fn main() {
    pyo3::prepare_freethreaded_python();

    let raw = parser("demo.musicxml");
    let events = fuckMyLife(&raw);

    println!("{:#?}", events);

    let raw = parser("demo.musicxml");
    let events = fuckMyLife(&raw);

    println!("{:#?}", events);
}