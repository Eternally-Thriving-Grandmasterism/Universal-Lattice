// src/lib.rs — Universal Lattice Root Bridge (ZK Valence Broadcast Integration)
// Eternal Thriving Grandmasterism — Jan 19 2026

pub mod halo2;
pub mod translations;

use halo2::zk_valence::aggregate_and_broadcast;

pub fn broadcast_valence(valences: Vec<f64>) -> Result<String, &'static str> {
    aggregate_and_broadcast(valences)
}
