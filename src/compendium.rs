use std::collections::HashMap;

use crate::pattern::AgentPattern;
use crate::spellbook::Spellbook;

#[derive(Debug, Clone)]
pub struct FleetCompendium {
    spellbooks: HashMap<String, Spellbook>,
}

impl FleetCompendium {
    pub fn new() -> Self {
        Self {
            spellbooks: HashMap::new(),
        }
    }

    pub fn register(&mut self, vessel_id: &str, spellbook: Spellbook) {
        self.spellbooks.insert(vessel_id.to_string(), spellbook);
    }

    pub fn find_pattern(&self, name: &str) -> Vec<(&str, &AgentPattern)> {
        self.spellbooks
            .iter()
            .filter_map(|(id, sb)| sb.get(name).map(|p| (id.as_str(), p)))
            .collect()
    }

    pub fn popular_patterns(&self, n: usize) -> Vec<(String, String, u64)> {
        let mut all: Vec<(String, String, u64)> = Vec::new();
        for (vessel_id, sb) in &self.spellbooks {
            // We need pattern names; iterate via search("") or store them.
            // Use search with empty query to get all patterns.
            for p in sb.search("") {
                all.push((vessel_id.clone(), p.name.clone(), p.times_used));
            }
        }
        all.sort_by(|a, b| b.2.cmp(&a.2));
        all.truncate(n);
        all
    }

    pub fn cross_vessel_search(&self, query: &str) -> Vec<(&str, &AgentPattern)> {
        let mut results: Vec<(&str, &AgentPattern)> = Vec::new();
        for (vessel_id, sb) in &self.spellbooks {
            for p in sb.search(query) {
                results.push((vessel_id.as_str(), p));
            }
        }
        results
    }

    pub fn merge(&mut self, other: FleetCompendium) {
        for (vessel_id, sb) in other.spellbooks {
            self.spellbooks.insert(vessel_id, sb);
        }
    }
}
