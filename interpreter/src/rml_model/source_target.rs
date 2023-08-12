use std::collections::HashMap;

use operator::formats::DataFormat;
use operator::IOType;
use sophia_api::term::TTerm;
use vocab::ToString;

use crate::{IriString, TermString};

#[derive(Debug, Clone)]
pub struct LogicalSource {
    pub identifier:            TermString,
    pub iterator:              Option<String>,
    pub source:                Source,
    pub reference_formulation: IriString,
}

impl Into<operator::Source> for LogicalSource {
    fn into(self) -> operator::Source {
        let source_type = match &self.source {
            Source::FileInput { path: _ } => IOType::File,
        };

        let data_format = match &self.reference_formulation.value().to_string()
        {
            p if *p == vocab::query::CLASS::CSV.to_string() => DataFormat::CSV,
            p if *p == vocab::query::CLASS::JSONPATH.to_string() => {
                DataFormat::JSON
            }
            p if *p == vocab::query::CLASS::XPATH.to_string() => {
                DataFormat::XML
            }
            p => panic!("Data format not supported {} ", p),
        };

        let reference_iterators = match &self.iterator{
            Some(iter) => vec![iter.to_owned()],
            None => vec![],
        };

        operator::Source {
            config: source_config_map(&self),
            reference_iterators,
            source_type,
            data_format,
        }
    }
}
#[derive(Debug, Clone)]
pub struct LogicalTarget {
    pub identifier:    TermString,
    pub compression:   Option<IriString>,
    pub serialization: IriString,
    pub output:        Output,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Source {
    FileInput { path: String },
}

impl Into<HashMap<String, String>> for Source {
    fn into(self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        match self {
            Source::FileInput { path } => map.insert("path".to_string(), path),
        };

        map
    }
}

#[derive(Debug, Clone)]
pub enum FileMode {
    Append,
    Overwrite,
}

#[derive(Debug, Clone)]
pub enum Output {
    FileOutput { path: String, mode: FileMode },
}

fn source_config_map(ls: &LogicalSource) -> HashMap<String, String> {
    let mut map = HashMap::new();

    map.insert("identifier".to_string(), ls.identifier.to_string());

    if let Some(iter) = &ls.iterator {
        map.insert("iterator".to_string(), iter.to_owned());
    }

    let source_map: HashMap<String, String> = ls.source.clone().into();

    map.extend(source_map.into_iter());

    map
}
