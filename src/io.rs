// src/io.rs
//! Minimal IO helpers placeholder for RustySrota.

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportNode {
    pub id: usize,
    pub x: f64,
    pub y: f64,
}

pub fn export_nodes_json<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(data)
}