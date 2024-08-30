use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub trait InputSource {
    fn reader(&self) -> Box<dyn BufRead>;
    fn filename(&self) -> &str;
}

pub struct FileInputSource {
    pub filename: String,
}

impl InputSource for FileInputSource {
    fn reader(&self) -> Box<dyn BufRead> {
        if self.filename == "-" {
            Box::new(io::stdin().lock())
        } else {
            Box::new(BufReader::new(File::open(&self.filename).unwrap()))
        }
    }

    fn filename(&self) -> &str {
        &self.filename
    }
}

#[allow(dead_code)]
pub struct TestInputSource {
    pub lines: Vec<String>,
}

impl InputSource for TestInputSource {
    fn reader(&self) -> Box<dyn BufRead> {
        Box::new(io::Cursor::new(self.lines.join("\n")))
    }

    fn filename(&self) -> &str {
        "test"
    }
}
