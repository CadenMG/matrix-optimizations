use std::time::{Instant};
use std::thread;
use rand;
use rayon::prelude::*;

const N: usize = 4096;

fn random_array() -> [[f64; N]; N] {
    let mut res: [[f64; N]; N] = [[0.0; N]; N];
    for i in 0..N {
        for j in 0..N {
            res[i][j] = rand::random();
        }
    }
    res
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn do_work() {
    let A = random_array();
    let B = random_array();
    let mut C = [[0.0; N]; N];

    let start = Instant::now();
    for i in 0..N {
            for k in 0..N {
        for j in 0..N {
                C[i][j] += A[i][k] * B[k][j];
            }
        }
    }
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    println!("{:?}", C[0][0]);
}

#[allow(non_snake_case)]
#[allow(dead_code)]
fn do_work_parallel() {
    let A = random_array();
    let B = random_array();
    let mut C = [[0.0; N]; N];

    let start = Instant::now();
    C.par_iter_mut().enumerate().for_each(|(i, row)| {
        row.par_iter_mut().enumerate().for_each(|(k, val)| 
            for j in 0..N {
                *val += A[i][k] * B[k][j];
            }
        );
    });
    let duration = start.elapsed();
    println!("Time taken: {:?}", duration);
    println!("{:?}", C[0][0]);
}

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(4096 * 4096 * 32)
        .spawn(do_work_parallel)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
