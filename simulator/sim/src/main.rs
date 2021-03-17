// Luke Anglin and Tobi Solarin
use rand::prelude::*; // For the rng
const N: usize = 1_000; // The number of trials
/// Monte-Carlo simulation function
/// # Parameters
///
/// None
///
/// # Return Value
///
///
///
/// # Example
/// ```rust
/// println("Hello")
/// ```
fn simulate() -> [f64; 1_000] {
    // Array initialization
    let mut w_final: [f64; N] = [0.0; 1_000];
    // Loop through n times
    for sim in 0..N {
        // Initialize global variables
        let mut w: f64 = 0.0; // The total time from rep call
        let mut c = 0; // The total calls so far
                       // We only do this while the calls is < 4
        while c < 4 {
            // Begin at the root of the tree.
            // Get a random number
            let mut rng = thread_rng();
            let rand: f64 = rng.gen();
            // Add 6 for call time
            w += 6f64;
            // Cases:
            // They're busy
            if rand <= 0.2 {
                w += 6f64;
                c += 1;
                continue;
            }

            // They're unavailable
            if 0.2 < rand && rand <= 0.5 {
                w += 26f64;
                c += 1;
                continue;
            }

            // They're available
            if rand > 0.5 {
                w += -12f64 * (1.0f64 - rand).ln();// This is a continuous, exponential random variable.
                break;
            }
        }

        // Now we have W. We put this in the global array of W's.
        w_final[sim] = w;
    }
    // Now, we return the array
    return w_final;
}

fn main() {
    let result: [f64; 1000] = simulate();
    for elem in 0..result.len() {
        println!("{}", result[elem]);
    }
}
