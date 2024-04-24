mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn format_citations(citations: String) -> String {
    let citations = citations.lines();
    // remove bys from the start of the citation
    let mut citations:Vec<&str> = citations.map(|citation| {
        if &citation[0..=2] == "By " {
            &citation[3..]
        } else {
            citation
        }
    }).collect();

    citations.sort_by_key(|citation| {
        citation.strip_prefix('"').unwrap_or(citation)
    });

    let citations:Vec<String> = citations.iter().map(| s| {
        let mut s = s.to_string();
        s.push('\n');
        s
    }).collect();

    citations.concat()
}
