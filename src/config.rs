// src/config.rs

/// Represents a payment connector with its success rate and transaction history.
#[derive(Debug, Clone)]
pub struct PaymentConnector {
    pub name: String,
    pub alpha: f64, // Successes (for Thompson Sampling)
    pub beta: f64,  // Failures (for Thompson Sampling)
    pub successes: usize, // Successes (for Sliding Window UCB)
    pub attempts: usize,   // Attempts (for Sliding Window UCB)
    pub window: Vec<bool>, // Sliding window of transaction outcomes
}

impl PaymentConnector {
    /// Create a new payment connector.
    pub fn new(name: String, window_size: usize) -> Self {
        PaymentConnector {
            name,
            alpha: 1.0, // Initialize with 1 to avoid division by zero
            beta: 1.0,
            successes: 0,
            attempts: 0,
            window: Vec::with_capacity(window_size),
        }
    }
}

/// Trait for payment routing algorithms.
pub trait RoutingAlgorithm {
    fn select_connector(&self, connectors: &mut Vec<PaymentConnector>) -> usize;
    fn update_connector(&mut self, connectors: &mut Vec<PaymentConnector>, connector_index: usize, success: bool);
}