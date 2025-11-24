use pyo3::prelude::*;
use pyo3::types::{PyModule, PyTuple};
use serde_json::Value;

fn parser(path:&str) -> Value{
    Python::with_gil(|py| {
        let src = include_str!("parser.py");

        let module = PyModule::from_code(py, src, "parser.py", "parser"
        ).expect("Couldn't load parser.py");

        let result = module.getattr("parse").unwrap()
            .call1(PyTuple::new(py,&[path])).unwrap();

        result.extract::<Value>().unwrap()
    })

}


fn main() {}