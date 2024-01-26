use std::path::PathBuf;

use path_slash::PathBufExt as _;

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
pub fn setup_html2xhtml() {
    #[cfg(target_os = "windows")] {
        //TODO!
        // Thinking of using C:\Users\<username>\AppData\Local\Temp\html2xhtml-windows
    }

    #[cfg(target_os = "linux")] {
        // TODO!
        // Thinking of using /tmp/html2xhtml-linux
    }

    #[cfg(target_os = "macos")] {
        // TODO!
        // You can find the macos tempdir by doing: echo $TMPDIR
    }
}

/// Delete html2xhtml from the operating system's temp directory.
pub fn delete_html2xhtml() {
    // TODO!
}