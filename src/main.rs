// src/main.rs

mod config;
mod thompson_sampling;
mod sliding_window_ucb;
mod utils;

use rand::Rng;
use std::env;
use config::{PaymentConnector, RoutingAlgorithm};
use thompson_sampling::ThompsonSampling;
use sliding_window_ucb::SlidingWindowUCB;
use utils::{log_connectors, print_separator};

fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Available algorithms: thompson, ucb");
        eprintln!("Run cargo run -- thompson/cargo run -- ucb");
        return;
    }

    // Initialize payment connectors
    let mut connectors = vec![
        PaymentConnector::new("Stripe".to_string(), 5),
        PaymentConnector::new("PayPal".to_string(), 5),
        PaymentConnector::new("Adyen".to_string(), 5),
        PaymentConnector::new("RazorPay".to_string(), 5),
        PaymentConnector::new("PayU".to_string(), 5),
    ];

    // Choose algorithm based on user input
    let algorithm_name = &args[1];
    let mut algorithm: Box<dyn RoutingAlgorithm> = match algorithm_name.as_str() {
        "thompson" => Box::new(ThompsonSampling::new(0.95)), // Discount factor = 0.95
        "ucb" => Box::new(SlidingWindowUCB::new(5, 2.0)),     // Window size = 5, exploration factor = 2.0
        _ => {
            eprintln!("Invalid algorithm. Available options: thompson, ucb");
            return;
        }
    };

    // Simulate transactions
    let num_transactions = 15;
    let mut rng = rand::thread_rng();
    let mut total_successes = 0;
    let mut total_attempts = 0;

    for i in 1..=num_transactions {
        print_separator();
        println!("Transaction {}:", i);

        // Select the best connector
        let connector_index = algorithm.select_connector(&mut connectors);
        let connector_name = connectors[connector_index].name.clone();

        // Simulate transaction success (80% success rate)
        let success = rng.gen_bool(0.8);
        algorithm.update_connector(&mut connectors, connector_index, success);

        // Update overall success rate
        if success {
            total_successes += 1;
        }
        total_attempts += 1;

        // Print results
        println!("Selected Connector: {}", connector_name);
        println!("Transaction Outcome: {}", if success { "Success" } else { "Failure" });

        // Log the current state of all connectors
        log_connectors(&connectors);
    }

    // Print overall success rate
    print_separator();
    println!("Overall Success Rate: {:.2}%", (total_successes as f64 / total_attempts as f64) * 100.0);
}