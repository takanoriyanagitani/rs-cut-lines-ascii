use std::process::ExitCode;

use std::io;

use rs_cut_lines_ascii::NUM_BYTES_TO_SKIP_DEFAULT;

use rs_cut_lines_ascii::skip1st::create_skip_1st_bytes;
use rs_cut_lines_ascii::skip1st::stdin2skipped2stdout;

fn env_val_by_key(key: &'static str) -> Result<String, io::Error> {
    std::env::var(key).map_err(io::Error::other)
}

fn num_bytes2skip() -> Result<usize, io::Error> {
    env_val_by_key("ENV_NUM_BYTES_TO_SKIP")
        .and_then(|s| str::parse(s.as_str()).map_err(io::Error::other))
}

fn skip1st() -> impl Fn(&[u8]) -> &[u8] {
    create_skip_1st_bytes(num_bytes2skip().unwrap_or_else(|_| {
        eprintln!("no ENV_NUM_BYTES_TO_SKIP specified. using default...");
        NUM_BYTES_TO_SKIP_DEFAULT
    }))
}

fn stdin2stdout() -> Result<(), io::Error> {
    stdin2skipped2stdout(skip1st())
}

fn main() -> ExitCode {
    stdin2stdout()
        .map(|_| ExitCode::SUCCESS)
        .unwrap_or_else(|e| {
            eprintln!("{e}");
            ExitCode::FAILURE
        })
}
