use std::hash::Hash;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PatternCategory {
    Perception,
    Decision,
    Action,
    Learning,
    Survival,
    Social,
    Meta,
}

#[derive(Debug, Clone)]
pub struct AgentPattern {
    pub name: String,
    pub category: PatternCategory,
    pub description: String,
    pub opcodes_used: Vec<u8>,
    pub energy_cost: u32,
    pub trust_required: f64,
    pub confidence_sensitive: bool,
    pub flux_ese_source: String,
    pub bytecodes: Vec<u8>,
    pub success_rate: f64,
    pub times_used: u64,
}
