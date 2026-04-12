use std::collections::HashMap;

use crate::pattern::{AgentPattern, PatternCategory};

#[derive(Debug, Clone)]
pub struct Spellbook {
    patterns: HashMap<String, AgentPattern>,
    categories: HashMap<PatternCategory, Vec<String>>,
}

impl Spellbook {
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            categories: HashMap::new(),
        }
    }

    pub fn add(&mut self, pattern: AgentPattern) {
        self.categories
            .entry(pattern.category.clone())
            .or_default()
            .push(pattern.name.clone());
        self.patterns.insert(pattern.name.clone(), pattern);
    }

    pub fn get(&self, name: &str) -> Option<&AgentPattern> {
        self.patterns.get(name)
    }

    pub fn by_category(&self, cat: PatternCategory) -> Vec<&AgentPattern> {
        self.categories
            .get(&cat)
            .map(|names| {
                names
                    .iter()
                    .filter_map(|n| self.patterns.get(n))
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn search(&self, query: &str) -> Vec<&AgentPattern> {
        let query_lower = query.to_lowercase();
        self.patterns
            .values()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower)
                    || p.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn most_used(&self, n: usize) -> Vec<&AgentPattern> {
        let mut all: Vec<&AgentPattern> = self.patterns.values().collect();
        all.sort_by(|a, b| b.times_used.cmp(&a.times_used));
        all.truncate(n);
        all
    }

    pub fn most_reliable(&self, n: usize) -> Vec<&AgentPattern> {
        let mut all: Vec<&AgentPattern> = self.patterns.values().collect();
        all.sort_by(|a, b| b.success_rate.partial_cmp(&a.success_rate).unwrap());
        all.truncate(n);
        all
    }

    pub fn by_energy(&self, max_cost: u32) -> Vec<&AgentPattern> {
        self.patterns
            .values()
            .filter(|p| p.energy_cost <= max_cost)
            .collect()
    }

    pub fn update_success(&mut self, name: &str, success: bool) {
        if let Some(p) = self.patterns.get_mut(name) {
            p.times_used += 1;
            let wins = p.success_rate * (p.times_used as f64 - 1.0)
                + if success { 1.0 } else { 0.0 };
            p.success_rate = wins / p.times_used as f64;
        }
    }

    pub fn total_patterns(&self) -> usize {
        self.patterns.len()
    }
}
