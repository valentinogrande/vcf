use std::collections::HashSet;
use std::io::BufRead;
use std::io::Seek;
use std::{fs::File, io::BufReader, io::BufWriter, io::Write};

#[derive(PartialEq, Hash, Eq, Clone)]
struct Node {
    word: String,
}

impl Node {
    pub fn new(word: String) -> Self {
        Self { word }
    }

    pub fn expand<'a>(&self, front: &'a mut HashSet<Node>) -> Option<&'a Node> {
        let mut best: Option<&Node> = None;
        let mut value = -1;

        front.remove(self);

        for node in front.iter() {
            let coincidence = self.calculate_value(node);
            if coincidence > value {
                value = coincidence;
                best = Some(node);
            }
        }
        best
    }

    pub fn calculate_value(&self, node: &Node) -> i16 {
        // `segment` is always a contiguous slice of `self.word`, so we track it
        // with byte indices and borrow it directly instead of allocating a
        // growing String on every char. Semantics are identical to pushing onto
        // a String and resetting it to the current char on a miss.
        let mut count: i16 = 0;
        let mut seg_start = 0; // byte index where the current segment begins
        for (idx, char) in self.word.char_indices() {
            let end = idx + char.len_utf8(); // segment = self.word[seg_start..end]
            let len = end - seg_start;
            if node.word.contains(&self.word[seg_start..end]) {
                if len > 1 {
                    count -= ((len - 1) * (len - 1)) as i16;
                    count += (len * len) as i16;
                } else {
                    count += 1;
                }
            } else {
                seg_start = idx; // reset segment to the current char
            }
        }
        count
    }
}

// reads the next line from the reader and turns it into a Node.
// returns None at EOF.
fn next_node(reader: &mut impl BufRead) -> std::io::Result<Option<Node>> {
    let mut line = String::new();
    if reader.read_line(&mut line)? == 0 {
        return Ok(None); // EOF
    }
    let word = line.trim_end().to_string();
    Ok(Some(Node::new(word)))
}

// renders a single-line progress bar to stderr.
fn print_progress(processed: usize, pos: u64, total: u64) {
    let frac = if total == 0 {
        0.0
    } else {
        (pos as f64 / total as f64).min(1.0)
    };
    let width = 30;
    let filled = (frac * width as f64) as usize;
    let bar: String = "=".repeat(filled) + &" ".repeat(width - filled);
    eprint!("\r[{}] {:>5.1}% ({} words)", bar, frac * 100.0, processed);
    let _ = std::io::stderr().flush();
}

// .txt file where order does not matter
fn main() -> std::io::Result<()> {
    let file = File::open("rockyou.txt")?;
    let total_bytes = file.metadata()?.len();
    let mut reader = BufReader::new(file);

    const WINDOW: usize = 512; // frontier (buffer) size

    let mut fronteir: HashSet<Node> = HashSet::with_capacity(WINDOW);

    // initial fill of the frontier
    for _ in 0..WINDOW {
        match next_node(&mut reader)? {
            Some(node) => {
                fronteir.insert(node);
            }
            None => break,
        }
    }

    let mut vcf = BufWriter::new(File::create("rockyou.vcf")?);

    vcf.write_all("VIG Compressed File\n".as_bytes())?;

    let mut node = Node::new("admin".to_string());
    let mut processed: usize = 0;

    // keep expanding while the frontier has candidates; after each
    // expansion we pull one more line from the buffer to refill it.
    while let Some(best) = node.expand(&mut fronteir) {
        node = best.clone();
        writeln!(vcf, "{}", node.word)?;

        // add a node from one more line of the current buffer
        if let Some(next) = next_node(&mut reader)? {
            fronteir.insert(next);
        }

        processed += 1;
        // refresh the bar every so often to avoid flushing on every word
        if processed.is_multiple_of(512) {
            print_progress(processed, reader.stream_position()?, total_bytes);
        }
    }

    print_progress(processed, total_bytes, total_bytes);
    eprintln!();

    vcf.flush()?;

    Ok(())
}
