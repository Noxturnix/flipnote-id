use std::io::Error;
use std::io::ErrorKind;
use std::process::exit;

use flipnote_id::FlipnoteDataError;

pub fn handle_file_read_error(err: Error, file_path: &String) {
    match err.kind() {
        ErrorKind::NotFound => eprintln!("Error: File `{}` not found", file_path),
        _ => eprintln!("Error: Can't read file"),
    }
    exit(1);
}

pub fn handle_flipnote_id_error(err: FlipnoteDataError) {
    match err {
        FlipnoteDataError::InvalidSize => eprintln!("Error: Invalid file size"),
    }
    exit(1);
}
