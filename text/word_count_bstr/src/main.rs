/*!
Trying out various ideas from BurntSushi on GitHub for counting word
frequency in Rust.

This version uses his `bstr` crate. It runs about four times slower than the
plain `word_count` example in the same folder, but performs useful work.

It handles non-valid UTF-8 better and uses full Unicode-aware algorithms both
to make a string lower case and to break it into words.

https://github.com/benhoyt/countwords/

TODO: Try ASCII-only lowercase and compare performance.
*/
use std::io;

use anyhow;
use bstr::{io::BufReadExt, BStr, BString, ByteSlice};
use rustc_hash::{FxHashMap as HashMap};


fn main() {
    // Rust blocks the broken pipe signal by default, and instead returns it as
    // an error from `write` if the consumer hangs up. So we look for it here
    // and exit gracefully, as an end user would expect.
    if let Err(err) = try_main() {
        if let Some(ioerr) = err.root_cause().downcast_ref::<io::Error>() {
            if ioerr.kind() == io::ErrorKind::BrokenPipe {
                std::process::exit(0);
            }
        }
        eprintln!("{:?}", err);
    }
}

fn try_main() -> anyhow::Result<()> {
    let stdin = io::stdin();
    let mut stdin = stdin.lock();

    let mut counts: HashMap<BString, usize> = HashMap::default();
    let mut buf = BString::from(vec![]);

    stdin.for_byte_line(|line| {
        for word in line.words() {
            // reuse the same buffer for lowercasing---an API not available
            // in std!---to avoid an alloc for every word.
            buf.clear();
            word.as_bytes().to_lowercase_into(&mut buf);
            increment(&mut counts, buf.as_bstr());
        }
        Ok(true)
    })?;

    let total: usize = counts.values().sum();
    println!("Found {} words, {} unique", total, counts.len());
    Ok(())
}

fn increment(counts: &mut HashMap<BString, usize>, word: &BStr) {
    // While this will do two hash lookups when 'word' is not in the map, it
    // will only do one lookup and no allocs in the much more common case of
    // 'word' being in the map.
    if let Some(count) = counts.get_mut(word) {
        *count += 1;
    } else {
        counts.insert(BString::from(word), 1);
    }
}
