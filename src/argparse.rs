use std::fmt::Display;
use std::ops::Range;

use regex::{Captures, Regex};

pub struct ArgParser {
    arguments: Option<Vec<Argument>>, // parser cache
    descriptions: Option<Vec<String>>,
    string: String,
}

#[derive(Debug)]
struct Argument {
    capture_str: String,
    description: Option<String>,
    is_required: bool,
    range: Range<usize>,
}

impl ArgParser {
    pub fn new(string: &str, descriptions: Option<Vec<&str>>) -> ArgParser {
        let descriptions = descriptions.map(|w| w.iter().map(|s| s.to_string()).collect());

        return ArgParser {
            arguments: None,
            descriptions,
            string: string.to_owned(),
        };
    }

    pub fn parse(&mut self) -> Result<(), failure::Error> {
        // currently supports {1}, {1?}, {1..}, {1..2}, {1..?}
        let variable = Regex::new(
            r"\{((?P<index>\d+)(?P<optional_idx>\?)?|(?P<start>\d+)\.\.((?P<end>(\d+|\?)))?)\}",
        )
        .unwrap();

        if !variable.is_match(&self.string) {
            self.arguments = Some(vec![]); // no argument(s)
            return Ok(());
        }

        let mut arguments: Vec<Argument> = vec![];
        for capture in variable.captures_iter(&self.string) {
            if let Some(_) = capture.name("index") {
                arguments.push(self.parse_single_index(capture)?);
            } else if let Some(_) = capture.name("start") {
                arguments.push(self.parse_range_index(capture)?);
            } else {
                return Err(failure::err_msg("not implemented yet (unknown pattern)"));
            }
        }

        self.arguments = Some(arguments);
        return Ok(());
    }

    fn parse_single_index(&self, captures: Captures) -> Result<Argument, failure::Error> {
        let index: usize = captures.name("index").unwrap().as_str().parse().unwrap();
        let optional: bool = match captures.name("optional_idx") {
            Some(value) => value.as_str() == "?", // always true?
            None => false,
        };
        let description = match &self.descriptions {
            Some(values) => values.get(index).map(|s| s.to_owned()),
            None => None,
        };

        return Ok(Argument {
            capture_str: captures.get(0).unwrap().as_str().to_owned(),
            description,
            is_required: !optional,
            range: (index..(index + 1)),
        });
    }

    fn parse_range_index(&self, captures: Captures) -> Result<Argument, failure::Error> {
        let start: usize = captures.name("start").unwrap().as_str().parse().unwrap();
        let end = captures
            .name("end")
            .map_or_else(|| "-1".to_owned(), |w| w.as_str().to_owned());

        if let Ok(index) = end.parse::<usize>() {
            return Ok(Argument {
                capture_str: captures.get(0).unwrap().as_str().to_owned(),
                description: None, // not supported yet
                is_required: true,
                range: (start..index),
            });
        } else if end == "?" {
            return Ok(Argument {
                capture_str: captures.get(0).unwrap().as_str().to_owned(),
                description: None, // not supported yet
                is_required: false,
                range: (start..std::usize::MAX),
            });
        } else {
            return Err(failure::err_msg(format!("invalid accessor: {}", end)));
        }
    }

    pub fn fill<T>(&mut self, variables: Vec<T>) -> Result<String, failure::Error>
    where
        T: Display,
    {
        let arguments = match &self.arguments {
            Some(values) => values,
            None => return Err(failure::err_msg("could not found parsed caches.")),
        };

        let mut replaced = self.string.to_owned();
        for argument in arguments {
            let actual = if argument.range.end == std::usize::MAX {
                variables.len()
            } else {
                argument.range.end
            };
            let params = match variables.get(argument.range.start..actual) {
                Some(values) => values
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                None => match argument.range.end == std::usize::MAX {
                    true => vec![],
                    false => return Err(failure::err_msg("index out of bounds or invalid access")),
                },
            };

            if argument.is_required && params.len() == 0 {
                return Err(failure::err_msg("argument is empty"));
            }

            replaced = replaced.replace(&argument.capture_str, params.join(" ").as_str());
        }

        return Ok(replaced.to_owned());
    }
}
