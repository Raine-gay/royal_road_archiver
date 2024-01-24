use std::path::PathBuf;

use clap::Args;
use url::Url;

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
    eprintln!("This is not implemented yet.");
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
    eprintln!("This is not implemented yet.");
}