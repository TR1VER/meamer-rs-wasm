use sophia_api::term::TermKind;
use sophia_inmem::graph::FastGraph;
use sophia_term::RcTerm;

use super::{FromVocab, TermMapExtractor};
use crate::extractors::error::ParseError;
use crate::extractors::Extractor;
use crate::rml_model::term_map::{PredicateMap, TermMapInfo};

impl TermMapExtractor<PredicateMap> for PredicateMap {
    fn create_term_map(
        subj_ref: &RcTerm,
        graph_ref: &FastGraph,
    ) -> super::ExtractorResult<PredicateMap> {
        let mut tm_info = TermMapInfo::extract(subj_ref, graph_ref)?;

        tm_info = match tm_info.term_type {
            Some(ttype) if ttype != TermKind::Iri => {
                return Err(ParseError::GenericError(format!(
                    "PredicateMap can only have rr:Iri as rr:termType!",
                )))
            }
            Some(_) => tm_info,
            None => {
                TermMapInfo {
                    term_type: Some(TermKind::Iri),
                    ..tm_info
                }
            }
        };

        Ok(PredicateMap { tm_info })
    }

    fn create_constant_map(tm_info: TermMapInfo) -> PredicateMap {
        if tm_info.term_type != Some(TermKind::Iri) {
            panic!("Constant-valued PredicateMap has to have an IRI as value");
        }
        PredicateMap { tm_info }
    }

    fn get_map_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::PREDICATEMAP.to_term()
    }

    fn get_const_pred() -> RcTerm {
        vocab::r2rml::PROPERTY::PREDICATE.to_term()
    }
}
