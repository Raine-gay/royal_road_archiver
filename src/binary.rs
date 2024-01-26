use std::{env, fs, path::{Path, PathBuf}, process::exit};

use clap::{Parser, Subcommand};
use url::Url;

#[derive(clap::Parser, Debug)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    subcommand: Subcommands,

    /// Enter the URL of the Webnovel
    book_url: String,

    /// Enter the output directory for the generated format.
    /// Leave blank to use current directory.
    output_directory: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// Generate an Audiobook from the webnovel.
    /// 'audiobook --help' for available arguments.
    Audiobook(royal_road_archiver_lib::AudiobookArgs),

    /// Generate an epub from the Webnovel.
    /// 'epub --help' for available arguments.
    Epub(royal_road_archiver_lib::EpubArgs),

    /// Store the webnovel as a collection of HTML pages.
    /// 'html --help' for available arguments.
    Html(royal_road_archiver_lib::HtmlArgs),

    /// Generate a markdown file from the Webnovel.
    /// 'markdown --help' for available arguments.
    Markdown(royal_road_archiver_lib::MarkdownArgs),
}

fn main() {
    let cli_input = Cli::parse();

    // Turn the inputted string into a path, or grab the current directory if empty.
    let output_directory: PathBuf;
    match cli_input.output_directory {
        Some(output_directory_input) => {
            output_directory = PathBuf::from(&output_directory_input);
        },
        None => {
            output_directory = env::current_dir().unwrap();
        }
    }

    valid_directory_check(&output_directory);
    let book_url = valid_url_check(&cli_input.book_url.to_lowercase());

    match cli_input.subcommand {
        Subcommands::Audiobook(audiobook_args) => royal_road_archiver_lib::generate_audiobook(audiobook_args, book_url, output_directory),
        Subcommands::Epub(epub_args) => royal_road_archiver_lib::generate_epub(epub_args, book_url, output_directory),
        Subcommands::Html(html_args) => royal_road_archiver_lib::generate_html(html_args, book_url, output_directory),
        Subcommands::Markdown(markdown_args) => royal_road_archiver_lib::generate_markdown(markdown_args, book_url, output_directory),
    }
}

/// Check if the directory exists and is writeable. Creates one if not.
/// 
/// Exits the program of failure.
fn valid_directory_check(output_directory: &Path) {
    // Check if the directory exists, if it does not; attempt to create one.
    if !output_directory.exists() {
        match fs::create_dir_all(output_directory) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("Error! Unable to create directory: {error}");
                exit(1);
            }
        }
    }

    // Check if the user has write permissions:
    if fs::metadata(output_directory).unwrap().permissions().readonly() {
        eprintln!("Error! You do not have permissions for the specified directory.");
        exit(1);
    }
}

// Check if the given URL is a valid royalroad url.
fn valid_url_check(book_url: &str) -> Url {
    match Url::parse(book_url) {
        Ok(book_url) => {

            if book_url.host_str() == Some("www.royalroad.com") {
                return book_url;
            }
            else {
                eprintln!("Error! Please enter a RoyalRoad URL.");
                exit(1);
            }
        },
        Err(error) => {
            eprintln!("Error! Unable to parse url: {book_url}\n{error}");
            exit(1);
        }
    }
}
