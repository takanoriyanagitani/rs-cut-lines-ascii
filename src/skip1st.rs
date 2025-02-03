use std::io;

use std::io::BufRead;

use std::io::BufWriter;
use std::io::Write;

pub fn create_skip_1st_bytes(num_bytes2skip: usize) -> impl Fn(&[u8]) -> &[u8] {
    move |input: &[u8]| {
        let sz: usize = input.len().min(num_bytes2skip);
        &input[sz..]
    }
}

pub fn lines2skipped2writer<I, S, W>(lines: I, skip: S, mut wtr: W) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Vec<u8>, io::Error>>,
    S: Fn(&[u8]) -> &[u8],
    W: Write,
{
    for rline in lines {
        let line: Vec<u8> = rline?;
        let skipped: &[u8] = skip(&line);
        wtr.write_all(skipped)?;
        writeln!(wtr)?;
    }
    wtr.flush()
}

pub fn reader2skipped2writer<R, S, W>(rdr: R, skip: S, wtr: W) -> Result<(), io::Error>
where
    R: BufRead,
    S: Fn(&[u8]) -> &[u8],
    W: Write,
{
    let lines = rdr.split(b'\n');
    lines2skipped2writer(lines, skip, wtr)
}

pub fn stdin2skipped2stdout<S>(skip: S) -> Result<(), io::Error>
where
    S: Fn(&[u8]) -> &[u8],
{
    let i = io::stdin();
    let il = i.lock();

    let o = io::stdout();
    let mut ol = o.lock();

    let bw = BufWriter::new(&mut ol);
    reader2skipped2writer(il, skip, bw)?;

    ol.flush()
}
