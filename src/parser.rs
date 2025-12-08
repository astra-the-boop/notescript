use quick_xml::events::Event as XmlEvent;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum ParsedEvent {
    #[serde(rename = "note")]
    Note {
        pitch: String,
        duration: f64,
    },
    #[serde(rename = "rest")]
    Rest {
        duration: f64,
    },
    #[serde(rename = "text")]
    Text {
        text: String,
    },
    #[serde(rename = "repeatStart")]
    RepeatStart,
    #[serde(rename = "repeatEnd")]
    RepeatEnd,
    #[serde(rename = "voltaStart")]
    VoltaStart {
        number: String,
    },
    #[serde(rename = "voltaEnd")]
    VoltaEnd {
        number: String,
    },
}

#[derive(Debug, Default)]
struct MeasureData {
    notes: Vec<ParsedEvent>,
    has_repeat_start: bool,
    has_repeat_end: bool,
    volta_starts: Vec<(String, String)>,
    volta_ends: Vec<(String, String)>,
}

pub fn parse(filename: &str) -> Result<Vec<ParsedEvent>, Box<dyn std::error::Error>> {
    let file = File::open(filename)?;
    let buf_reader = BufReader::new(file);
    let mut reader = Reader::from_reader(buf_reader);

    let mut output = Vec::new();
    let mut buf = Vec::new();

    let mut in_part = false;
    let mut in_measure = false;
    let mut in_note = false;
    let mut in_attributes = false;
    let mut in_direction = false;
    let mut in_direction_type = false;
    let mut in_words = false;
    let mut in_barline = false;
    let mut in_repeat = false;
    let mut in_ending = false;
    let mut barline_location = String::new();

    let mut current_pitch: Option<String> = None;
    let mut current_duration: Option<f64> = None;
    let mut is_rest = false;
    let mut current_text = String::new();

    let mut current_measure = MeasureData::default();
    let mut divisions: f64 = 1.0;

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(XmlEvent::Start(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                
                match name.as_str() {
                    "part" => in_part = true,
                    "measure" if in_part => {
                        in_measure = true;
                        current_measure = MeasureData::default();
                    }
                    "attributes" if in_measure => in_attributes = true,
                    "divisions" if in_attributes => {
                        if let Ok(XmlEvent::Text(t)) = reader.read_event_into(&mut buf) {
                            if let Ok(div) = String::from_utf8_lossy(&t).parse::<f64>() {
                                divisions = div;
                            }
                        }
                    }
                    "note" if in_measure => {
                        in_note = true;
                        current_pitch = None;
                        current_duration = None;
                        is_rest = false;
                    }
                    "rest" if in_note => is_rest = true,
                    "pitch" if in_note => {
                        let mut step = String::new();
                        let mut alter = 0;
                        let mut octave = String::new();
                        
                        loop {
                            match reader.read_event_into(&mut buf) {
                                Ok(XmlEvent::Start(ref e)) => {
                                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                                    if tag == "step" {
                                        if let Ok(XmlEvent::Text(t)) = reader.read_event_into(&mut buf) {
                                            step = String::from_utf8_lossy(&t).to_string();
                                        }
                                    } else if tag == "alter" {
                                        if let Ok(XmlEvent::Text(t)) = reader.read_event_into(&mut buf) {
                                            alter = String::from_utf8_lossy(&t).parse::<i32>().unwrap_or(0);
                                        }
                                    } else if tag == "octave" {
                                        if let Ok(XmlEvent::Text(t)) = reader.read_event_into(&mut buf) {
                                            octave = String::from_utf8_lossy(&t).to_string();
                                        }
                                    }
                                }
                                Ok(XmlEvent::End(ref e)) => {
                                    let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                                    if tag == "pitch" {
                                        break;
                                    }
                                }
                                Ok(XmlEvent::Eof) => break,
                                Err(_) => break,
                                _ => {}
                            }
                        }
                        
                        let accidental = match alter {
                            1 => "#",
                            -1 => "-",
                            2 => "##",
                            -2 => "--",
                            _ => "",
                        };
                        current_pitch = Some(format!("{}{}{}", step, accidental, octave));
                    }
                    "duration" if in_note => {
                        if let Ok(XmlEvent::Text(t)) = reader.read_event_into(&mut buf) {
                            if let Ok(dur) = String::from_utf8_lossy(&t).parse::<f64>() {
                                current_duration = Some(dur / divisions);
                            }
                        }
                    }
                    "direction" if in_measure => in_direction = true,
                    "direction-type" if in_direction => in_direction_type = true,
                    "words" if in_direction_type => {
                        in_words = true;
                        current_text.clear();
                    }
                    "barline" if in_measure => {
                        in_barline = true;
                        barline_location.clear();
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                                if key == "location" {
                                    barline_location = String::from_utf8_lossy(&attr.value).to_string();
                                }
                            }
                        }
                    }
                    "repeat" if in_barline => {
                        in_repeat = true;
                        let mut direction = String::new();
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                                if key == "direction" {
                                    direction = String::from_utf8_lossy(&attr.value).to_string();
                                }
                            }
                        }
                        if direction == "forward" {
                            current_measure.has_repeat_start = true;
                        } else if direction == "backward" {
                            current_measure.has_repeat_end = true;
                        }
                    }
                    "ending" if in_barline => {
                        in_ending = true;
                        let mut ending_type = String::new();
                        let mut number = String::new();
                        for attr in e.attributes() {
                            if let Ok(attr) = attr {
                                let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                                let value = String::from_utf8_lossy(&attr.value).to_string();
                                if key == "type" {
                                    ending_type = value;
                                } else if key == "number" {
                                    number = value;
                                }
                            }
                        }
                        if ending_type == "start" {
                            current_measure.volta_starts.push((ending_type, number));
                        } else if ending_type == "stop" || ending_type == "discontinue" {
                            current_measure.volta_ends.push((ending_type, number));
                        }
                    }
                    _ => {}
                }
            }
            Ok(XmlEvent::Text(e)) => {
                if in_words {
                    current_text.push_str(&String::from_utf8_lossy(&e));
                }
            }
            Ok(XmlEvent::End(ref e)) => {
                let name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                
                match name.as_str() {
                    "part" => {
                        in_part = false;
                        break;
                    }
                    "measure" => {
                        if current_measure.has_repeat_start {
                            output.push(ParsedEvent::RepeatStart);
                        }
                        
                        for (_type, number) in &current_measure.volta_starts {
                            output.push(ParsedEvent::VoltaStart {
                                number: number.clone(),
                            });
                        }
                        
                        output.extend(current_measure.notes.drain(..));
                        
                        for (_type, number) in &current_measure.volta_ends {
                            output.push(ParsedEvent::VoltaEnd {
                                number: number.clone(),
                            });
                        }
                        
                        if current_measure.has_repeat_end {
                            output.push(ParsedEvent::RepeatEnd);
                        }
                        
                        in_measure = false;
                    }
                    "attributes" => in_attributes = false,
                    "note" => {
                        if let Some(duration) = current_duration {
                            if is_rest {
                                current_measure.notes.push(ParsedEvent::Rest { duration });
                            } else if let Some(pitch) = current_pitch.clone() {
                                current_measure.notes.push(ParsedEvent::Note { pitch, duration });
                            }
                        }
                        in_note = false;
                    }
                    "direction" => in_direction = false,
                    "direction-type" => in_direction_type = false,
                    "words" => {
                        if !current_text.is_empty() {
                            current_measure.notes.push(ParsedEvent::Text {
                                text: current_text.trim().to_string(),
                            });
                            current_text.clear();
                        }
                        in_words = false;
                    }
                    "barline" => in_barline = false,
                    "repeat" => in_repeat = false,
                    "ending" => in_ending = false,
                    _ => {}
                }
            }
            Ok(XmlEvent::Eof) => break,
            Err(e) => return Err(Box::new(e)),
            _ => {}
        }
        buf.clear();
    }

    Ok(output)
}

pub fn parse_to_json(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
    let events = parse(filename)?;
    Ok(serde_json::to_string(&events)?)
}
