use std::{collections::HashMap, fs::OpenOptions, io::Write, path::PathBuf, process::exit, sync::{Mutex, MutexGuard}};

use bytes::Buf;
use chrono::prelude::Local;
use clap::Args;
use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use file_system_crap::convert_path_to_os_specific;
use html::{html_to_xhtml, remove_image_tags, string_to_html_fragment};
use lazy_static::lazy_static;
use indicatif::{ProgressBar, ProgressStyle};
use misc::Oses;
use reqwest::header::ToStrError;
use thiserror::Error;
use url::Url;

mod book;
mod constants;
mod file_system_crap;
mod html;
mod http;
mod misc;

/// struct that corresponds to arguments for Audiobook generation.
#[derive(Args, Debug)]
pub struct AudiobookArgs {
    /// Disable the generation of chapter titles in the audio file. Useful to avoid chapter titles appearing twice.
    #[arg(short='c', long)]
    pub no_chapter_titles: bool,

    /// Split the novel into multiple audio files by chapter.
    #[arg(short, long)]
    pub split_novel_by_chapters: bool,
}

/// struct that corresponds to arguments for Epub generation.
#[derive(Args, Debug)]
pub struct EpubArgs {
    /// Disable the inclusion of images.
    /// Will speed up epub generation and significantly decrease epub size.
    #[arg(short, long)]
    pub no_images: bool,
}

/// struct that corresponds to arguments for Html generation.
#[derive(Args, Debug)]
pub struct HtmlArgs {

}

/// struct that corresponds to arguments for Markdown generation.
#[derive(Args, Debug)]
pub struct MarkdownArgs {
    /// Disable the generation of chapter titles. Useful to avoid chapter titles appearing twice.
    #[arg(short='c', long)]
    pub no_chapter_titles: bool,

    /// Disables the inclusion of html image tags in the markdown.
    #[arg(short, long)]
    pub no_image_tags: bool,
}

lazy_static! {
    static ref WARNINGS: Mutex<GenerationWarnings> = Mutex::new(GenerationWarnings::new());
}

/// Generate an audiobook from the given arguments, url, & outputs it to the output directory.
/// 
/// This function DOES NOT do any error checking on the Url or output directory & WILL panic if they are wrong. 
/// Make sure the Url is valid and the output directory is writable BEFORE passing them to this.
pub fn generate_audiobook(audiobook_args: AudiobookArgs, book_url: Url, output_directory: PathBuf) -> Result<MutexGuard<'static, GenerationWarnings>, GenerationError> {
    return Err(GenerationError::GenerationUnsupportedError);
}

/// Generate an epub file from the given arguments, url, & outputs it to the output directory.
/// 
/// This function DOES NOT do any error checking on the Url or output directory & WILL panic if they are wrong. 
/// Make sure the Url is valid and the output directory is writable BEFORE passing them to this.
pub fn generate_epub(epub_args: EpubArgs, book_url: Url, output_directory: PathBuf) -> Result<MutexGuard<'static, GenerationWarnings>, GenerationError> {
    let book = book::Book::new(book_url)?;

    // Initialize the epub builder.
    let mut epub_builder = EpubBuilder::new(ZipLibrary::new().unwrap()).unwrap();

    // Add author and title metadata.
    epub_builder.stylesheet(constants::EPUB_CSS.as_bytes()).unwrap(); // Use the epub_css in the constants.rs file.
    epub_builder
        .metadata("author", &book.author)
        .expect("Unable to add author metadata");
    epub_builder
        .metadata("title", &book.title)
        .expect("Unable to add title metadata");

    // Download the cover image & add it to the epub.
    let cover_image = http::get_response(book.cover_image_url)?;
    let (cover_mime_type, cover_file_extension) = cover_image.get_content_type_and_file_extension();
    epub_builder.add_cover_image(
        format!("cover.{cover_file_extension}"), 
        cover_image.get_bytes()?.to_vec().as_slice(), 
        cover_mime_type).expect("Error! Unable to add cover image.");

    // Generate the cover xhtml.
    let cover_xhtml = format!(
        r#"<head></head><body><div style="text-align: center;">
        <h1><a href="{0}">{1}</a></h1>
        <img src="cover.{2}"/>
        <h2>by: {3}</h2>
        <h3>Archived on: {4}</h3></div></body>"#,
        book.book_url,
        book.title,
        cover_file_extension,
        book.author,
        chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
    );
    let cover_xhtml = format!("{0}{cover_xhtml}{1}", constants::EPUB_XML_HEAD, constants::EPUB_XML_TAIL);

    // Add the cover xhtml to the epub.
    epub_builder.add_content(
        EpubContent::new("title.xhtml", cover_xhtml.as_bytes())
            .title("Cover")
            .reftype(ReferenceType::Cover),
    ).expect("Error! Unable to add cover");

    // Add a table of contents after the cover page.
    epub_builder.inline_toc();

    // Setup html2xhtml on the operating system.
    let html2xhtml_dir = file_system_crap::setup_html2xhtml()?;

    let mut old_tags_new_tags: HashMap<String, String> = HashMap::new();

    if !epub_args.no_images {
        // Download the images and add em to the epub.

        println!("\nDownloading and processing images:");
        // Spawn a progress bar showing how many images have been downloaded & processed.
        let progress_bar = ProgressBar::new(book.image_urls_and_tags.keys().len().try_into().unwrap());
        progress_bar.set_style(
            ProgressStyle::with_template("[{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}%  ")
                .unwrap()
                .progress_chars("#>-"),
        );

        let mut i: usize = 0;
        for image_url in book.image_urls_and_tags.keys() {
            let image = http::get_response(image_url.clone())?;
            let (image_mime_type, image_file_extension) = image.get_content_type_and_file_extension();
            epub_builder.add_resource(
                format!("image_{i}.{image_file_extension}"), 
                image.get_bytes()?.to_vec().reader(), 
                image_mime_type).expect("Error! Unable to add content image");
            
            for image_tag in book.image_urls_and_tags[image_url].clone() {
                old_tags_new_tags.insert(image_tag.clone(), html::replace_img_src(image_tag, format!("image_{i}.{image_file_extension}")));
            }

            i+=1;
            progress_bar.inc(1);
        }

        progress_bar.finish();
    }

    // Convert the html to xhtml and add the xhtml to the epub for each chapter.
    for (i, chapter) in book.chapters.iter().enumerate() {

        let xhtml: String;
        if epub_args.no_images {
            xhtml = html_to_xhtml(string_to_html_fragment(&remove_image_tags(&chapter.isolated_chapter_html)), &html2xhtml_dir)?
        }
        else {
            let mut replaced_html = chapter.isolated_chapter_html.html();
            for old_img_tag in old_tags_new_tags.keys() {
                replaced_html = replaced_html.replace(&old_img_tag.clone(), &old_tags_new_tags[old_img_tag]);
            }

            xhtml = html_to_xhtml(string_to_html_fragment(&replaced_html), &html2xhtml_dir)?;
        }

        epub_builder.add_content(EpubContent::new(format!("chapter_{}.xhtml", i+1), xhtml.as_bytes())
            .title(chapter.chapter_name.clone())
            .reftype(ReferenceType::Text)).expect("Error! Unable to add chapter");
    }

    // Generate the finished epub data as a byte vector.
    let mut finished_epub: Vec<u8> = vec![];
    epub_builder.generate(&mut finished_epub).expect("Unable to generate epub data");

    // Create the epub file and write the finished epub data to it.
    let output_path = convert_path_to_os_specific(output_directory.join(format!("{0}.epub", book.file_name_title)));
    let mut output_file = match OpenOptions::new().write(true).create_new(true).open(&output_path) {
        Ok(output_file) => output_file,
        Err(error) => {
            eprintln!("Error! Unable to create: {0}\n{error}", output_path.to_string_lossy());
            exit(1);
        }
    };

    output_file.write_all(finished_epub.as_slice())
        .expect(format!("Unable to write finished epub data to {0}", output_path.to_string_lossy()).as_str());

    // Delete the html2xhtml temp directory. It's good to clean up after yourself.
    file_system_crap::delete_html2xhtml(html2xhtml_dir);

    Ok(WARNINGS.lock().unwrap())
}

/// Generate an html archive from the given arguments, url, & outputs it to the output directory.
/// 
/// This function DOES NOT do any error checking on the Url or output directory & WILL panic if they are wrong. 
/// Make sure the Url is valid and the output directory is writable BEFORE passing them to this.
pub fn generate_html(html_args: HtmlArgs, book_url: Url, output_directory: PathBuf) -> Result<MutexGuard<'static, GenerationWarnings>, GenerationError> {
    return Err(GenerationError::GenerationUnsupportedError);
}

/// Generate a markdown file from the given arguments, url, & outputs it to the output directory.
/// 
/// This function DOES NOT do any error checking on the Url or output directory & WILL panic if they are wrong. 
/// Make sure the Url is valid and the output directory is writable BEFORE passing them to this.
pub fn generate_markdown(markdown_args: MarkdownArgs, book_url: Url, output_directory: PathBuf) -> Result<MutexGuard<'static, GenerationWarnings>, GenerationError> {
    let book = book::Book::new(book_url)?;

    let output_path = convert_path_to_os_specific(output_directory.join(format!("{0}.md", book.file_name_title)));

    // Create the md file. This will crash if it already exists or can not be created.
    let mut output_file = match OpenOptions::new().write(true).create_new(true).open(&output_path) {
        Ok(output_file) => output_file,
        Err(error) => {
            return Err(GenerationError::FileCreationError{error, file_path: output_path});
        }
    };

    // Append the book title & author.
    let buf = format!("{}\n\nby: {}", &book.title, &book.author);
    output_file.write_all(buf.as_bytes()).unwrap();

    let buf = format!(
        "\nArchived on: {}\n\n",
        Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
    );
    output_file.write_all(buf.as_bytes()).unwrap();

    for chapter in book.chapters {
        let mut buf;

        if !markdown_args.no_chapter_titles {
            buf = format!("----\n{}", chapter.chapter_name);
            output_file.write_all(buf.as_bytes()).unwrap();
        }

        if markdown_args.no_image_tags {
            // Remove image tags or not depending on args.
            buf = format!("\n\n{}\n\n", html2md::parse_html(&html::remove_image_tags(&chapter.isolated_chapter_html)));

        } else {
            buf = format!("\n\n{}\n\n", html2md::parse_html(&chapter.isolated_chapter_html.html()));
        }

        output_file.write_all(buf.as_bytes()).unwrap();
    }

    Ok(WARNINGS.lock().unwrap())
}

/// An error struct representing all the documented errors that can occur while archiving a RoyalRoad webnovel.
#[derive(Error, Debug)]
pub enum GenerationError {
    /// Represents errors during file creation.
    #[error("Unable to create file: {file_path}\n{error}")]
    FileCreationError{error: std::io::Error, file_path: PathBuf},

    /// Represents errors when getting a Response from a Url.
    #[error("Unable to get response for: {url}\n{error}")]
    ResponseGetError{error: reqwest::Error, url: Url},

    /// Represents errors when converting a Response to a String.
    #[error("Unable to convert response to text: {error}")]
    ResponseConvertToTextError{error: reqwest::Error},

    /// Represents errors when converting a Response to Bytes.
    #[error("Unable to convert response to bytes: {error}")]
    ResponseConvertToBytesError{error: reqwest::Error},

    /// Represents errors when trying to parse a String to a Url.
    #[error("Unable to parse a valid Url from: {string_url}\n{error}")]
    UrlParseError{error: url::ParseError, string_url: String},

    /// Represents io errors when trying to create a temporary directory.
    #[error("Unable to create temporary directory: {error}")]
    TempDirCreationError{error: std::io::Error},

    /// Represents an error when trying to extract the html2xhtml binaries into the temporary directory.
    #[error("Unable to extract html2xhtml into the temporary directory: {error}")]
    Html2XhtmlExtractionError{error: zip_extract::ZipExtractError},

    /// Represents an error when trying to start html2xhtml.
    #[error("Unable to start html2xhtml: {error}")]
    Html2XhtmlStartError{error: std::io::Error},

    /// Represents an error when trying to find the book title.
    #[error("Unable to fetch the book title for: {url}")]
    BookTitleFetchError{url: Url},

    /// Represents an error when trying to find the book author.
    #[error("Unable to fetch the book author for: {url}")]
    BookAuthorFetchError{url: Url},

    /// Represents an error when trying to find the book cover image url.
    #[error("Unable to fetch the book cover image url: {url}")]
    BookCoverImageUrlFetchError{url: Url},

    /// Represents an error when trying to find the chapter names and urls.
    /// 
    /// This typically occurs due to RoyalRoad changing their json scheme.
    #[error("Unable to fetch the chapter names and urls for: {url}")]
    BookChapterNameAndUrlFetchError{url: Url},

    /// Represents an error when trying to isolate the chapter content.
    #[error("Unable to isolate chapter content for: {url}")]
    ChapterContentIsolationError{url: Url},

    /// Represents an error for when the target os is unsupported.
    #[error("{os} is unsupported")]
    OsUnsupportedError{os: Oses},

    /// Represents an error that shows the generation method is unsupported.
    #[error("This generation mode is currently unsupported")]
    GenerationUnsupportedError,
}


/// A struct that contains a vector of warnings.
pub struct GenerationWarnings{warnings: Vec<Warning>}

impl GenerationWarnings {
    fn new() -> Self {
        GenerationWarnings { 
            warnings: Vec::new(),
        }
    }
    
    /// Push a warning into this struct.
    pub fn add_warning(&mut self, warning: Warning) {
        self.warnings.push(warning);
    }

    pub fn get_warnings(&self) -> &Vec<Warning> {
        &self.warnings
    }

    /// Returns how many warnings have been accumulated.
    pub fn warnings_count(&self) -> usize {
        self.warnings.len()
    }
}

/// An enum to represent a warning.
#[derive(Error, Debug)]
pub enum Warning {
    /// Warning for when no ``content-type`` header can be found in the Response headers.
    #[error("{warning_msg}")]
    MissingContentType {
        warning_msg: String,
        url: Url,
        error: ToStrError,
    },

    /// Warning for when a temporary directory is unable to be deleted.
    #[error("{warning_msg}")]
    TempDirDeletionError {
        warning_msg: String,
        temp_directory_path: PathBuf,
        error: std::io::Error,
    },

    /// Warning for when the program can not parse a url in an image tag.
    #[error("{warning_msg}")]
    ImageTagParseError {
        warning_msg: String,
        raw_image_tag: String,
        error: url::ParseError,
    }
}