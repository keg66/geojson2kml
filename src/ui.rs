use crate::error::{GeojsonError, Result};
use crate::io::{InputReader, OutputWriter};
use crate::{Geo, TrainLine, generate_kml_body, generate_filename, search_candidates};
use std::collections::BTreeSet;

const PROMPT_SEPARATOR: &str = "==================================";
const PROMPT_QUERY: &str = "enter train company name or line name or 'q' to exit:";
const PROMPT_QUIT: &str = "q";
const PROMPT_MERGE: &str = "merge lines? [yes/no]:";
const RESPONSE_YES: &str = "yes";

pub struct InteractiveSession<R: InputReader, W: OutputWriter> {
    input: R,
    output: W,
}

impl<R: InputReader, W: OutputWriter> InteractiveSession<R, W> {
    pub fn new(input: R, output: W) -> Self {
        Self { input, output }
    }

    pub fn run(&mut self, geo: &Geo) -> Result<()> {
        loop {
            self.output.write_line(PROMPT_SEPARATOR)?;
            self.output.write_line(PROMPT_QUERY)?;
            
            let query = self.input.read_line()?;
            if query == PROMPT_QUIT {
                break;
            }

            match self.process_query(&query, geo) {
                Ok(_) => {},
                Err(GeojsonError::UserCancelled) => continue,
                Err(GeojsonError::NoMatchFound(query)) => {
                    self.output.write_error(&format!("{} is not found...", query))?;
                    continue;
                },
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }

    fn process_query(&mut self, query: &str, geo: &Geo) -> Result<()> {
        let candidates: Vec<TrainLine> = search_candidates(query, geo).into_iter().collect();
        
        if candidates.is_empty() {
            return Err(GeojsonError::NoMatchFound(query.to_string()));
        }

        let chosen_ids = self.choose_candidates(&candidates)?;
        if chosen_ids.is_empty() {
            return Err(GeojsonError::UserCancelled);
        }

        let should_merge = if chosen_ids.len() > 1 {
            self.ask_merge_preference()?
        } else {
            false
        };

        if should_merge {
            let train_lines: Vec<&TrainLine> = chosen_ids.iter()
                .map(|&id| &candidates[id])
                .collect();
            self.generate_and_save_kml(train_lines, geo)?;
        } else {
            for &id in &chosen_ids {
                self.generate_and_save_kml(vec![&candidates[id]], geo)?;
            }
        }

        Ok(())
    }

    fn choose_candidates(&mut self, candidates: &[TrainLine]) -> Result<BTreeSet<usize>> {
        if candidates.len() == 1 {
            let mut ids = BTreeSet::new();
            ids.insert(0);
            return Ok(ids);
        }

        self.display_candidates(candidates)?;

        loop {
            self.output.write_line(&format!(
                "choose '{}'...'{}' or : 'q' to exit", 
                0, 
                candidates.len() - 1
            ))?;
            
            let input = self.input.read_line()?;
            if input == PROMPT_QUIT {
                return Ok(BTreeSet::new());
            }

            match self.parse_selection(&input, candidates.len()) {
                Ok(ids) if !ids.is_empty() => return Ok(ids),
                Ok(_) => continue, // Empty selection, try again
                Err(e) => {
                    self.output.write_error(&e.to_string())?;
                    continue;
                }
            }
        }
    }

    fn display_candidates(&mut self, candidates: &[TrainLine]) -> Result<()> {
        self.output.write_line("candidates:")?;
        for (id, candidate) in candidates.iter().enumerate() {
            self.output.write_line(&format!(
                "[{}] {} {}",
                id, candidate.company_name, candidate.line_name
            ))?;
        }
        Ok(())
    }

    fn parse_selection(&self, input: &str, max_id: usize) -> Result<BTreeSet<usize>> {
        let mut ids = BTreeSet::new();
        let parts: Vec<&str> = input.split(',').collect();

        for part in parts {
            let trimmed = part.trim();
            let id: usize = trimmed.parse()
                .map_err(|_| GeojsonError::InvalidInput(format!("{} is invalid", trimmed)))?;
            
            if id >= max_id {
                return Err(GeojsonError::InvalidInput(format!("{} is invalid", trimmed)));
            }
            
            ids.insert(id);
        }

        Ok(ids)
    }

    fn ask_merge_preference(&mut self) -> Result<bool> {
        self.output.write_line(PROMPT_MERGE)?;
        let answer = self.input.read_line()?;
        Ok(answer == RESPONSE_YES)
    }

    fn generate_and_save_kml(&mut self, train_lines: Vec<&TrainLine>, geo: &Geo) -> Result<()> {
        let filename = generate_filename(&train_lines);
        let filename_with_ext = format!("{}.kml", filename);

        self.output.write_line(&format!("creating {} ...", filename_with_ext))?;

        let kml_content = self.build_kml_content(&filename, &train_lines, geo);
        crate::io::write_kml_file(&filename_with_ext, &kml_content)?;

        self.output.write_line("succeeded!!")?;
        Ok(())
    }

    fn build_kml_content(&self, filename: &str, train_lines: &[&TrainLine], geo: &Geo) -> String {
        let mut content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<kml xmlns="http://www.opengis.net/kml/2.2">
<Document>
<name>{}</name>"#,
            filename
        );

        for train_line in train_lines {
            content.push_str(&generate_kml_body(train_line, geo));
        }

        content.push_str(
            r#"
</Document>
</kml>
"#
        );

        content
    }
}