use std::process::ExitCode;

use std::io;

use std::collections::BTreeSet;

use std::fs::File;

use rs_find_missing::reader2short;
use rs_find_missing::stdin2lines2filtered2stdout;

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(|e| io::Error::other(format!("env var {key} missing: {e}")))
}

fn shorter_filename() -> Result<String, io::Error> {
    env_val_by_key("ENV_SHORTER_TEXT_FILENAME")
}

fn shorter_file_size_limit() -> Result<u64, io::Error> {
    let s: String = env_val_by_key("ENV_SHORTER_TEXT_FILE_SIZE_LIMIT")?;
    str::parse(s.as_str()).map_err(io::Error::other)
}

fn shorter_file() -> Result<File, io::Error> {
    File::open(shorter_filename()?)
}

fn shorter() -> Result<BTreeSet<Vec<u8>>, io::Error> {
    Ok(reader2short(
        shorter_file()?,
        shorter_file_size_limit().unwrap_or(1048576),
    ))
}

fn stdin2stdout() -> Result<(), io::Error> {
    stdin2lines2filtered2stdout(&shorter()?)
}

fn main() -> ExitCode {
    stdin2stdout()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
