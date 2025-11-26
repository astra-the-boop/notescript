use pyo3::prelude::*;
use pyo3::types::{PyModule, PyTuple};
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
            .expect("Couldn't load parser.py");

        let result = module
            .getattr("parse")
            .unwrap()
            .call1(PyTuple::new(py, &[path]))
            .unwrap();

        result.extract::<Value>().unwrap()
    })
}

fn fuckMyLife(json: &Value) -> Vec<Event> {
    let mut out = Vec::new();
    let arr = json.as_array().expect("parser shalt return a list >:c hmph");

    for i in arr{
        let obj = i.as_object().unwrap();
        let eType = obj["type"].as_str().unwrap();

        match eType {
            "note" => {
                out.push(Event::Note {
                    pitch:obj["pitch"].as_str().unwrap().to_string(),
                    duration: obj["duration"].as_f64().unwrap() as f32,
                });
            }
            "text" => {
                out.push(Event::Text(
                    obj["text"].as_str().unwrap().to_string(),
                ));
            }
            "repeatStart" => out.push(Event::RepeatStart),
            "repeatEnd" => out.push(Event::RepeatEnd),
            other => panic!("unknown event: {}", other),
        }
    }

    out

}


fn main() {
    let raw = parser("demo.musicxml");
    let events = fuckMyLife(&raw);

    println!("{:#?}", events);
}