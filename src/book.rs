use std::collections::HashMap;

use indicatif::{ProgressBar, ProgressStyle};
use crate::{misc::HashMapExt, GenerationError};
use scraper::Html;
use url::Url;

use crate::{file_system_crap::remove_illegal_chars, html, http};

/// A struct representing a book & all the needed data to generate one.
pub struct Book {
    /// The RoyalRoad Url for the book.
    pub book_url: Url,

    /// The book's title.
    pub title: String,

    /// Book title used for the filename.
    /// Should have illegal chars expunged via file_system_crap::remove_illegal_chars.
    pub file_name_title: String,

    /// The book's author.
    pub author: String,
    
    /// A Url to the book's cover image.
    pub cover_image_url: Url,

    /// The raw html data of the RoyalRoad index page.
    index_html: Html,

    /// A vector of the book's chapters.
    pub chapters: Vec<Chapter>,

    /// A hashmap representing the book image urls and their corresponding img html tags.
    pub image_urls_and_tags: HashMap<Url, Vec<String>>,
}

impl Book {
    /// Generate a new book instance with all the needed data from a given url.
    pub fn new(book_url: Url) -> Result<Book, GenerationError> {
        let index_html = html::string_to_html_document(&http::get_response(book_url.clone())?.get_text()?);

        let chapter_names_and_urls = html::get_chapter_names_and_urls_from_index(&index_html, &book_url)?;
        let mut chapters: Vec<Chapter> = Vec::with_capacity(chapter_names_and_urls.len());

        let mut image_urls_and_tags: HashMap<Url, Vec<String>> = HashMap::new();

        println!("\nDownloading and processing chapters:");
        // Spawn a progress bar showing how many chapters have been downloaded & processed.
        let progress_bar = ProgressBar::new(chapter_names_and_urls.len().try_into().unwrap());
        progress_bar.set_style(
            ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}%  ")
                .unwrap()
                .progress_chars("#>-"),
        );

        // Generate the chapters and add em to the book.
        for i in 0..chapter_names_and_urls.len() {
            let chapter = Chapter::new(&chapter_names_and_urls[i].0, &chapter_names_and_urls[i].1)?;

            // extract the image urls and add em to the image_urls_and_tags hashmap.
            image_urls_and_tags = image_urls_and_tags.join(html::extract_urls_and_img_tag(&chapter.isolated_chapter_html));

            chapters.push(chapter);

            progress_bar.inc(1);
        }

        progress_bar.finish();

        let title = html::get_title_from_index(&index_html, &book_url)?;

        let book = Book {
            author: html::get_author_from_index(&index_html, &book_url)?,
            cover_image_url: html::get_cover_image_url_from_index(&index_html, &book_url)?,
            book_url: book_url, 
            title: title.clone(),
            file_name_title: remove_illegal_chars(title),
            index_html: index_html,
            chapters: chapters,
            image_urls_and_tags: image_urls_and_tags,
        };

        return Ok(book);
    }

    /// Count how many paragraphs are in the book.
    pub fn count_paragraphs(&self) -> u128 {
        // TODO!
        0
    }
}

/// A struct representing a chapter.
pub struct Chapter {
    /// The Url of the chapter.
    chapter_url: Url,
    
    /// The name of the chapter.
    pub chapter_name: String,
    
    /// The raw html data of the chapter page.
    raw_chapter_html: Html,

    /// The isolated chapter html.
    pub isolated_chapter_html: Html,
}

impl Chapter {
    fn new(chapter_name: &str, chapter_url: &str) -> Result<Self, GenerationError> {
        let chapter_url = http::string_to_url(&chapter_url)?;
        let raw_chapter_html = html::string_to_html_document(&http::get_response(chapter_url.clone())?.get_text()?);

        let chapter = Chapter {
            isolated_chapter_html: html::isolate_chapter_content(&raw_chapter_html, &chapter_url)?,
            chapter_url: chapter_url, 
            chapter_name: chapter_name.to_string(),
            raw_chapter_html: raw_chapter_html,
        };

        return Ok(chapter);
    }
}