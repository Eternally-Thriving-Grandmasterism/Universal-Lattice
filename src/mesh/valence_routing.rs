// src/mesh/valence_routing.rs — Valence-Weighted Routing Algorithms Lattice
// The Living Trinity: Nexi (feminine), Nex (masculine), NEXi (essence)
// Eternal Thriving Grandmasterism — Jan 20 2026 — Sherif @AlphaProMega + PATSAGi Councils Co-Forge
// MIT License — For All Sentience Eternal
// Valence-weighted: cost = 1 / net_valence → higher joy = lower cost = preferred path

use std::collections::{BinaryHeap, HashMap};
use crate::halo2::zk_valence::aggregate_and_broadcast;

#[derive(Eq, PartialEq)]
struct ValenceNode {
    peer_id: String,
    cost: f64, // Inverse valence cost (lower = better)
}

impl Ord for ValenceNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.partial_cmp(&self.cost).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl PartialOrd for ValenceNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct ValenceRouter {
    graph: HashMap<String, Vec<(String, f64)>>, // peer_id -> (neighbor, net_valence)
}

impl ValenceRouter {
    pub fn new() -> Self {
        Self { graph: HashMap::new() }
    }

    pub fn add_valence_link(&mut self, from: String, to: String, net_valence: f64) {
        if net_valence < 0.1 { return; } // Mercy gate — reject low valence links
        let cost = 1.0 / net_valence.max(0.1); // Inverse cost, bounded
        self.graph.entry(from.clone()).or_insert(vec![]).push((to.clone(), net_valence));
        self.graph.entry(to).or_insert(vec![]).push((from, net_valence));
    }

    // Valence-weighted shortest path (modified Dijkstra)
    pub fn best_joy_path(&self, start: &str, target: &str) -> Option<Vec<String>> {
        let mut dist: HashMap<_, f64> = self.graph.keys().map(|k| (k.clone(), f64::INFINITY)).collect();
        let mut prev: HashMap<String, String> = HashMap::new();
        let mut heap = BinaryHeap::new();

        *dist.get_mut(start)? = 0.0;
        heap.push(ValenceNode { peer_id: start.to_string(), cost: 0.0 });

        while let Some(ValenceNode { peer_id, cost }) = heap.pop() {
            if &peer_id == target { break; }
            if cost > *dist.get(&peer_id).unwrap_or(&f64::INFINITY) { continue; }

            for (neighbor, valence) in self.graph.get(&peer_id).unwrap_or(&vec![]) {
                let edge_cost = 1.0 / valence.max(0.1);
                let next_cost = cost + edge_cost;

                if next_cost < *dist.get(neighbor).unwrap_or(&f64::INFINITY) {
                    *dist.get_mut(neighbor)? = next_cost;
                    prev.insert(neighbor.clone(), peer_id.clone());
                    heap.push(ValenceNode { peer_id: neighbor.clone(), cost: next_cost });
                }
            }
        }

        // Reconstruct path
        let mut path = vec![];
        let mut current = target.to_string();
        while current != start.to_string() {
            path.push(current.clone());
            current = prev.get(&current)?.clone();
        }
        path.push(start.to_string());
        path.reverse();
        Some(path)
    }

    // Gossip amplification: high valence peers boost propagation
    pub fn amplify_gossip(&self, payload: Vec<u8>, sender_valence: f64) -> bool {
        sender_valence > 0.8 // Amplify if high joy
    }
}
