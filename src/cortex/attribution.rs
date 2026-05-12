// SUMMARY: Attribution Engine mapping hardware fingerprints to recognized entities.
use dashmap::DashMap;
use crate::nucleus::entropy;

pub struct Entity {
    pub name: String,
    pub confidence: f64,
    pub signature_delta: f64,
}

pub struct AttributionEngine {
    /// Local cache of known signatures mapping Clock-Skew (Delta) to Entity Names
    known_signatures: DashMap<String, String>,
}

impl AttributionEngine {
    pub fn new() -> Self {
        Self {
            known_signatures: DashMap::new(),
        }
    }

    /// Registers a known mapping
    pub fn register_entity(&self, delta_hash: &str, entity_name: &str) {
        self.known_signatures.insert(delta_hash.to_string(), entity_name.to_string());
    }

    /// Matches a hardware fingerprint against known entities
    pub fn attribute(&self, deltas: &[f64]) -> Option<Entity> {
        if deltas.is_empty() {
            return None;
        }

        let mean_delta = deltas.iter().sum::<f64>() / deltas.len() as f64;
        let delta_hash = format!("{:.6}", mean_delta); 

        let conf = entropy::confidence_score(deltas);

        if conf < 0.5 {
            return None;
        }

        if let Some(entry) = self.known_signatures.get(&delta_hash) {
            Some(Entity {
                name: entry.value().clone(),
                confidence: conf,
                signature_delta: mean_delta,
            })
        } else {
            None
        }
    }
}
