use std::io;

use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;

use std::io::BufWriter;
use std::io::Write;

use std::collections::BTreeSet;

pub fn filter_missing<I, O>(longer: I, shorter: &BTreeSet<O>) -> impl Iterator<Item = O>
where
    I: Iterator<Item = O>,
    O: Ord,
{
    longer.filter(|o: &O| !shorter.contains(o))
}

pub fn reader2lines2filtered2writer<R, W>(
    rdr: R,
    mut wtr: W,
    shorter: &BTreeSet<Vec<u8>>,
) -> Result<(), io::Error>
where
    R: BufRead,
    W: Write,
{
    let rlines = rdr.split(b'\n');
    let lines = rlines.filter_map(Result::ok);
    let filtered = filter_missing(lines, shorter);

    let mut bw = BufWriter::new(&mut wtr);

    for line in filtered {
        bw.write_all(&line)?;
        writeln!(&mut bw)?;
    }

    bw.flush()?;
    drop(bw);

    wtr.flush()
}

pub fn stdin2lines2filtered2stdout(shorter: &BTreeSet<Vec<u8>>) -> Result<(), io::Error> {
    let i = io::stdin();
    let il = i.lock();

    let o = io::stdout();
    reader2lines2filtered2writer(il, o, shorter)
}

pub fn reader2short<R>(rdr: R, limit: u64) -> BTreeSet<Vec<u8>>
where
    R: Read,
{
    let taken = rdr.take(limit);
    let br = BufReader::new(taken);
    let rlines = br.split(b'\n');
    let lines = rlines.filter_map(Result::ok);
    BTreeSet::from_iter(lines)
}
