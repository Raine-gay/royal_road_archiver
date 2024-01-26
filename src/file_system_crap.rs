use std::{io::Cursor, path::PathBuf, process::exit};

use path_slash::PathBufExt as _;
use tempdir::TempDir;

/// Converts a given path to windows style if needed.
pub fn convert_path_to_os_specific(path: PathBuf) -> PathBuf {
    // If target os is windows.
    #[cfg(target_os = "windows")] {
        return PathBuf::from_slash_lossy(path.into_os_string());
    }

    // If target os is not windows.
    #[cfg(not(target_os = "windows"))] {
        return PathBuf::from_backslash_lossy(path.into_os_string());
    }
}

/// Remove chars that are illegal to be used in filenames on both unix & windows.
pub fn remove_illegal_chars(mut string: String) -> String {
    const ILLEGAL_CHARS: [char; 9] = ['/', '\u{005C}', '<', '>', ':', '\u{0022}', '|', '?', '*'];

    for char in ILLEGAL_CHARS {
        string = string.replace(char, " ");
    }

    return string;
}

/// Setup html2xhtml in the operating system's temp directory.
pub fn setup_html2xhtml() -> TempDir {
    #[cfg(target_os = "linux")] {
        const HTML2XHTML: &[u8; 245025] = include_bytes!("../html2xhtml-windows.zip"); // This will not compile on windows due to this and no I don't give a shit.
                                                                                       // Compile it on linux for windows like a sane person.
        let html2xhtml_dir = match TempDir::new("html2xhtml-windows") {
            Ok(temp_dir) => temp_dir,
            Err(error) => {
                eprintln!("Error! Unable to create temp directory: {error}");
                exit(1);
            }
        };

        match zip_extract::extract(Cursor::new(HTML2XHTML), html2xhtml_dir.path(), true) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("Error! Unable to extract html2xhtml into into the temp directory\n{error}");
                exit(1);
            }
        }

        return html2xhtml_dir;
    }

    #[cfg(target_os = "linux")] {
        const HTML2XHTML: &[u8; 186938] = include_bytes!("../html2xhtml-linux.zip");
        let html2xhtml_dir = match TempDir::new("html2xhtml-linux") {
            Ok(temp_dir) => temp_dir,
            Err(error) => {
                eprintln!("Error! Unable to create temp directory: {error}");
                exit(1);
            }
        };

        match zip_extract::extract(Cursor::new(HTML2XHTML), html2xhtml_dir.path(), true) {
            Ok(_) => (),
            Err(error) => {
                eprintln!("Error! Unable to extract html2xhtml into the temp directory\n{error}");
                exit(1);
            }
        }

        return html2xhtml_dir;
    }

    #[cfg(target_os = "macos")] {
        // TODO!
        // You can find the macos tempdir by doing: echo $TMPDIR

        eprint!("Error! This mode does not currently support MacOS. Try either html mode or markdown mode.");
        exit(1);
    }
}

/// Delete html2xhtml from the operating system's temp directory.
pub fn delete_html2xhtml(html2xhtml_dir: TempDir) {
    match html2xhtml_dir.close() {
        Ok(_) => (),
        Err(warning) => {
            eprintln!("Warning! Unable to close & delete temp directory: {warning}");
        }
    }
}