#![allow(unused)]
use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, Write},
};

const TEST_FILE_INIT_LINE_TOTAL: usize = 10;

pub struct TestFile {
    pub file: File,
    pub path: String,
    pub lines: Vec<String>,
}

impl TestFile {
    pub fn new() -> io::Result<Self> {
        let path = "./TestVmtranslator.vm";
        let lines = TEST_FILE_INIT_LINE_TOTAL;
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)?;

        for i in 0..lines {
            file.write_all(format!("{i}\n").as_bytes())?
        }

        let mut f = Self {
            file,
            path: path.to_string(),
            lines: vec![],
        };
        f.read_to_lines()?;
        Ok(f)
    }

    pub fn clear(&mut self) -> io::Result<()> {
        self.file.seek(io::SeekFrom::Start(0))?;
        self.file.set_len(0)?;
        self.read_to_lines()?;
        Ok(())
    }

    pub fn add_line(&mut self, newline: &str) -> io::Result<()> {
        self.file.write_all(format!("{newline}\n").as_bytes())?;
        self.read_to_lines()?;

        Ok(())
    }

    // privates
    fn read_to_lines(&mut self) -> io::Result<()> {
        let mut contents = String::new();
        self.file.seek(io::SeekFrom::Start(0))?;
        self.file.read_to_string(&mut contents)?;
        let lines = contents.lines().map(|s| s.to_string()).collect::<Vec<_>>();
        self.lines = lines;
        self.file.seek(io::SeekFrom::End(0))?;
        Ok(())
    }
}

impl Drop for TestFile {
    fn drop(&mut self) {
        fs::remove_file(&mut self.path)
            .expect(format!("remove test file `{}` fail...", &self.path).as_str());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() -> io::Result<()> {
        let f = TestFile::new()?;
        assert_eq!(f.lines.len(), TEST_FILE_INIT_LINE_TOTAL);
        Ok(())
    }

    #[test]
    fn test_clear() -> io::Result<()> {
        let mut f = TestFile::new()?;
        f.clear()?;
        assert_eq!(f.lines.len(), 0);

        Ok(())
    }

    #[test]
    fn test_add_line() -> io::Result<()> {
        let mut f = TestFile::new()?;
        let newline = "new line";
        f.add_line(newline)?;
        assert_eq!(f.lines.len(), TEST_FILE_INIT_LINE_TOTAL + 1);
        assert_eq!(f.lines[TEST_FILE_INIT_LINE_TOTAL], newline);
        Ok(())
    }
}
