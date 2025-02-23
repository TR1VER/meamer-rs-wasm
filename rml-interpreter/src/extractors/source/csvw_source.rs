use std::collections::HashMap;

use lazy_static::lazy_static;
use sophia_api::term::TTerm;
use sophia_inmem::graph::FastGraph;


use crate::extractors::store::get_object;

use crate::extractors::{ExtractorResult, FromVocab, RcTerm};
use crate::rml_model::source_target::{Source};
use crate::TermString;

lazy_static! {
    static ref PARSE_CONFIGS_PREDICATES: Vec<(String, TermString)> = vec![
        (
            vocab::csvw::PROPERTY::TRIM.1.to_string(),
            vocab::csvw::PROPERTY::TRIM.to_term()
        ),
        (
            vocab::csvw::PROPERTY::COMMENT_PREFIX.1.to_string(),
            vocab::csvw::PROPERTY::COMMENT_PREFIX.to_term()
        ),
        (
            vocab::csvw::PROPERTY::DELIMITER.1.to_string(),
            vocab::csvw::PROPERTY::DELIMITER.to_term()
        ),
        (
            vocab::csvw::PROPERTY::DOUBLE_QUOTE.1.to_string(),
            vocab::csvw::PROPERTY::DOUBLE_QUOTE.to_term()
        ),
        (
            vocab::csvw::PROPERTY::ENCODING.1.to_string(),
            vocab::csvw::PROPERTY::ENCODING.to_term()
        ),
        (
            vocab::csvw::PROPERTY::HEADER.1.to_string(),
            vocab::csvw::PROPERTY::HEADER.to_term()
        ),
        (
            vocab::csvw::PROPERTY::HEADER_ROW_COUNT.1.to_string(),
            vocab::csvw::PROPERTY::HEADER_ROW_COUNT.to_term()
        ),
        (
            vocab::csvw::PROPERTY::LINE_TERMINATORS.1.to_string(),
            vocab::csvw::PROPERTY::LINE_TERMINATORS.to_term()
        ),
        (
            vocab::csvw::PROPERTY::QUOTE_CHARS.1.to_string(),
            vocab::csvw::PROPERTY::QUOTE_CHARS.to_term()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_ROWS.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_ROWS.to_term()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_COLUMNS.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_COLUMNS.to_term()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_BLANK_ROWS.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_BLANK_ROWS.to_term()
        ),
        (
            vocab::csvw::PROPERTY::SKIP_INITIAL_SPACE.1.to_string(),
            vocab::csvw::PROPERTY::SKIP_INITIAL_SPACE.to_term()
        ),
    ];
}

fn extract_parse_config(
    dialect_subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<HashMap<String, String>> {
    let mut result = HashMap::new();

    let _ = PARSE_CONFIGS_PREDICATES.iter().try_for_each(
        |(key, config_pred)| -> ExtractorResult<()> {
            let config_val = get_object(graph, dialect_subject, config_pred)?;
            result.insert(key.to_string(), config_val.value().to_string());
            Ok(())
        },
    );

    Ok(result)
}

pub fn extract_csvw_source(
    subject: &RcTerm,
    graph: &FastGraph,
) -> ExtractorResult<Source> {
    let url_pred = vocab::csvw::PROPERTY::URL.to_rcterm();
    let url = get_object(graph, subject, &url_pred)?.value().to_string();
    let dialect_pred = vocab::csvw::PROPERTY::DIALECT.to_rcterm();
    let dialect_iri = get_object(graph, subject, &dialect_pred)?;
    let parse_config = extract_parse_config(&dialect_iri, graph)?;

    Ok(Source::CSVW { url, parse_config })
}
