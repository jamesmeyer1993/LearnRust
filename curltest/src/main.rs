extern crate curl;

use std::vec::Vec;
use curl::easy::Easy;

struct HtmlElem {
    start: Option<String>,
    end: Option<String>,
}

impl HtmlElem {
    fn new_div() -> HtmlElem {
        HtmlElem {
            start: Some(String::from("<div>")),
            end: Some(String::from("</div>")),
        }
    }
}

fn http_get(url: &String) -> String {
    let mut dst: Vec<u8> = Vec::new();

    let mut easy = Easy::new();
    easy.url(url).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }

    let mut buff;
    unsafe {
        buff = String::from_utf8_unchecked(dst);
    }
    return buff;
}

fn main() {

    let domain = String::from("https://www.google.com");
    let page: String = http_get(&domain);
    println!("{}", page);

    let mut start: i32 = 0;
    let mut end: i32 = 0;
    match page.find("<style>") {
        Some(x) => start = x as i32,
        None => start = -1,
    }
    match page.find("</style>") {
        Some(x) => end = x as i32,
        None => end = -1,
    }

    let div = HtmlElem::new_div();
    let elements: Vec<_>;
    match div.start {
        Some(s) => elements = page.match_indices(s).collect(),
        None => panic!(""),
    }

    for int in elements.iter() {
        println!("{}", int);
    }
}
