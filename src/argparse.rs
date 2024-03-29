use std::fmt::Display;
use std::ops::Range;

use itertools::Itertools;
use regex::{Captures, Regex};

pub struct ArgParser {
    arguments: Option<Vec<Argument>>, // parser cache
    descriptions: Option<Vec<String>>,
    string: String,
}

#[derive(Debug)]
pub struct Argument {
    capture_str: String,
    description: Option<String>,
    is_required: bool,
    range: Range<usize>,
}

impl ArgParser {
    pub fn new(string: &str, descriptions: Option<Vec<&str>>) -> ArgParser {
        let descriptions = descriptions.map(|w| w.iter().map(|s| s.to_string()).collect());

        ArgParser {
            arguments: None,
            descriptions,
            string: string.to_owned(),
        }
    }

    // accessors
    pub fn arguments(&mut self) -> Option<&Vec<Argument>> {
        self.arguments.as_ref()
    }

    pub fn has_arguments(&self) -> Result<bool, failure::Error> {
        match &self.arguments {
            Some(value) => Ok(!value.is_empty()),
            None => Err(failure::err_msg("could not found parsed caches")),
        }
    }

    // methods
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
            if capture.name("index").is_some() {
                arguments.push(self.parse_single_index(capture)?);
            } else if capture.name("start").is_some() {
                arguments.push(self.parse_range_index(capture)?);
            } else {
                return Err(failure::err_msg("not implemented yet (unknown pattern)"));
            }
        }

        arguments.sort_by_key(|w| w.range.start);
        self.arguments = Some(
            arguments
                .into_iter()
                .unique_by(|w| w.capture_str.to_owned())
                .collect(),
        );
        Ok(())
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

        Ok(Argument {
            capture_str: captures.get(0).unwrap().as_str().to_owned(),
            description,
            is_required: !optional,
            #[allow(clippy::range_plus_one)]
            range: (index..(index + 1)),
        })
    }

    fn parse_range_index(&self, captures: Captures) -> Result<Argument, failure::Error> {
        let start: usize = captures.name("start").unwrap().as_str().parse().unwrap();
        let end = captures
            .name("end")
            .map_or_else(|| "-1".to_owned(), |w| w.as_str().to_owned());

        if let Ok(index) = end.parse::<usize>() {
            Ok(Argument {
                capture_str: captures.get(0).unwrap().as_str().to_owned(),
                description: None, // not supported yet
                is_required: true,
                range: (start..index),
            })
        } else if end == "?" {
            Ok(Argument {
                capture_str: captures.get(0).unwrap().as_str().to_owned(),
                description: None, // not supported yet
                is_required: false,
                range: (start..std::usize::MAX),
            })
        } else if end == "-1" {
            Ok(Argument {
                capture_str: captures.get(0).unwrap().as_str().to_owned(),
                description: None, // not supported yet
                is_required: true,
                range: (start..std::usize::MAX),
            })
        } else {
            Err(failure::err_msg(format!("invalid accessor: {}", end)))
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
            let actual = if argument.is_unlimited_range() {
                variables.len()
            } else {
                argument.range.end
            };

            let params = match variables.get(argument.range.start..actual) {
                Some(values) => values
                    .iter()
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>(),
                None => {
                    if argument.is_required {
                        return Err(failure::err_msg("index out of bounds or invalid access"));
                    } else {
                        vec![]
                    }
                }
            };

            if argument.is_required && params.is_empty() {
                return Err(failure::err_msg("argument is empty"));
            }

            replaced = replaced.replace(&argument.capture_str, params.join(" ").as_str());
        }

        Ok(replaced.to_owned())
    }
}

impl Argument {
    pub fn description(&self) -> &str {
        match &self.description {
            Some(value) => value,
            None => {
                if self.is_optional_range() {
                    "Extra arguments that passing to original command"
                } else {
                    "No description provided"
                }
            }
        }
    }

    pub fn attribute(&self) -> &str {
        if self.is_required {
            "required"
        } else {
            "optional"
        }
    }

    fn is_unlimited_range(&self) -> bool {
        self.range.end == std::usize::MAX
    }

    fn is_optional_range(&self) -> bool {
        !self.is_required && self.is_unlimited_range()
    }
}

#[cfg(test)]
impl Clone for Argument {
    fn clone(&self) -> Self {
        let description = match &self.description {
            Some(value) => Some(value.to_owned()),
            None => None,
        };

        Argument {
            capture_str: self.capture_str.to_owned(),
            description,
            is_required: if self.is_required { true } else { false },
            range: Range {
                start: self.range.start,
                end: self.range.end,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ArgParser, Argument};
    use std::ops::Range;
    use std::usize;

    // helper
    fn initialize_and_parsed(string: &str) -> Result<Vec<Argument>, failure::Error> {
        let mut parser = ArgParser::new(string, None);
        parser.parse()?;

        Ok(parser
            .arguments()
            .unwrap()
            .iter()
            .map(|w| w.clone())
            .collect())
    }

    fn unlimited_range(start: usize) -> Range<usize> {
        Range {
            start,
            end: usize::MAX,
        }
    }

    #[test]
    fn parse() {
        // required index argument
        let arguments = initialize_and_parsed("{0}").unwrap();

        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0].capture_str, "{0}");
        assert_eq!(arguments[0].is_required, true);
        assert_eq!(arguments[0].range, 0..1);

        // optional index argument
        let arguments = initialize_and_parsed("{0?}").unwrap();

        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0].capture_str, "{0?}");
        assert_eq!(arguments[0].is_required, false);
        assert_eq!(arguments[0].range, 0..1);

        // required range argument (omitted last index)
        let arguments = initialize_and_parsed("{0..}").unwrap();

        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0].capture_str, "{0..}");
        assert_eq!(arguments[0].is_required, true);
        assert_eq!(arguments[0].range, unlimited_range(0));

        // required range argument
        let arguments = initialize_and_parsed("{0..2}").unwrap();

        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0].capture_str, "{0..2}");
        assert_eq!(arguments[0].is_required, true);
        assert_eq!(arguments[0].range, 0..2);

        // optional range argument
        let arguments = initialize_and_parsed("{0..?}").unwrap();

        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0].capture_str, "{0..?}");
        assert_eq!(arguments[0].is_required, false);
        assert_eq!(arguments[0].range, unlimited_range(0));

        // multiple ordered arguments (index)
        let arguments = initialize_and_parsed("{0} {1?}").unwrap();

        assert_eq!(arguments.len(), 2);
        assert_eq!(arguments[0].capture_str, "{0}");
        assert_eq!(arguments[0].is_required, true);
        assert_eq!(arguments[0].range, 0..1);
        assert_eq!(arguments[1].capture_str, "{1?}");
        assert_eq!(arguments[1].is_required, false);
        assert_eq!(arguments[1].range, 1..2);

        // multiple inverted arguments (index)
        let arguments = initialize_and_parsed("{1?} {0}").unwrap();

        assert_eq!(arguments.len(), 2);
        assert_eq!(arguments[0].capture_str, "{0}");
        assert_eq!(arguments[0].is_required, true);
        assert_eq!(arguments[0].range, 0..1);
        assert_eq!(arguments[1].capture_str, "{1?}");
        assert_eq!(arguments[1].is_required, false);
        assert_eq!(arguments[1].range, 1..2);

        // multiple same arguments (index)
        let arguments = initialize_and_parsed("{0} {0}").unwrap();

        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0].capture_str, "{0}");
        assert_eq!(arguments[0].is_required, true);
        assert_eq!(arguments[0].range, 0..1);

        // no matches
        let arguments = initialize_and_parsed("").unwrap();

        assert_eq!(arguments.len(), 0);

        // no matches
        let arguments = initialize_and_parsed("{-1}").unwrap();

        assert_eq!(arguments.len(), 0);
    }
}
