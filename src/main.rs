use std::{fs, io, path::Path};

use minidom::Element;

const FEEDS_DIR_PATH: &'static str = "./feeds/";
const FEED_NS: &'static str = "feed";

#[derive(Debug)]
struct Feed {
    link: String,
    item: Element,
}

fn main() {
    let feeds = parse_dir_to_feeds(FEEDS_DIR_PATH).unwrap();

    println!("{:?}", feeds[1])
}

fn read_file<P: AsRef<Path>>(path: P) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

fn parse_dir_to_feeds(dir: &str) -> Result<Vec<Feed>, std::io::Error> {
    let mut feeds: Vec<Feed> = vec![];

    let mut it = fs::read_dir(dir)?.into_iter();
    while let Some(file) = it.next() {
        let file = file?;
        let content = read_file(file.path()).unwrap();
        feeds.push(xml_to_feed(content).unwrap());
    }
    Ok(feeds)
}

fn xml_to_feed(content: String) -> Option<Feed> {
    let root: Element = content.parse().unwrap();
    let link = root.get_child("link", FEED_NS)?.text();
    let item = root.get_child("item", FEED_NS)?.to_owned();
    Some(Feed { link, item })
}

// fn parse_element(el: Element, values: &mut Vec<String>) {
//     let mut stack = vec![el];

//     while let Some(curr) = stack.pop() {
//         if curr.children().count() == 0 {
//             // maybe implement change here

//             values.push(curr.text());
//         } else {
//             for child in curr.children() {
//                 stack.push(child.to_owned());
//             }
//         }
//     }
// }
