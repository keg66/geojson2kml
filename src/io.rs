use crate::error::{GeojsonError, Result};
use crate::Geo;
use std::fs;
use std::io::{self, Write};

pub trait InputReader {
    fn read_line(&mut self) -> Result<String>;
}

pub trait OutputWriter {
    fn write_line(&mut self, msg: &str) -> Result<()>;
    fn write_error(&mut self, msg: &str) -> Result<()>;
}

pub struct StdinReader;

impl InputReader for StdinReader {
    fn read_line(&mut self) -> Result<String> {
        let mut buf = String::new();
        io::stdin()
            .read_line(&mut buf)
            .map_err(GeojsonError::IoError)?;
        Ok(buf.trim().to_string())
    }
}

pub struct StdoutWriter;

impl OutputWriter for StdoutWriter {
    fn write_line(&mut self, msg: &str) -> Result<()> {
        println!("{}", msg);
        Ok(())
    }

    fn write_error(&mut self, msg: &str) -> Result<()> {
        eprintln!("{}", msg);
        Ok(())
    }
}

pub fn load_geojson(file_path: &str) -> Result<Geo> {
    let content = fs::read_to_string(file_path)?;
    let geo: Geo = serde_json::from_str(&content)?;
    Ok(geo)
}

pub fn write_kml_file(filename: &str, content: &str) -> Result<()> {
    let mut file = fs::File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

pub struct MockInputReader {
    inputs: Vec<String>,
    current: usize,
}

impl MockInputReader {
    pub fn new(inputs: Vec<String>) -> Self {
        Self { inputs, current: 0 }
    }
}

impl InputReader for MockInputReader {
    fn read_line(&mut self) -> Result<String> {
        if self.current < self.inputs.len() {
            let input = self.inputs[self.current].clone();
            self.current += 1;
            Ok(input)
        } else {
            Err(GeojsonError::InvalidInput("No more input available".to_string()))
        }
    }
}

pub struct MockOutputWriter {
    pub lines: Vec<String>,
    pub errors: Vec<String>,
}

impl MockOutputWriter {
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            errors: Vec::new(),
        }
    }
}

impl OutputWriter for MockOutputWriter {
    fn write_line(&mut self, msg: &str) -> Result<()> {
        self.lines.push(msg.to_string());
        Ok(())
    }

    fn write_error(&mut self, msg: &str) -> Result<()> {
        self.errors.push(msg.to_string());
        Ok(())
    }
}