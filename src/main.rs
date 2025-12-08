//fuck you rust
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]
#![allow(unreachable_code)]
#![allow(deprecated)]

use clap::Parser;
use std::fs;
use std::collections::HashMap;
use serde_json::Value;

mod parser;

// enums and bullshit

#[derive(Debug)]
enum Event {
    Note { pitch:String, duration:f32},
    Text(String),
    Rest,
    RepeatStart,
    RepeatEnd,
}

#[derive(Debug)]
enum Op {
    Add,
}

enum ValueType{
    str,
    bool,
    int,
    float,
    other
}

#[derive(Parser)]
#[command(name = "notescript", version, about = "she note on my script til i SyntaxError")]
struct Cli {
    filename: String,
}

// shit that converts the thing from the python to the thing thing thing
//also helpers

fn tonedeafCunt(pitch: &str) -> &str {
    pitch.trim_end_matches(|c: char| c.is_ascii_digit())
}

fn parser(path: &str) -> Value {
    let events = parser::parse(path).expect("Failed to parse MusicXML file");
    serde_json::to_value(events).expect("Failed to convert to JSON")
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

fn varProcess(
    events: &[Event],
    i: usize,
    vars: &mut HashMap<String, serde_json::Value>,
) -> Option<usize> {
    if i + 3 >= events.len() {
        return None;
    }

    let name = match &events[i] {
        Event::Text(s) => s.trim().to_string(),
        _ => return None,
    };

    let assign_note = match &events[i + 1] {
        Event::Note { pitch, .. } => tonedeafCunt(pitch),
        _ => return None,
    };
    if assign_note != "D" && assign_note != "D#" {
        return None;
    }

    let rhs_opt = match &events[i + 2] {
        Event::Text(s) => Some(s.trim().to_string()),
        _ => {
            eprintln!("SyntaxError: expected staff text at index {}", i + 2);
            return None;
        }
    };

    let type_pitch = match &events[i + 3] {
        Event::Note { pitch, .. } => tonedeafCunt(pitch),
        _ => {
            panic!("SyntaxError: expected type note at index {}", i + 3);
        }
    };

    if type_pitch == "D" {
        panic!("TypeError: 'D' is not a valid type note for '{}'", name);
    }

    match type_pitch {
        "A" | "B" | "B#" | "B##" | "B--" | "D#" => {}
        other => {
            panic!("TypeError: invalid type note '{}' for '{}'", other, name);
        }
    }

    let deref = assign_note == "D#" || type_pitch == "D#";

    let resolved: serde_json::Value = if deref {
        match rhs_opt {
            Some(ref token) => match vars.get(token) {
                Some(v) => v.clone(),
                None => {
                    panic!("NameError: variable '{}' not found", token);
                    serde_json::Value::Null
                }
            },
            None => {
                eprintln!("NameError: dereference requested but no staff text token for '{}'", name);
                serde_json::Value::Null
            }
        }
    } else {
        match rhs_opt {
            Some(ref token) => serde_json::json!(token.clone()),
            None => serde_json::Value::Null,
        }
    };

    let final_val = match type_pitch {
        "A" => {
            if let Some(s) = resolved.as_str() {
                serde_json::json!(s.to_string())
            } else {
                serde_json::json!(resolved.to_string())
            }
        }

        "B" => {
            // float
            if resolved.is_number() {
                if let Some(f) = resolved.as_f64() { serde_json::json!(f) } else { serde_json::json!(null) }
            } else if let Some(s) = resolved.as_str() {
                match s.parse::<f64>() {
                    Ok(f) => serde_json::json!(f),
                    Err(_) => {
                        panic!("TypeError: cannot parse '{}' as float for '{}'", s, name);
                        serde_json::json!(null)
                    }
                }
            } else {
                panic!("TypeError: cannot convert {:?} to float for '{}'", resolved, name);
                serde_json::json!(null)
            }
        }

        "B#" => {
            if resolved.is_i64() || resolved.is_u64() {
                if let Some(i) = resolved.as_i64() { serde_json::json!(i) }
                else if let Some(f) = resolved.as_f64() { serde_json::json!(f.floor() as i64) }
                else { serde_json::json!(null) }
            } else if let Some(f) = resolved.as_f64() {
                serde_json::json!(f.floor() as i64)
            } else if let Some(s) = resolved.as_str() {
                match s.parse::<f64>() {
                    Ok(f) => serde_json::json!(f.floor() as i64),
                    Err(_) => {
                        panic!("TypeError: cannot parse '{}' as int (via floor) for '{}'", s, name);
                        serde_json::json!(null)
                    }
                }
            } else {
                panic!("TypeError: cannot convert {:?} to int for '{}'", resolved, name);
            }
        }

        "B##" => serde_json::json!(true),

        "B--" => serde_json::json!(false),

        "D#" => resolved.clone(),

        _ => {
            panic!("TypeError: unknown type '{}' for '{}'", type_pitch, name);
        }
    };

    vars.insert(name.clone(), final_val.clone());
    // println!("VAR SET: {:?} = {:?}", name, final_val);
    //debug bs

    Some(i + 4)
}



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
    fn asOp(&self) -> Option<Op> {
        if let Event::Note { pitch, .. } = self {
            match pitch.as_str() {
                "D#" => Some(Op::Add),
                // add more shits later
                _ => None,
            }
        } else {
            None
        }
    }

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

    if !current.isPrint() {
        return;
    }

    // Check previous element if not at start
    if i > 0 {
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

            panic!("NameError: variable '{}' not found", trimmed);
        }
    }

    // Check next element
    if i + 1 < events.len() {
        if let Some(t) = events[i+1].text() {
            println!("{}", t);
            return;
        }
    }
}




// main function

fn main() {
    let args = Cli::parse();

    let raw = parser(&args.filename);
    let events = fuckMyLife(&raw);

    let mut vars = HashMap::new();
    let mut i = 0;

    while i < events.len() {
        if let Some(next_i) = varProcess(&events, i, &mut vars) {
            i = next_i;
            continue;
        }

        print(&events, i, &vars);
        i += 1;
    }
}
