use std::{fs::OpenOptions, io::Write, path::PathBuf, process::exit};

use chrono::prelude::Local;
use clap::Args;
use url::Url;


mod book;
mod html;
mod http;

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

    let output_path = convert_path_to_windows(output_directory.join(format!("{0}.md", book.title)));

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
            buf = format!("\n\n{}\n\n", html2md::parse_html(&html::remove_image_tags(chapter.isolated_chapter_html)));

        } else {
            buf = format!("\n\n{}\n\n", html2md::parse_html(&chapter.isolated_chapter_html.html()));
        }

        output_file.write_all(buf.as_bytes()).unwrap();
    }
}

/// Converts a given path to windows style if needed.
fn convert_path_to_windows(path: PathBuf) -> PathBuf {
    // If target os is windows.
    #[cfg(target_os = "windows")] {
        use path_slash::PathBufExt as _;

        return PathBuf::from_slash(path.into_os_string().into_string().unwrap());
    }

    // If target os is not windows.
    #[cfg(not(target_os = "windows"))] {
        return path;
    }
}