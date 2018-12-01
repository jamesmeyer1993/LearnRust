extern crate curl;

use std::vec::Vec;
use curl::easy::Easy;

fn http_get(url: &String) -> Vec<u8> {
    let mut dst: Vec<u8> = Vec::new();
    //let mut buff = String::new();
    let mut easy = Easy::new();
    easy.url(url).unwrap();

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            //buff = String::from_utf8_lossy(data).to_string();
            Ok(data.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    return dst;
    //println!("Vec len = {}", dst.len());
}

fn main() {

    let domain = String::from("https://www.google.com");
    let mut u8_page: Vec<u8> = http_get(&domain);
    println!("{} bytes collected from {}\n", u8_page.len(), domain);
    //let mut str_page = String::from_utf8(u8_page).expect("Found invalid UTF-8");

    unsafe {
        let str_page = String::from_utf8_unchecked(u8_page);
        println!("{}", str_page);
    }
}
