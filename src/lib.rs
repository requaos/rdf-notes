use std::borrow::Borrow;

use sophia::graph::{*, inmem::FastGraph};
use sophia::ns::Namespace;
use sophia::parser::turtle;
use sophia::serializer::*;
use sophia::serializer::nt::NtSerializer;
use sophia::triple::stream::TripleSource;
use wasm_bindgen::prelude::*;

// Called by our JS entry point to run the example
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    // Use `web_sys`'s global `window` function to get a handle on the global
    // window object.
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;
    val.set_text_content(Some(graph_it().as_str()));

    body.append_child(&val)?;

    Ok(())
}

pub fn graph_it() -> String {
    let example = r#"
        @prefix : <http://example.org/>.
        @prefix foaf: <http://xmlns.com/foaf/0.1/>.

        :alice foaf:name "Alice";
               foaf:mbox <mailto:alice@work.example> .

        :bob foaf:name "Bob".
    "#;
    let mut graph: FastGraph = turtle::parse_str(example).collect_triples().unwrap();

    let ex = Namespace::new("http://example.org/").unwrap();
    let foaf = Namespace::new("http://xmlns.com/foaf/0.1/").unwrap();
    graph.insert(
        &ex.get("bob").unwrap(),
        &foaf.get("knows").unwrap(),
        &ex.get("alice").unwrap(),
    ).unwrap();

    let mut nt_stringifier = NtSerializer::new_stringifier();
    let example2 = nt_stringifier.serialize_graph(&mut graph).unwrap().as_str();

    return format!("The resulting graph\n{}", example2)
}