use ndarray::{Array1, Array2};
use std::fmt::Display;
use std::cmp::min;

#[derive(Debug)]
pub struct CountMinSketch {
    num_row: i64,
    num_col: i64,
    counters: Array2<i64>
}
//
impl CountMinSketch {
    fn new(num_row: i64, num_col: i64) -> Self {
        CountMinSketch{
            num_row: num_row,
            num_col: num_col,
            counters: Array2::<i64>::zeros((num_row as usize, num_col as usize))
        }
    }

    fn insert(&mut self, key: i64, value: i64) {
        // TODO: add hash function traits.
        let mut j: usize = 0;
        for i in 0..(self.num_row as usize) {
            j = (key % self.num_col) as usize;
            self.counters[[i,j]] += value
        }
    }

    fn query(&self, key: i64) -> i64 {
        // TODO: hash functions traits.
        let mut hash_key: usize = key as usize;
        let mut result: i64 = self.counters[[0,hash_key]];
        for i in 0..(self.num_row as usize) {
            hash_key = key as usize;
            result = min(result, self.counters[[i,hash_key]]);
        }
        result
    }
}


fn main() {
    let mut sketch = CountMinSketch::new(3,5);
    sketch.insert(4, 114514);
    println!("sketch = {:?}", sketch);
    println!("Hello, world!");
}
