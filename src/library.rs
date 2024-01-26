use std::{fs::OpenOptions, io::Write, path::PathBuf, process::exit};

use chrono::prelude::Local;
use clap::Args;
use epub_builder::{EpubBuilder, EpubContent, ReferenceType, ZipLibrary};
use file_system_crap::convert_path_to_os_specific;
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
    #[arg(short, long)]
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
    #[arg(short, long)]
    pub no_chapter_titles: bool,

    /// Disables the inclusion of html image tags in the markdown.
    #[arg(short='i', long)]
    pub no_image_tags: bool,
}

/// Generate an audiobook from the given arguments, url, & outputs it to the output directory.
/// 
/// This function DOES NOT do any error checking on the Url or output directory & WILL panic if they are wrong. 
/// Make sure the Url is valid and the output directory is writable BEFORE passing them to this.
pub fn generate_audiobook(audiobook_args: AudiobookArgs, book_url: Url, output_directory: PathBuf) {
    eprintln!("This is not implemented yet.");
}

/// Generate an epub file from the given arguments, url, & outputs it to the output directory.
/// 
/// This function DOES NOT do any error checking on the Url or output directory & WILL panic if they are wrong. 
/// Make sure the Url is valid and the output directory is writable BEFORE passing them to this.
pub fn generate_epub(epub_args: EpubArgs, book_url: Url, output_directory: PathBuf) {
    let book = book::Book::new(book_url);

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
    let cover_image = http::get_response(book.cover_image_url).get_bytes().to_vec();
    epub_builder.add_cover_image("cover.jpeg", cover_image.as_slice(), "image/jpeg").expect("Unable to add cover image.");

    // Generate the cover xhtml.
    let cover_xhtml = format!(
        r#"<head></head><body><div style="text-align: center;">
        <h1><a href="{0}">{1}</a></h1>
        <img src="cover.jpeg"/>
        <h2>by: {2}</h2>
        <h3>Archived on: {3}</h3></div></body>"#,
        book.book_url,
        book.title,
        book.author,
        chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false)
    );
    let cover_xhtml = format!("{0}{cover_xhtml}{1}", constants::EPUB_XML_HEAD, constants::EPUB_XML_TAIL);

    // Add the cover xhtml to the epub.
    epub_builder.add_content(
        EpubContent::new("title.xhtml", cover_xhtml.as_bytes())
            .title("Cover")
            .reftype(ReferenceType::Cover),
    ).expect("Unable to add cover");

    // Add a table of contents after the cover page.
    epub_builder.inline_toc();

    // Setup html2xhtml on the operating system.
    let html2xhtml_dir = file_system_crap::setup_html2xhtml();

    // TODO! Generate the epub body, deal with images etc etc. You know pickup from last night etc etc.
    // Finish setup_html2xhtml() first though dummy.

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
}

/// Generate an html archive from the given arguments, url, & outputs it to the output directory.
/// 
/// This function DOES NOT do any error checking on the Url or output directory & WILL panic if they are wrong. 
/// Make sure the Url is valid and the output directory is writable BEFORE passing them to this.
pub fn generate_html(html_args: HtmlArgs, book_url: Url, output_directory: PathBuf) {
    eprintln!("This is not implemented yet.");
}

/// Generate a markdown file from the given arguments, url, & outputs it to the output directory.
/// 
/// This function DOES NOT do any error checking on the Url or output directory & WILL panic if they are wrong. 
/// Make sure the Url is valid and the output directory is writable BEFORE passing them to this.
pub fn generate_markdown(markdown_args: MarkdownArgs, book_url: Url, output_directory: PathBuf) {
    let book = book::Book::new(book_url);

    let output_path = convert_path_to_os_specific(output_directory.join(format!("{0}.md", book.file_name_title)));

    // Create the md file. This will crash if it already exists or can not be created.
    let mut output_file = match OpenOptions::new().write(true).create_new(true).open(&output_path) {
        Ok(output_file) => output_file,
        Err(error) => {
            eprintln!("Error! Unable to create: {0}\n{error}", output_path.to_string_lossy());
            exit(1);
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
}