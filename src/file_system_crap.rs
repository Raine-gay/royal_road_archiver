use std::{io::Cursor, path::PathBuf};

use path_slash::PathBufExt as _;
use tempdir::TempDir;

use crate::{misc, GenerationError, Warning, WARNINGS};

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
pub fn setup_html2xhtml() -> Result<TempDir, GenerationError> {
    #[cfg(target_os = "windows")] {
        const HTML2XHTML: &[u8; 245025] = include_bytes!("../html2xhtml-windows.zip"); // This will not compile on windows due to this and no I don't give a shit.
                                                                                       // Compile it on linux for windows like a sane person.
        let html2xhtml_temp_dir = match TempDir::new("html2xhtml-windows") {
            Ok(temp_dir) => temp_dir,
            Err(error) => return Err(GenerationError::TempDirCreationError {error}),
        };

        match zip_extract::extract(Cursor::new(HTML2XHTML), html2xhtml_temp_dir.path(), true) {
            Ok(_) => (),
            Err(error) => return Err(GenerationError::Html2XhtmlExtractionError {error}),
        }

        return Ok(html2xhtml_temp_dir);
    }

    #[cfg(target_os = "linux")] {
        const HTML2XHTML: &[u8; 186938] = include_bytes!("../html2xhtml-linux.zip");
        let html2xhtml_temp_dir = match TempDir::new("html2xhtml-linux") {
            Ok(temp_dir) => temp_dir,
            Err(error) => return Err(GenerationError::TempDirCreationError {error}),
        };

        match zip_extract::extract(Cursor::new(HTML2XHTML), html2xhtml_temp_dir.path(), true) {
            Ok(_) => (),
            Err(error) => return Err(GenerationError::Html2XhtmlExtractionError {error}),
        }

        return Ok(html2xhtml_temp_dir);
    }

    #[cfg(target_os = "macos")] {
        Err(GenerationError::OsUnsupportedError {os: misc::Oses::MacOs})
    }
    
    // In the event the OS is unknown.
    #[allow(unreachable_code)]
    Err(GenerationError::OsUnsupportedError {os: misc::Oses::OtherUnknownOs})
}

/// Delete html2xhtml from the operating system's temp directory.
pub fn delete_html2xhtml(html2xhtml_dir: TempDir) {
    let temp_dir_path = html2xhtml_dir.path().to_path_buf();

    match html2xhtml_dir.close() {
        Ok(_) => (),
        Err(warning) => {
            let warning = Warning::TempDirDeletionError { 
                warning_msg: "Unable to close and delete temp directory".to_string(), 
                temp_directory_path: temp_dir_path, 
                error: warning 
            };
            WARNINGS.lock().unwrap().add_warning(warning);
        }
    }
}