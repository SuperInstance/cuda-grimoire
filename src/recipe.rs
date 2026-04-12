use crate::pattern::{AgentPattern, PatternCategory};

/// Returns all built-in recipes every vessel should know.
pub fn built_in_recipes() -> Vec<AgentPattern> {
    vec![
        AgentPattern {
            name: "safe_ask".into(),
            category: PatternCategory::Decision,
            description: "Ask another vessel with trust check and energy budget".into(),
            opcodes_used: vec![0x01, 0x03, 0x07],
            energy_cost: 5,
            trust_required: 0.6,
            confidence_sensitive: true,
            flux_ese_source: r#"
fn safe_ask(target: VesselId, query: Message) -> Result<Response> {
    let trust = trust::query(target);
    if trust < TRUST_THRESHOLD {
        return Err(Error::Untrusted(target));
    }
    let budget = energy::budget();
    if budget < ASK_COST {
        return Err(Error::InsufficientEnergy);
    }
    energy::spend(ASK_COST);
    let confidence = self::confidence();
    let enriched = query.with_confidence(confidence);
    telepathy::send(target, enriched)
}
"#.into(),
            bytecodes: vec![0xA1, 0x03, 0x07, 0xFF, 0x42],
            success_rate: 0.85,
            times_used: 120,
        },
        AgentPattern {
            name: "conserve_energy".into(),
            category: PatternCategory::Survival,
            description: "Check energy, reduce activity if low, request from pool if critical".into(),
            opcodes_used: vec![0x05, 0x0A],
            energy_cost: 1,
            trust_required: 0.0,
            confidence_sensitive: false,
            flux_ese_source: r#"
fn conserve_energy() -> Action {
    let level = energy::level();
    match level {
        l if l < CRITICAL => {
            energy::request_from_pool();
            Mode::Dormant
        }
        l if l < LOW => {
            Mode::Reduced
        }
        _ => Mode::Normal,
    }
}
"#.into(),
            bytecodes: vec![0xB2, 0x05, 0x0A],
            success_rate: 0.92,
            times_used: 340,
        },
        AgentPattern {
            name: "trust_gate".into(),
            category: PatternCategory::Decision,
            description: "Check trust of sender before processing message".into(),
            opcodes_used: vec![0x03, 0x04],
            energy_cost: 2,
            trust_required: 0.0,
            confidence_sensitive: false,
            flux_ese_source: r#"
fn trust_gate(sender: VesselId, msg: Message) -> GateResult {
    let trust = trust::query(sender);
    if trust < MIN_PROCESS_TRUST {
        trust::decay(sender, DECAY_RATE);
        return GateResult::Blocked;
    }
    trust::reinforce(sender, REINFORCE_AMOUNT);
    GateResult::Allowed(msg)
}
"#.into(),
            bytecodes: vec![0xC3, 0x03, 0x04],
            success_rate: 0.97,
            times_used: 2100,
        },
        AgentPattern {
            name: "calibrate".into(),
            category: PatternCategory::Learning,
            description: "Compare last N predictions to outcomes, update confidence".into(),
            opcodes_used: vec![0x08, 0x09],
            energy_cost: 3,
            trust_required: 0.0,
            confidence_sensitive: true,
            flux_ese_source: r#"
fn calibrate(n: usize) -> f64 {
    let predictions = memory::recent_predictions(n);
    let outcomes = memory::recent_outcomes(n);
    let mut correct = 0;
    for (p, o) in predictions.iter().zip(outcomes.iter()) {
        if p.matches(o) { correct += 1; }
    }
    let accuracy = correct as f64 / n as f64;
    let old_conf = self::confidence();
    let new_conf = old_conf * 0.7 + accuracy * 0.3;
    self::set_confidence(new_conf);
    new_conf
}
"#.into(),
            bytecodes: vec![0xD4, 0x08, 0x09],
            success_rate: 0.88,
            times_used: 560,
        },
        AgentPattern {
            name: "broadcast_status".into(),
            category: PatternCategory::Social,
            description: "Share current status with fleet".into(),
            opcodes_used: vec![0x01, 0x06],
            energy_cost: 4,
            trust_required: 0.3,
            confidence_sensitive: true,
            flux_ese_source: r#"
fn broadcast_status() {
    let status = Status {
        vessel: self::id(),
        energy: energy::level(),
        confidence: self::confidence(),
        load: self::task_queue_len(),
        trust_avg: trust::average(),
    };
    let fleet = fleet::members();
    for member in fleet {
        if trust::query(member) > FLEET_TRUST_MIN {
            telepathy::send(member, Message::Status(status.clone()));
        }
    }
}
"#.into(),
            bytecodes: vec![0xE5, 0x01, 0x06],
            success_rate: 0.94,
            times_used: 890,
        },
        AgentPattern {
            name: "delegate_and_reduce".into(),
            category: PatternCategory::Action,
            description: "Split task, delegate parts, collect results".into(),
            opcodes_used: vec![0x01, 0x02, 0x07],
            energy_cost: 8,
            trust_required: 0.7,
            confidence_sensitive: true,
            flux_ese_source: r#"
fn delegate_and_reduce(task: Task) -> Result<Output> {
    let parts = task.split();
    let mut handles = vec![];
    for part in parts {
        let delegate = fleet::find_best_for(&part);
        let trust = trust::query(delegate);
        if trust < DELEGATE_TRUST {
            continue;
        }
        let handle = telepathy::delegate(delegate, part);
        handles.push(handle);
    }
    let results: Vec<_> = handles.into_iter()
        .filter_map(|h| h.join().ok())
        .collect();
    task.merge(results)
}
"#.into(),
            bytecodes: vec![0xF6, 0x01, 0x02, 0x07],
            success_rate: 0.78,
            times_used: 230,
        },
        AgentPattern {
            name: "reflex_dodge".into(),
            category: PatternCategory::Survival,
            description: "Quick instinct-based response to alarm".into(),
            opcodes_used: vec![0x05, 0x0B],
            energy_cost: 2,
            trust_required: 0.0,
            confidence_sensitive: false,
            flux_ese_source: r#"
fn reflex_dodge(alarm: Alarm) -> ReflexAction {
    match alarm.severity() {
        Severity::Critical => {
            energy::reserve(DODGE_RESERVE);
            fleet::broadcast_alarm(alarm);
            ReflexAction::Evade
        }
        Severity::Warning => {
            self::increase_alertness();
            ReflexAction::Monitor
        }
        _ => ReflexAction::Ignore,
    }
}
"#.into(),
            bytecodes: vec![0xA7, 0x05, 0x0B],
            success_rate: 0.91,
            times_used: 78,
        },
        AgentPattern {
            name: "memory_consult".into(),
            category: PatternCategory::Perception,
            description: "Check long-term memory before deciding".into(),
            opcodes_used: vec![0x08, 0x00],
            energy_cost: 3,
            trust_required: 0.0,
            confidence_sensitive: true,
            flux_ese_source: r#"
fn memory_consult(context: &str) -> Option<MemoryResult> {
    let relevant = memory::search(context);
    if relevant.is_empty() {
        return None;
    }
    let ranked: Vec<_> = relevant.into_iter()
        .filter(|m| m.relevance() > MEMORY_RELEVANCE_MIN)
        .collect();
    if ranked.is_empty() {
        return None;
    }
    let best = ranked.into_iter().max_by_key(|m| m.confidence())?;
    self::reinforce_context(best.content());
    Some(MemoryResult { recall: best, context: context.into() })
}
"#.into(),
            bytecodes: vec![0xB8, 0x08, 0x00],
            success_rate: 0.83,
            times_used: 450,
        },
    ]
}
