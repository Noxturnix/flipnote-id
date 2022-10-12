use clap::Parser;
use cli::{Actions, Cli};

use file::{handle_file_create_error, handle_file_read_error, handle_flipnote_id_error};
use flipnote_id::{compute_checksum, extract_id_with_checksum, set_fsid};

use std::{
    fs::{copy, read, File},
    io::Write,
    process::exit,
};

mod cli;
mod file;

fn main() {
    let cli = Cli::parse();

    match &cli.action {
        Actions::Set => match read(&cli.file) {
            Ok(bin_data) => match set_fsid(&bin_data, &cli.fsid.unwrap()) {
                Ok(new_data) => {
                    if !cli.no_backup {
                        if let Err(_) = copy(&cli.file, format!("{}.bak", &cli.file)) {
                            eprintln!("Error: Can't make a backup file");
                            exit(1);
                        }
                    }

                    match File::create(&cli.file) {
                        Ok(mut fp) => match fp.write_all(&new_data) {
                            Ok(_) => println!(
                                "Successfully set FSID to {:016X}",
                                extract_id_with_checksum(&new_data).unwrap().id
                            ),
                            Err(_) => {
                                eprintln!("Error: Can't write to file");
                                exit(1);
                            }
                        },
                        Err(err) => handle_file_create_error(err),
                    }
                }
                Err(_) => {
                    eprintln!("Error: Invalid FSID");
                    exit(1);
                }
            },
            Err(err) => handle_file_read_error(err, &cli.file),
        },
        Actions::Extract => match read(&cli.file) {
            Ok(bin_data) => match extract_id_with_checksum(&bin_data) {
                Ok(flipnote_id) => println!("{:016X}", flipnote_id.id),
                Err(err) => handle_flipnote_id_error(err),
            },
            Err(err) => handle_file_read_error(err, &cli.file),
        },
        Actions::Check => match read(&cli.file) {
            Ok(bin_data) => match extract_id_with_checksum(&bin_data) {
                Ok(flipnote_id) => {
                    let valid_checksum = compute_checksum(&bin_data).unwrap();

                    println!("Flipnote Studio ID: {:016X}", flipnote_id.id);
                    println!(
                        "Checksum: {:04X} ({})",
                        flipnote_id.checksum,
                        if flipnote_id.checksum == valid_checksum {
                            String::from("valid")
                        } else {
                            format!("invalid; expect {:04X}", valid_checksum)
                        }
                    );
                }
                Err(err) => handle_flipnote_id_error(err),
            },
            Err(err) => handle_file_read_error(err, &cli.file),
        },
    }
}
