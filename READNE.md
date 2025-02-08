# **Adaptive Payment Connector Selection Using Non-Stationary Multi-Armed Bandits**

## **Problem Statement**
In a payment system, we need to dynamically choose the best **payment connector** (e.g., Stripe, PayPal, Adyen) to maximize **successful transactions**. Each connector has a **success rate (SR)** that changes over time due to factors like:
- Network performance fluctuations
- Fraud detection policies
- Regulatory changes
- Seasonal traffic variations

This makes the problem **non-stationary**, meaning past success rates may not reflect future performance.

## **Solution Approach: Non-Stationary Multi-Armed Bandits**
A **multi-armed bandit (MAB)** framework helps balance **exploration (trying different connectors)** and **exploitation (choosing the best one)**. Since success rates change over time, we use **non-stationary bandit algorithms** like:
- **Sliding Window UCB / Thompson Sampling** (keeps recent data only)
- **Discounted UCB / Thompson Sampling** (weights recent rewards more)
- **Bayesian Change-Point Detection + Bandits** (detects shifts and resets estimates)

We focus on **Discounted Thompson Sampling (DTS)**, as it efficiently adapts to changes.

## **Modeling the Problem Using Beta Distribution**
Each connector \( i \) has an **unknown success rate \( p_i \)**, modeled as a **Beta distribution**:
$$
p_i \sim \text{Beta}(\alpha_i, \beta_i)
$$
Where:
- (alpha_i ) → Number of **successful transactions** for connector \( i \)
- (beta_i ) → Number of **failed transactions** for connector \( i \)

### **Updating the Beta Distribution**
After each transaction:
- If **successful**:  
  $$
  \alpha_i \leftarrow \gamma \alpha_i + 1
  $$
- If **failed**:  
  $$
  \beta_i \leftarrow \gamma \beta_i + 1
  $$
- \( \gamma \) is a **discount factor** (0 < \(gamma) < 1) that reduces the weight of older data.

## **Choosing the Best Payment Connector (Thompson Sampling)**
1. **Sample a success rate** for each connector:
   $$
   \hat{p}_i \sim \text{Beta}(\alpha_i, \beta_i)
   $$
2. **Pick the connector with the highest sampled value**.

This ensures that:
- **High-success connectors get chosen more often**.
- **Uncertain connectors get explored occasionally**.

## **Choosing the Discount Factor (\( \gamma \))**
| Discount Factor \( \gamma \) | Adaptation Speed | Best Use Case |
|------------------|---------------------|------------------------------|
| **1.0** | No forgetting | Use when success rates are mostly stable |
| **0.99 - 0.95** | Slow forgetting | Good for gradual changes over days/weeks |
| **0.9 - 0.8** | Fast forgetting | Works well for hourly/daily changes |
| **< 0.7** | Very aggressive forgetting | Use for highly volatile environments (rare) |

## **Final Takeaway**
By using **Discounted Thompson Sampling**, we can dynamically select the best payment connector, adapting to **changing success rates** in real-time. This method efficiently balances **exploration vs. exploitation** while ensuring optimal transaction success.