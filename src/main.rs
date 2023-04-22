use std::fs;
use std::fmt;
use std::env;

#[derive(Debug)]
enum ByteSearch {
    Byte(u8),
    WildCard
}

impl fmt::Display for ByteSearch {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ByteSearch::Byte(v) => write!(f, "{:02x}", v),
            ByteSearch::WildCard => write!(f, "??"),
        }
    }
}

#[derive(Debug)]
struct BytePattern(Vec<ByteSearch>);

impl BytePattern {

    /// search a byte pattern for the pattern
    fn match_pattern(&self, bytes: &[u8]) -> bool {

        if self.0.len() != bytes.len() {
            panic!("fatal: byte sequence length != pattern length");
        }

        for i in 0..self.0.len() {
            match self.0[i] {
                ByteSearch::Byte(v) => if v != bytes[i] {
                    return false;
                },
                ByteSearch::WildCard => (),
            }
        }
        true
    }

    /// return all positions where pattern was found
    pub fn find(&self, haystack: &[u8]) -> Option<Vec<usize>> {

        let mut offsets = vec![];

        if self.0.len() > haystack.len() {
            eprintln!("can't search for {} byte pattern in {} byte haystack",
                self.0.len(), haystack.len());
            return None;
        }

        for i in 0..=haystack.len() - self.0.len() {
            if self.match_pattern(&haystack[i..i+self.0.len()]) {
                offsets.push(i);
            }
        }

        if offsets.len() != 0 {
            return Some(offsets);
        }

        None
    }

}

impl fmt::Display for BytePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x?}", self.0)
    }
}

impl From<&str> for BytePattern {

    fn from(value: &str) -> Self {
        // expects values in form: 0a ? 10 20 30 ?
        // space is always separator
        BytePattern {
            0 : value
                .split(" ")
                .map(|x| match x {
                    "?" => ByteSearch::WildCard,
                    _ => {
                        match u8::from_str_radix(x, 16) {
                            Ok(v) => ByteSearch::Byte(v),
                            Err(e) => {
                                eprintln!("pattern has invalid byte: {x} error: {e}");
                                std::process::exit(1);
                            }
                        } // match inner
                    } // guard
                })
                .collect()
        }
    }

} // from &str

struct Pattern {
    name: String,
    patterns: Vec<BytePattern>,
}

impl Pattern {

    pub fn from_file(path: &str) -> Self {
        let data = fs::read_to_string(path)
            .expect("unable to read file");
        Pattern {
            name: path.to_owned(),
            patterns: data
                        .split("\n")
                        .filter(|s| !s.is_empty())
                        .map(|s| BytePattern::from(s))
                        .collect()
        }
    }

    pub fn search(&self, haystack: &[u8]) {
        println!("[*] searching for pattern {}", self.name);
        for pattern in &self.patterns {
            let res = pattern.find(haystack);
            if res.is_some() {
                println!("[FOUND] pattern {} ({}) found at offsets {:x?}", self.name, pattern, res.unwrap());
            } 
        }
    }

}

fn main() -> std::io::Result<()> {
    
    let mut patterns = vec![];

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("usage: ./psearch [pattern directory] [file-to-search]");
        println!("       ./psearch -p \"10 20 30 ?\" [file-to-search]");
        return Ok(());
    }

    if args[1] == "-p" {
        if args.len() != 4 {
            println!("usage: ./psearch -p \"10 20 30 ?\" [file-to-search]");
        }
        let pat = BytePattern::from(args[2].as_str());
        let haystack = fs::read(&args[3])?;
        match pat.find(&haystack) {
            Some(v) => {
                println!("[FOUND] pattern found at offsets {:x?}", v);
            },
            None => println!("[!] pattern not found"),
        }
    } else {
        println!("[+] reading patterns from directory {}", args[1]);

        // Get all files in target directory.
        // Replace "." with a more useful directory if needed.
        for entry in fs::read_dir(&args[1])? {
            let path = entry?.path();
            // Get path string.
            let path_str = path.to_str().unwrap();
            println!("[+] reading pattern from file {}", path_str);
            patterns.push(Pattern::from_file(path_str));
        }

        let haystack = fs::read(&args[2])?;

        println!("\n\n[+] starting search...\n\n");
        for pattern in patterns {
            pattern.search(&haystack);
        }
    }

    Ok(())
}
