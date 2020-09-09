extern crate wasm_bindgen;

mod hd_script_parser;
mod xml_parser;
mod dependency_parser;

use xml_parser::{Node, XmlElement};
use std::collections::{HashSet, HashMap};

use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

extern crate web_sys;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[derive(Serialize, Deserialize)]
pub enum Component {
    Text,
    Number,
    TrueFalse,
    Date,
    Image,
    MultipleChoice,
    Computation(HashSet<String>),
    Dialog { children: Vec<String>, script: HashSet<String> }
}

fn parse_script_node(element: &XmlElement) -> HashSet<String> {
    let script = 
        element
        .children
        .iter()
        .filter(|e| match e {
            Node::XmlElement(xe) if xe.name == "hd:script" => true,
            _ => false
        })
        .nth(0);

    
    match script {
        Some(Node::XmlElement(xe)) if xe.children.len() > 0 => {
            let script_body = xe.children[0].as_value();
            let (_, ast) = hd_script_parser::parse(script_body).unwrap();
            dependency_parser::parse(&ast)
        },
        _ => HashSet::new()
    }
}

#[wasm_bindgen]
pub fn analyze(component: &str) -> JsValue {
    let parsed_cmp = 
        match xml_parser::parse(component) {
            Ok((_, cmp)) => cmp,
            Err(msg) => {
                log!("{}", msg);
                panic!("AARRGHHH");
            }
        };
    let components = parsed_cmp.children[1].as_element();

    let component_map = components
        .children
        .iter()
        .map(|e| match e {
            Node::XmlElement(xe) if xe.name == "hd:text" => Some((xe.attributes["name"].to_owned(), Component::Text)),
            Node::XmlElement(xe) if xe.name == "hd:number" => Some((xe.attributes["name"].to_owned(), Component::Number)),
            Node::XmlElement(xe) if xe.name == "hd:date" => Some((xe.attributes["name"].to_owned(), Component::Date)),
            Node::XmlElement(xe) if xe.name == "hd:trueFalse" => Some((xe.attributes["name"].to_owned(), Component::TrueFalse)),
            Node::XmlElement(xe) if xe.name == "hd:image" => Some((xe.attributes["name"].to_owned(), Component::Image)),
            Node::XmlElement(xe) if xe.name == "hd:multipleChoice" => {
                let name = 
                    match &xe.attributes["name"] {
                        x if x.ends_with("_SelectionVariable") => x.replace("_SelectionVariable", ""),
                        x if x.ends_with("_MultiSelectVariable") => x.replace("_MultiSelectVariable", ""),
                        x => x.to_owned()
                    };

                Some((name, Component::MultipleChoice))
            },
            Node::XmlElement(xe) if xe.name == "hd:dialog" => {
                let contents = 
                    xe
                    .children
                    .iter()
                    .filter(|e| match e {
                        Node::XmlElement(xe) if xe.name == "hd:contents" => true,
                        _ => false
                    })
                    .nth(0);
                    
                let children =
                    match contents {
                        Some(Node::XmlElement(xe)) => {
                            xe.children.iter().map(|e| e.as_element().attributes["name"].to_owned()).collect()
                        },
                        _ => vec![]
                    };
                
                Some((xe.attributes["name"].to_owned(), Component::Dialog { children: children, script: parse_script_node(xe) }))
            },
            Node::XmlElement(xe) if xe.name == "hd:computation" => {
                match &xe.attributes["name"] {
                    x if x.ends_with("_OptionTable") => None,
                    x if x.ends_with("_TableVariable") => None,
                    x => Some((x.to_owned(), Component::Computation(parse_script_node(xe))))
                }
            },
            _ => None,
        })
        .fold(
            HashMap::new(), 
            |mut acc, e| match e {
                Some((name, cmp)) => {
                    acc.insert(name, cmp);
                    acc
                },
                _ => acc
            });

    JsValue::from_serde(&component_map).unwrap()
}
