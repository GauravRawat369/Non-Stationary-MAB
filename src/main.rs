use rand_distr::{Distribution, Beta};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
struct Connector {
    alpha: f64,
    beta: f64,
    success_rate_history: Vec<f64>,
}

impl Connector {
    fn new() -> Self {
        Self {
            alpha: 1.0,
            beta: 1.0,
            success_rate_history: Vec::new(),
        }
    }

    fn sample_success_rate(&self) -> f64 {
        let beta_dist = Beta::new(self.alpha, self.beta).unwrap();
        beta_dist.sample(&mut rand::thread_rng())
    }

    fn update(&mut self, success: bool, gamma: f64, window_size: usize) {
        // Use Discounted Thompson Sampling (DTS): Apply exponential discounting to forget old data
        if success {
            self.alpha = gamma * self.alpha + 1.0;
        } else {
            self.beta = gamma * self.beta + 1.0;
        }
        
        // Experiment with Sliding Window Thompson Sampling: Store success rate history and apply sliding window
        let current_sr = self.alpha / (self.alpha + self.beta);
        self.success_rate_history.push(current_sr);
        if self.success_rate_history.len() > window_size {
            self.success_rate_history.remove(0);
        }
    }

    fn detect_change(&self, threshold: f64) -> bool {
        // Introduce a Change-Point Detection Mechanism: Detect sudden performance drops
        if self.success_rate_history.len() < 2 {
            return false;
        }
        let recent_avg: f64 = self.success_rate_history.iter().sum::<f64>()
            / self.success_rate_history.len() as f64;
        let last_value = *self.success_rate_history.last().unwrap();
        (last_value - recent_avg).abs() > threshold
    }
}

fn select_best_connectors(
    connectors: &mut HashMap<String, Connector>,
    top_n: usize,
) -> Vec<String> {
    // Select best connectors based on Thompson Sampling
    let mut sampled_connectors: Vec<(&String, f64)> = connectors
        .iter()
        .map(|(name, connector)| (name, connector.sample_success_rate()))
        .collect();
    
    sampled_connectors.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    sampled_connectors.iter().take(top_n).map(|(name, _)| (*name).clone()).collect()
}

fn main() {
    let mut connectors: HashMap<String, Connector> = vec![
        "Stripe".to_string(),
        "PayPal".to_string(),
        "Adyen".to_string(),
        "razorpay".to_string(),
        "payU".to_string(),
    ]
    .into_iter()
    .map(|name| (name, Connector::new()))
    .collect();

    let gamma = 0.95;
    let window_size = 5;
    let change_threshold = 0.2;
    let mut rng = rand::thread_rng();
    
    for iteration in 0..1500 {
        let selected_connectors = select_best_connectors(&mut connectors, 3);
        let chosen = selected_connectors[0].clone();
        let success = rng.gen_bool(0.8); // Simulating an 80% success rate on transactions

        {
            let connector = connectors.get_mut(&chosen).unwrap();
            connector.update(success, gamma, window_size);
        }

        // Print only the success rate of the selected connector
        let sr = connectors.get(&chosen).unwrap().alpha / 
            (connectors.get(&chosen).unwrap().alpha + connectors.get(&chosen).unwrap().beta);
       println!("Iteration {}: {}'s Success Rate: {:.2}%", iteration + 1, chosen, (sr*100.0));

        
        if connectors.get(&chosen).unwrap().detect_change(change_threshold) {
            println!("Detected significant change in {}'s success rate!", chosen);
            // Adaptive Exploration: Re-test eliminated connectors to detect improvements
        }
    }
    
    let final_best_connectors = select_best_connectors(&mut connectors, 3);
    println!("Final Top 3 Payment Connectors: {:?}", final_best_connectors);
}
