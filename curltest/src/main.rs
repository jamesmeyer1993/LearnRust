extern crate curl;

use std::io::{stdout, Write};
use std::vec::Vec;
use curl::easy::Easy;

// Write the contents of a webpage to stdout
fn http_get_print(url: &String) {
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    easy.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
    easy.perform().unwrap();
}

fn http_get_to_vec(url: &String) {
    let mut dst = Vec::new();
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

    println!("Vec len = {}", dst.len());
}

// this function does not compile. Need to learn more about vectors to get it straight.
fn http_get_to_vec2(url: &String) {
    let mut dst: Vec<u8> = Vec::new();
    let mut easy = Easy::new();
    easy.url(url).unwrap();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            let len = data.len();
            dst.append(&mut data); // apparently faster
            Ok(len)
        }).unwrap();
    }
    println!("Vec len = {}", dst.len());
}

fn main() {

    //http_get_print(&String::from("https://www.reddit.com"));
    //http_get_to_vec(&String::from("https://www.reddit.com"));
    http_get_to_vec2(&String::from("https://www.reddit.com"));
}
