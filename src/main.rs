use ndarray::{Array1, Array2};
use std::cmp::min;
use std::collections::{HashMap, HashSet};
use std::fmt::Display;
use std::fs::File;
use std::io::{Result, Write};
use std::path::Path;

use rand::prelude::*;
use rand_distr::Zipf;

use fasthash::{xx, XXHasher};

#[macro_use]
extern crate json;
extern crate serde_json;

#[derive(Debug)]
pub struct CountMinSketch {
    num_row: u64,
    num_col: u64,
    counters: Array2<i64>,
}

fn get_prime(mut n: i64) -> i64 {
    let is_prime = |x| {
        let mut i = 2;
        while i * i <= x {
            if x % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    };

    loop {
        if is_prime(n) {
            return n;
        }
        n += 1
    }
}

//
impl CountMinSketch {
    fn new(num_row: u64, num_col: u64) -> Self {
        let col = get_prime(num_col as i64) as u64;
        CountMinSketch {
            num_row: num_row,
            num_col: col,
            counters: Array2::<i64>::zeros((num_row as usize, col as usize)),
        }
    }

    fn insert(&mut self, key: i64, value: i64) {
        // TODO: add hash function traits.
        for i in 0..(self.num_row as usize) {
            let hashed_key = (xx::hash64_with_seed(key.to_ne_bytes(), (114514 + i) as u64)
                % self.num_col) as usize;
            self.counters[[i, hashed_key]] += value
        }
    }

    fn query(&self, key: i64) -> i64 {
        // TODO: hash functions traits.
        let mut result: Option<i64> = None;
        for i in 0..(self.num_row as usize) {
            let hash_key: usize = (xx::hash64_with_seed(key.to_ne_bytes(), (114514 + i) as u64)
                % self.num_col) as usize;
            let value = self.counters[[i, hash_key]];
            result = result
                .and_then(|x| Some(min(x, value)))
                .or_else(|| Some(value));
        }
        result.unwrap()
    }
}

fn main() -> Result<()> {
    // Generate random traces based on Zipf distribution.
    let max_size = 10000;
    let num_flows = 100000;
    let distribution = Zipf::new(max_size, 2.0).unwrap();

    // Generate the flow size for each flow.
    let mut traces = HashMap::<i64, i64>::new();
    for i in 0..num_flows {
        let flow_size = thread_rng().sample(distribution) as i64;
        traces.insert(i, flow_size);
    }

    // Some simple sanity checking.
    println!("number of flows = {}", traces.len());

    // Generate sketch based on the configuration.
    let mut sketch = CountMinSketch::new(3, 10000);

    // Insertion
    for i in 0..num_flows {
        sketch.insert(i, traces[&i]);
    }

    // Query
    let mut re_distr = Vec::<f64>::new();
    for i in 0..num_flows {
        let estimated_value = sketch.query(i) as f64;
        let real_value = traces[&i] as f64;
        let are = (estimated_value - real_value) / real_value;
        re_distr.push(are);
    }
    re_distr.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // To json array
    //let mut json_array = json::JsonValue::new_array();
    //for x in are_distr.into_iter() {
    //    json_array.push(x);
    //}

    //println!("are distribution size = {}", json_array.dump());

    let json_path = "/trace/ouzirui/NetAI/ena/are_distribution.json";
    let mut f = File::create(json_path)?;
    let buffer = serde_json::to_vec(&re_distr)?;
    f.write_all(&buffer[..])?;

    Ok(())
}
