use std::{collections::HashMap, process::exit};

use regex::Regex;
use scraper::{Html, Selector};
use url::Url;

use crate::misc::HashMapExt;

/// Convert a string to an html document.
pub fn string_to_html_document(document_string: &str) -> Html {
    Html::parse_document(document_string)
}

/// Convert a string to an html fragment.
pub fn string_to_html_fragment(fragment_string: &str) -> Html {
    Html::parse_fragment(fragment_string)
}

/// Get the book's title from the index.
pub fn get_title_from_index(index_html: &Html) -> String {
    let selector = Selector::parse("meta").unwrap(); // Build a selector that finds the 'meta' html tag
        for element in index_html.select(&selector) {
            // Loop through all meta tags in the html document.
            match element.value().attr("name") {
                // Check if the meta tag contains attribute: "name"
                None => continue,
                Some(x) => {
                    if x == "twitter:title" {
                        // If it does contain attribute "name", check if the content of that attribute is "twitter:title"
                        return element.value().attr("content").unwrap().to_owned();
                        // If it is, extract the data from the content attribute.
                    }
                }
            }
        }
    eprintln!("Error! Unable to find book title. Royal road have probably changed their front-end code. Please report this to me on:\nhttps://github.com/Raine-gay/royal_road_archiver");
    exit(1);
}

/// Get the book's author from index
pub fn get_author_from_index(index_html: &Html) -> String {
    let selector = Selector::parse("meta").unwrap();
    for element in index_html.select(&selector) {
        match element.value().attr("property") {
            None => continue,
            Some(x) => {
                if x == "books:author" {
                    return element.value().attr("content").unwrap().to_owned();
                }
            }
        }
    }
    eprintln!("Error! Unable to find book author. Royal road have probably changed their front-end code. Please report this to me on:\nhttps://github.com/Raine-gay/royal_road_archiver");
    exit(1);
}

/// Get the book's cover image url from the index
pub fn get_cover_image_url_from_index(index_html: &Html) -> String {
    let selector = Selector::parse("meta").unwrap();
    for element in index_html.select(&selector) {
        match element.value().attr("property") {
            None => continue,
            Some(x) => {
                if x == "og:image" {
                    return element.value().attr("content").unwrap().to_owned();
                }
            }
        }
    }
    eprintln!("Error! Unable to find cover image url. Royal road have probably changed their front-end code. Please report this to me on:\nhttps://github.com/Raine-gay/royal_road_archiver");
    exit(1);
}

/// Gets the chapter names and urls from the index.
/// 
/// This gets stored in a vector where index 0 is the chapter name, and index 1 is the url.
pub fn get_chapter_names_and_urls_from_index(index_html: &Html) -> Vec<[String; 2]> {
    // I wont lie. I have almost 0 idea what a bunch of this shit does since it's highly specific to RoyalRoad.
    // I've commented in the gist of it, but we have no memory actually writing this function.

    let mut chapters: Vec<[String; 2]> = Vec::new();
    let mut raw_json_data = String::new();

    // Find a script tag that has "window.chapters" inside the inner html. This is all in json format.
    let selector = Selector::parse("script").unwrap();
    for element in index_html.select(&selector) {
        if element.inner_html().contains("window.chapters") {
            raw_json_data = element.inner_html();
            break;
        }
    }
    // Exit it if unable to find the needed json data. That probably means royal road has changed their code.
    if raw_json_data.is_empty() {
        eprintln!("Error! Unable to find json chapter data. Royal road have probably changed their front-end code. Please report this to me on:\nhttps://github.com/Raine-gay/royal_road_archiver");
        exit(1);
    }

    // I have absolutely no idea what this regex does; but it's probably important.
    const REGEX: &str = r#"window.chapters = (\[.*?]);"#;
    let regex = Regex::new(REGEX).unwrap();

    // I still have no fucking clue what this magic part does; but it works so we ain't fucking touching it.
    let chapter_raw_json = regex
        .captures(&raw_json_data)
        .unwrap()
        .get(1)
        .map_or("[]", |m| m.as_str());

    // and it just spits out json when done. Neat.
    let chapter_json: serde_json::Value = serde_json::from_str(chapter_raw_json).unwrap();

    // For each chapter in the json, do some processing to remove the quotes then shove it onto the vector.
    for chapter in chapter_json.as_array().unwrap() {
        let chapter_name = chapter["title"].to_string().replace('"', "");
        let url = format!(
            "https://www.royalroad.com{}",
            chapter["url"].to_string().replace('"', "")
        );

        chapters.push([chapter_name, url]);
    }

    // Return that wanker.
    return chapters;
}

/// Isolate chapter content from the rest of the shit on the page.
pub fn isolate_chapter_content(raw_chapter_html: &Html) -> Html {
    let page_html = Html::parse_document(&raw_chapter_html.html());

    let selector = Selector::parse("div").unwrap();
    for element in page_html.select(&selector) {
        match element.value().attr("class") {
            None => continue,
            Some(x) => {
                if x == "chapter-inner chapter-content" {
                    return string_to_html_fragment(&element.inner_html());
                }
            }
        }
    }
    eprintln!("Error! Unable to isolate chapter content");
    exit(1);
}

/// Remove all img tags from the html fragment.
pub fn remove_image_tags(html_fragment: &Html) -> String {
    let mut image_tags: Vec<String> = Vec::new();

    let selector = Selector::parse("img").unwrap();
    for element in html_fragment.select(&selector) {
        if !image_tags.contains(&element.html()) {
            image_tags.push(element.html());
        }
    }

    let mut html_fragment = html_fragment.html();

    for image_tag in image_tags {
        html_fragment = html_fragment.replace(&image_tag, "");
    }

    return html_fragment;
}

/// Extract the urls and image tags from a chapter and put them in the hashmap:
/// ``Hashmap<Url, Vec<String>>``
pub fn extract_urls_and_img_tag(chapter_html: &Html) -> HashMap<Url, Vec<String>> {
    let mut chapter_image_urls: HashMap<Url, Vec<String>> = HashMap::new();

    let selector = Selector::parse("img").unwrap();
    for element in chapter_html.select(&selector) {
        let url = element.attr("src");
        let image_tag = element.html();

        if url.is_none() { continue; }
        let url = match Url::parse(url.unwrap()) {
            Ok(url) => url,
            Err(warning) => {
                eprintln!("Warning! Unable to parse url on image tag: {image_tag}\n{warning}");
                continue;
            },
        };

        let temp_map: HashMap<Url, Vec<String>> = HashMap::from([(url, vec![image_tag])]);

        chapter_image_urls = chapter_image_urls.join(temp_map);
    }

    return chapter_image_urls;
}