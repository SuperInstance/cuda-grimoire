use cuda_grimoire::*;
use std::collections::HashSet;

fn make_test_pattern(name: &str, cat: PatternCategory, opcodes: Vec<u8>, energy: u32, confidence: bool) -> AgentPattern {
    AgentPattern {
        name: name.into(),
        category: cat,
        description: format!("test pattern {}", name),
        opcodes_used: opcodes,
        energy_cost: energy,
        trust_required: 0.5,
        confidence_sensitive: confidence,
        flux_ese_source: format!("fn {}() {{}}", name),
        bytecodes: vec![0x00],
        success_rate: 0.9,
        times_used: 10,
    }
}

#[test]
fn test_add_pattern_to_spellbook() {
    let mut sb = Spellbook::new();
    let p = make_test_pattern("alpha", PatternCategory::Perception, vec![0x01], 3, false);
    sb.add(p);
    assert_eq!(sb.total_patterns(), 1);
    assert!(sb.get("alpha").is_some());
}

#[test]
fn test_get_pattern_by_name() {
    let mut sb = Spellbook::new();
    sb.add(make_test_pattern("bravo", PatternCategory::Action, vec![0x02], 5, true));
    let got = sb.get("bravo");
    assert!(got.is_some());
    assert_eq!(got.unwrap().name, "bravo");
    assert!(sb.get("nonexistent").is_none());
}

#[test]
fn test_list_by_category() {
    let mut sb = Spellbook::new();
    sb.add(make_test_pattern("cat_a", PatternCategory::Perception, vec![0x01], 1, false));
    sb.add(make_test_pattern("cat_b", PatternCategory::Perception, vec![0x02], 2, false));
    sb.add(make_test_pattern("cat_c", PatternCategory::Action, vec![0x03], 3, false));
    let perception = sb.by_category(PatternCategory::Perception);
    assert_eq!(perception.len(), 2);
    let action = sb.by_category(PatternCategory::Action);
    assert_eq!(action.len(), 1);
}

#[test]
fn test_search_by_name_substring() {
    let mut sb = Spellbook::new();
    sb.add(make_test_pattern("hello_world", PatternCategory::Meta, vec![], 1, false));
    sb.add(make_test_pattern("world_peace", PatternCategory::Social, vec![], 1, false));
    sb.add(make_test_pattern("no_match", PatternCategory::Meta, vec![], 1, false));
    let results = sb.search("world");
    assert_eq!(results.len(), 2);
}

#[test]
fn test_most_used_returns_sorted() {
    let mut sb = Spellbook::new();
    let mut p1 = make_test_pattern("low", PatternCategory::Meta, vec![], 1, false);
    p1.times_used = 5;
    let mut p2 = make_test_pattern("high", PatternCategory::Meta, vec![], 1, false);
    p2.times_used = 100;
    let mut p3 = make_test_pattern("mid", PatternCategory::Meta, vec![], 1, false);
    p3.times_used = 50;
    sb.add(p1);
    sb.add(p2);
    sb.add(p3);
    let top = sb.most_used(2);
    assert_eq!(top[0].name, "high");
    assert_eq!(top[1].name, "mid");
}

#[test]
fn test_most_reliable_sorted_by_success_rate() {
    let mut sb = Spellbook::new();
    let mut p1 = make_test_pattern("ok", PatternCategory::Meta, vec![], 1, false);
    p1.success_rate = 0.7;
    let mut p2 = make_test_pattern("great", PatternCategory::Meta, vec![], 1, false);
    p2.success_rate = 0.99;
    sb.add(p1);
    sb.add(p2);
    let top = sb.most_reliable(2);
    assert_eq!(top[0].name, "great");
    assert_eq!(top[1].name, "ok");
}

#[test]
fn test_filter_by_energy_cost() {
    let mut sb = Spellbook::new();
    sb.add(make_test_pattern("cheap", PatternCategory::Meta, vec![], 2, false));
    sb.add(make_test_pattern("expensive", PatternCategory::Meta, vec![], 20, false));
    let filtered = sb.by_energy(5);
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0].name, "cheap");
}

#[test]
fn test_update_success_increases_times_used() {
    let mut sb = Spellbook::new();
    sb.add(make_test_pattern("counter", PatternCategory::Meta, vec![], 1, false));
    assert_eq!(sb.get("counter").unwrap().times_used, 10);
    sb.update_success("counter", true);
    assert_eq!(sb.get("counter").unwrap().times_used, 11);
}

#[test]
fn test_update_success_changes_rate_correctly() {
    let mut sb = Spellbook::new();
    let mut p = make_test_pattern("rate_test", PatternCategory::Meta, vec![], 1, false);
    p.times_used = 10;
    p.success_rate = 0.8; // 8 successes out of 10
    sb.add(p);
    sb.update_success("rate_test", true);  // 9/11
    assert!((sb.get("rate_test").unwrap().success_rate - 9.0 / 11.0).abs() < 1e-9);
    sb.update_success("rate_test", false); // 9/12
    assert!((sb.get("rate_test").unwrap().success_rate - 9.0 / 12.0).abs() < 1e-9);
}

#[test]
fn test_confidence_sensitive_flag() {
    let recipes = built_in_recipes();
    let sensitive: Vec<_> = recipes.iter().filter(|r| r.confidence_sensitive).collect();
    let insensitive: Vec<_> = recipes.iter().filter(|r| !r.confidence_sensitive).collect();
    assert!(!sensitive.is_empty());
    assert!(!insensitive.is_empty());
}

#[test]
fn test_recipe_safe_ask_has_correct_opcodes() {
    let recipes = built_in_recipes();
    let safe_ask = recipes.iter().find(|r| r.name == "safe_ask").unwrap();
    assert_eq!(safe_ask.opcodes_used, vec![0x01, 0x03, 0x07]);
    assert!(!safe_ask.flux_ese_source.is_empty());
}

#[test]
fn test_recipe_conserve_energy_has_correct_category() {
    let recipes = built_in_recipes();
    let ce = recipes.iter().find(|r| r.name == "conserve_energy").unwrap();
    assert_eq!(ce.category, PatternCategory::Survival);
}

#[test]
fn test_compendium_register_vessel() {
    let mut comp = FleetCompendium::new();
    let mut sb = Spellbook::new();
    sb.add(make_test_pattern("pat1", PatternCategory::Meta, vec![], 1, false));
    comp.register("vessel-01", sb);
    let results = comp.find_pattern("pat1");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, "vessel-01");
}

#[test]
fn test_compendium_find_pattern_across_vessels() {
    let mut comp = FleetCompendium::new();
    let mut sb1 = Spellbook::new();
    sb1.add(make_test_pattern("shared", PatternCategory::Meta, vec![], 1, false));
    let mut sb2 = Spellbook::new();
    sb2.add(make_test_pattern("shared", PatternCategory::Meta, vec![], 1, false));
    comp.register("vessel-a", sb1);
    comp.register("vessel-b", sb2);
    let results = comp.find_pattern("shared");
    let ids: HashSet<&str> = results.iter().map(|(id, _)| *id).collect();
    assert!(ids.contains("vessel-a"));
    assert!(ids.contains("vessel-b"));
    assert_eq!(ids.len(), 2);
}

#[test]
fn test_compendium_cross_vessel_search() {
    let mut comp = FleetCompendium::new();
    let mut sb1 = Spellbook::new();
    sb1.add(make_test_pattern("fire_spell", PatternCategory::Action, vec![], 1, false));
    let mut sb2 = Spellbook::new();
    sb2.add(make_test_pattern("water_spell", PatternCategory::Action, vec![], 1, false));
    comp.register("mage-1", sb1);
    comp.register("mage-2", sb2);
    let results = comp.cross_vessel_search("spell");
    assert_eq!(results.len(), 2);
}
