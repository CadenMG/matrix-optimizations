use std::time::{Instant};
use std::thread;
use rand;

const N: usize = 1024;

fn random_array() -> [[f64; N]; N] {
    let mut res: [[f64; N]; N] = [[0.0; N]; N];
    for i in 0..N {
        for j in 0..N {
            res[i][j] = rand::random();
        }
    }
    res
}

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
}

fn main() {
    // Spawn thread with explicit stack size
    let child = thread::Builder::new()
        .stack_size(1024 * 1024 * 32)
        .spawn(do_work)
        .unwrap();

    // Wait for thread to join
    child.join().unwrap();
}
