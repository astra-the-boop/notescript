use pyo3::prelude::*;
use pyo3::types::PyModule;
use serde_json::Value;

// enums and bullshit

#[derive(Debug)]
enum Event {
    Note { pitch:String, duration:f32},
    Text(String),
    Rest,
    RepeatStart,
    RepeatEnd,
}

enum ValueType{
    str,
    bool,
    int,
    float,
    other
}


// shit that converts the thing from the python to the thing thing thing

fn parser(path: &str) -> Value {
    Python::with_gil(|py| {
        let src = include_str!("parser.py");
        let module = PyModule::from_code(py, src, "parser.py", "parser")
            .expect("failed to load parser.py");

        let raw_json: String = module
            .getattr("parse").unwrap()
            .call1((path,))
            .unwrap()
            .extract()
            .unwrap();

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
            "rest" => out.push(Event::Rest),
            "repeatStart" => out.push(Event::RepeatStart),
            "repeatEnd" => out.push(Event::RepeatEnd),
            other => panic!("unknown event: {}", other),
        }
    }

    out
}



// the actual good shit

fn inferType(pitch: &str) -> ValueType {
    match pitch {
        p if p.starts_with("A") => ValueType::str,
        p if p == "B##" || p == "B--" => ValueType::bool,
        p if p == "B#" => ValueType::int,
        p if p == "B" => ValueType::float,
        _ => ValueType::other
    }
}


//  printing

impl Event{
    fn pitch(&self)-> Option<&str>{
        if let Event::Note{pitch, ..} = self{
            Some(pitch.as_str())
        }else{None}
    }

    fn text(&self)-> Option<&str>{
        if let Event::Text(s) = self{
            Some(s.as_str())
        }else{None}
    }

    fn eventType(&self)-> Option<ValueType>{
        self.pitch().map(|p| inferType(&p))
    }

    fn isPrint(&self)-> bool{
        match self{
            Event::Note{pitch,..} if pitch.starts_with("C") => true,
            _ => false,
        }
    }
}

fn print(
    events: &[Event],
    i: usize,
    vars: &std::collections::HashMap<String, serde_json::Value>
) {
    let current = &events[i];

    if !current.isPrint() || i == 0 {
        return;
    }

    let prev = &events[i-1];

    if let Some(text) = prev.text() {
        let trimmed = text.trim();

        if let Ok(n) = trimmed.parse::<f64>() {
            println!("{}", n);
            return;
        }

        if let Some(val) = vars.get(trimmed) {
            println!("{}", val);
            return;
        }

        eprintln!("NameError: variable '{}' not found", trimmed);
        return;
    }

    if i + 1 < events.len() {
        if let Some(t) = events[i+1].text() {
            println!("{}", t);
            return;
        }
    }
}




// main function

fn main() {
    pyo3::prepare_freethreaded_python();

    let raw = parser("demo.musicxml");
    let events = fuckMyLife(&raw);

    println!("{:#?}", events);
    let mut vars = std::collections::HashMap::new();

    for(i, _events) in events.iter().enumerate() {
        print(&events,i,&vars);
    }
}