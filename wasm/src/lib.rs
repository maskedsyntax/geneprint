use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DNASequence {
    sequence: String,
    bases: Vec<char>,
}

#[wasm_bindgen]
impl DNASequence {
    #[wasm_bindgen(getter)]
    pub fn sequence(&self) -> String {
        self.sequence.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn length(&self) -> usize {
        self.bases.len()
    }
}

// Simple SDBM hash function to get a u32 seed from a string
fn hash_string(s: &str) -> u32 {
    let mut hash: u32 = 0;
    for c in s.chars() {
        hash = (c as u32).wrapping_add(hash << 6).wrapping_add(hash << 16).wrapping_sub(hash);
    }
    hash
}

// Mulberry32 PRNG
struct Mulberry32 {
    state: u32,
}

impl Mulberry32 {
    fn new(seed: u32) -> Self {
        Self { state: seed }
    }

    fn next(&mut self) -> u32 {
        self.state = self.state.wrapping_add(0x6D2B79F5);
        let mut z = self.state;
        z = (z ^ (z >> 15)).wrapping_mul(z | 1);
        z ^= z.wrapping_add(z ^ (z >> 7)).wrapping_mul(z | 61);
        z ^ (z >> 14)
    }

    fn next_f32(&mut self) -> f32 {
        (self.next() as f32) / (u32::MAX as f32)
    }

    fn next_range(&mut self, min: usize, max: usize) -> usize {
        let range = max - min;
        min + (self.next() as usize % range)
    }
}

#[wasm_bindgen]
pub fn generate_dna_sequence(identifier: &str, length: usize) -> DNASequence {
    let seed = hash_string(identifier);
    let mut rng = Mulberry32::new(seed);
    
    let bases = ['A', 'C', 'G', 'T'];
    let mut sequence = String::with_capacity(length);
    let mut bases_vec = Vec::with_capacity(length);

    for _ in 0..length {
        let idx = rng.next_range(0, 4);
        let base = bases[idx];
        sequence.push(base);
        bases_vec.push(base);
    }

    DNASequence {
        sequence,
        bases: bases_vec,
    }
}

#[wasm_bindgen]
pub fn get_base_color(base: char) -> String {
    match base {
        'A' => "#4ADE80".to_string(), // Tailwind green-400
        'C' => "#60A5FA".to_string(), // Tailwind blue-400
        'G' => "#FACC15".to_string(), // Tailwind yellow-400
        'T' => "#F87171".to_string(), // Tailwind red-400
        _ => "#FFFFFF".to_string(),
    }
}
