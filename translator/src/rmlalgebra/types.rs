use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use rml_interpreter::rml_model::term_map::{
    GraphMap, ObjectMap, PredicateMap, SubjectMap,
};
use rml_interpreter::rml_model::{PredicateObjectMap, TriplesMap};
use operator::Target;
use plangenerator::plan::{Plan, Processed};

#[derive(Debug, Clone)]
pub struct RefPOM<'a> {
    pub pm: Vec<&'a PredicateMap>,
    pub om: Vec<&'a ObjectMap>,
}

impl<'a> From<&'a PredicateObjectMap> for RefPOM<'a> {
    fn from(value: &'a PredicateObjectMap) -> Self {
        Self {
            pm: value.predicate_maps.iter().collect(),
            om: value.object_maps.iter().collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Quads<'a> {
    pub triples: Triples<'a>,
    pub gms:     Vec<&'a GraphMap>,
}

#[derive(Debug, Clone)]
pub struct Triples<'a> {
    pub sm:   &'a SubjectMap,
    pub poms: Vec<RefPOM<'a>>,
}


#[derive(Debug, Clone)]
pub struct SearchMap<'a> {
    pub tm_rccellplan_map:
        HashMap<String, (&'a TriplesMap, Rc<RefCell<Plan<Processed>>>)>,
    pub variable_map:       HashMap<String, String>,
    pub target_map:         HashMap<String, Target>,
    pub lt_id_tm_group_map: HashMap<String, Vec<Quads<'a>>>,
}
