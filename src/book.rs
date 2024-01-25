use indicatif::{ProgressBar, ProgressStyle};
use scraper::Html;
use url::Url;

use crate::{html, http};

/// A struct representing a book & all the needed data to generate one.
pub struct Book {
    /// The RoyalRoad Url for the book.
    book_url: Url,

    /// The book's title.
    pub title: String,

    /// The book's author.
    pub author: String,
    
    /// A Url to the book's cover image.
    cover_image_url: Url,

    /// The raw html data of the RoyalRoad index page.
    index_html: Html,

    /// A vector of the book's chapters.
    pub chapters: Vec<Chapter>,
}

impl Book {
    /// Generate a new book instance with all the needed data from a given url.
    pub fn new(book_url: Url) -> Book {
        let index_html = html::string_to_html_document(&http::get_response(book_url.clone()).get_text());

        let chapter_names_and_urls = html::get_chapter_names_and_urls_from_index(&index_html);

        let mut chapters: Vec<Chapter> = Vec::with_capacity(chapter_names_and_urls.len());

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
            let chapter = Chapter::new(&chapter_names_and_urls[i][0], &chapter_names_and_urls[i][1]);
            chapters.push(chapter);

            progress_bar.inc(1);
        }

        progress_bar.finish();

        Book { 
            book_url: book_url, 
            title: html::get_title_from_index(&index_html),
            author: html::get_author_from_index(&index_html),
            cover_image_url: http::string_to_url(&html::get_cover_image_url_from_index(&index_html)),
            index_html: index_html,
            chapters: chapters,
        }
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
    
    /// The raw html data of the page.
    raw_chapter_html: Html,

    /// The isolated chapter html.
    pub isolated_chapter_html: Html,
}

impl Chapter {
    fn new(chapter_name: &str, chapter_url: &str) -> Self {
        let chapter_url = http::string_to_url(&chapter_url);
        let raw_chapter_html = html::string_to_html_document(&http::get_response(chapter_url.clone()).get_text());

        Chapter {
            chapter_url: chapter_url, 
            chapter_name: chapter_name.to_string(),
            raw_chapter_html: raw_chapter_html.clone(),
            isolated_chapter_html: html::isolate_chapter_content(raw_chapter_html)
        }
    }
}

// TODO!
struct BookImages {

}

// TODO!
struct BookCss {

}