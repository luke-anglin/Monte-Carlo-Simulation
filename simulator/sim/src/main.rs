// Luke Anglin and Tobi Solarin
use rand::Rng;
const N: usize = 1_000; // The number of trials
use std::io::Write;

fn rng(x: usize) -> usize {
    let a = 24693;
    let c = 1753;
    let k = 2u32.pow(15);
    if x == 0 {
        return 1000;
    } else {
        return (a * rng(x - 1) + c) % (k as usize);
    }
}

/// Monte-Carlo simulation function
/// # Parameters
///
/// None
///
/// # Return Value
///
/// Array of the results
///
/// # Example
/// ```rust
/// fn main() {
///     let result: [f64; 1000] = simulate();
///     for elem in 0..result.len() {
///         println!("{}", result[elem]);
///     }
/// }
/// ```

fn simulate() -> [f64; 1_000] {
    // Array initialization
    let mut w_final: [f64; N] = [0.0; 1_000];
    let mut sims = 0; 
    // Loop through n times
    for sim in 0..N {
        // Initialize global variables
        let mut w: f64 = 0.0; // The total time from rep call
        let mut c = 0; // The total calls so far
                       // We only do this while the calls is < 4
        while c < 4 {
            sims= sims+1;
            // Begin at the root of the tree.
            // Get a random number
            let rand: f64 = (rng(sims) as f64) / 2u32.pow(15) as f64;
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

            // They ansewr
            if rand > 0.5 {
                c += 1;
                let mut rng = rand::thread_rng();

                let t = -12f64 * (1.0f64 - rng.gen::<f64>()).ln(); // This is a continuous, exponential random variable.
                if t > 25f64 {
                    w += 26.0;
                    continue;
                } else {
                    w += t;
                    break;
                }
            }
        }

        // Now we have W. We put this in the global array of W's.
        w_final[sim] = w;
    }
    // Now, we return the array
    return w_final;
}

fn q1(result: &[f64]) -> f64 {
    // Takes an f64 array
    let end = result.len() / 2; // End index
    let start = result.len() / 4; // Start index
    let mut mean: f64 = 0.0;
    for val in result[start..end].iter() {
        mean += val;
    }
    let avg = mean / ((end - start) as f64);
    return avg;
}

fn q3(result: &[f64]) -> f64 {
    // Takes an f64 array
    let end = result.len() / 2 + result.len() / 4; // End index
    let start = result.len() / 2; // Start index
    let mut mean: f64 = 0.0;
    for val in result[start..end].iter() {
        mean += val;
    }
    let avg = mean / ((end - start) as f64);
    return avg;
}

fn mean(result: &[f64]) -> f64 {
    let mut mean: f64 = 0.0;
    for val in result[0..result.len()].iter() {
        mean += val;
    }
    mean = mean / (result.len() as f64);
    return mean;
}

fn main() {
    let mut result: [f64; 1000] = simulate();
    result.sort_by(|a, b| a.partial_cmp(b).unwrap()); // Sorts array inplace
    let q1 = q1(&result);
    println!("Q1: {}", q1);
    let q3 = q3(&result);
    println!("Q3: {}", q3);
    let mean = mean(&result);
    println!("Mean: {}", mean);
    let median = result[500];
    println!("Median: {}", median);
    let mut w15 = 0.0;
    let mut w20 = 0.0;
    let mut w30 = 0.0;
    let mut w40 = 0.0;
    let mut w5 = 0.0;
    let mut w6 = 0.0;
    let mut w7 = 0.0;

    // the first four
    for val in result.iter() {
        if val <= &15.0 {
            w15 += 1.0;
        }
        if val <= &20.0 {
            w20 += 1.0;
        }
        if val <= &30.0 {
            w30 += 1.0;
        }
        if val > &40.0 {
            w40 += 1.0;
        }
        if val > &45.0 {
            w5 += 1.0;
        }
        if val > &55.0 {
            w6 += 1.0;
        }
        if val > &70.0 {
            w7 += 1.0;
        }
    }
    w15 /= 1000.0;
    w20 /= 1000.0;
    w30 /= 1000.0;
    w40 /= 1000.0;
    w5 /= 1000.0;
    w6 /= 1000.0;
    w7 /= 1000.0;

    println!("W15: {}", w15);
    println!("W20: {}", w20);
    println!("W30: {}", w30);
    println!("W40: {}", w40);
    println!("W5: {}", w5);
    println!("W6: {}", w6);
    println!("W7: {}", w7);
    let mut output = String::new();
    for val in result.iter() {
        let s: String = val.to_string() + "\n";
        output.push_str(&s);
    }
    let mut file = std::fs::File::create("results.txt").expect("Create failed");
    file.write_all(output.as_bytes()).expect("Write failed");
    println!("Data written to file results.txt in the the current working dir");
}
