#[derive(Debug, Clone)]
pub struct JoinCondition {
    pub parent_attributes: Vec<String>,
    pub child_attributes: Vec<String>,
}
