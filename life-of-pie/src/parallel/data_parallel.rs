//! # Data Parallel Pizza Production
//!
//! This module implements the "data parallel" approach to pizza production, where each
//! chef independently makes entire pizzas from start to finish, working in parallel.
//! This approach demonstrates the concept of data parallelism - multiple workers
//! performing the same operations on different pieces of data.
//!
//! ## Key Concepts Demonstrated
//!
//! * Data parallelism - multiple chefs executing the same workflow in parallel
//! * Resource contention - chefs competing for shared kitchen equipment
//! * Task coordination - managing concurrent access to shared resources
//! * Error handling in parallel operations
//!
//! ## Implementation Guide
//!
//! Your task is to implement the `data_parallel_production` function that coordinates
//! multiple chefs working independently to produce pizzas. Follow these steps:
//!
//! 1. **Create worker tasks**: Spawn a separate task for each chef
//!
//! 2. **Implement the pizza-making workflow**: Each chef should:
//!    - Create hydrated dough
//!    - Develop (mix) the dough
//!    - Ferment the dough
//!    - Prove the dough
//!    - Prepare the pizza base
//!    - Prepare toppings (sauce and cheese) - consider making these concurrent!
//!    - Assemble the pizza
//!    - Cook the pizza
//!
//! 3. **Handle resource constraints**:
//!    - Kitchen equipment is limited and shared
//!    - Use proper synchronization to handle resource contention
//!    - Consider using tokio's synchronization primitives
//!
//! 4. **Collect results and report statistics**:
//!    - Track successful/failed pizzas
//!    - Return the counts as a tuple (successful_pizzas, failed_pizzas)
//!    - Report overall production statistics
//!
//! ## Hints
//!
//! * The `tokio::spawn` function creates new asynchronous tasks
//! * Remember that the `Chef` struct already has methods for each step
//! * You'll need to handle errors at each step in the workflow
//! * The `futures::future::join_all` function can help collect results from multiple tasks
//! stored in a vector
use crate::{Chef, Kitchen, PizzaError};
use futures::future::join_all;
use std::sync::Arc;

/// # Data Parallel Production
///
/// This function implements a pure data-parallel approach to pizza production, where
/// each chef independently makes exactly one pizza from start to finish. This contrasts
/// with the task-parallel approach where work is divided by specialization.
///
/// ## Key Concurrency Concepts Demonstrated:
/// * **Data Parallelism**: The same operations are performed on different data items (pizzas) in parallel
/// * **Independent Workers**: Each worker (chef) operates independently with minimal coordination
/// * **Resource Contention**: Workers compete for shared resources (kitchen equipment)
/// * **Task Spawning**: Creating multiple concurrent tasks using Tokio's runtime
/// * **Future Joining**: Collecting results from multiple asynchronous tasks
///
/// ## Implementation Details:
/// 1. We create exactly one chef per pizza to be made
/// 2. Each chef makes their entire pizza from start to finish without sharing work
/// 3. Each chef operates in their own thread of execution
/// 4. Results are collected and statistics computed asynchronously
///
/// Data parallelism is often simpler to reason about than task parallelism because
/// there are fewer dependencies between workers. However, it may lead to more
/// resource contention as all workers need access to all types of equipment.
///
/// This pattern is analogous to:
/// - Map operations in MapReduce frameworks
/// - Parallel array processing in GPU computing
/// - SIMD (Single Instruction, Multiple Data) operations
pub async fn data_parallel_production(
    kitchen_org: &Arc<Kitchen>,
    num_pizzas: usize,
) -> (usize, usize) {
    // Create a vector to hold the tasks for each chef
    // Each chef will be responsible for making one pizza

    // Spawn a task for each chef
        // Clone the kitchen reference for each chef
        // Each chef is identified by the pizza they're making
        // let chef_id = pizza_id; // useful for debugging

        // Spawn a new task (thread) for each chef (Use tokio::spawn not std::thread!)
            // Save reference to one chef

            // === STEP 1: Create hydrated dough ===

            // === STEP 2: Develop (mix) the dough ===

            // === STEP 3: Ferment the dough ===

            // === STEP 4: Prove the dough ===

            // === STEP 5: Prepare pizza base ===

            // === STEP 6 & 7: Prepare sauce and cheese ===

            // === STEP 8: Assemble the pizza ===

            // === STEP 9: Cook the pizza ===

        // Push the task into the vector of tasks

    // Wait for all chefs to complete their pizzas

    // Count successful and failed pizzas

    // Return the counts of successful and failed pizzas
    // This is a tuple (successful, failed)
    todo!(
        "Implement data_parallel_production function to coordinate multiple chefs making pizzas"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_data_parallel_production_1() {
        let kitchen = Arc::new(Kitchen::default());
        let (s, f) = data_parallel_production(&kitchen, 1).await;
        assert_eq!(s, 1);
        assert_eq!(f, 0);
    }

    #[tokio::test]
    async fn test_data_parallel_production_2() {
        let kitchen = Arc::new(Kitchen::default());
        let (s, f) = data_parallel_production(&kitchen, 2).await;
        assert_eq!(s, 2);
        assert_eq!(f, 0);
    }

    #[tokio::test]
    async fn test_data_parallel_production_5() {
        let kitchen = Arc::new(Kitchen::default());
        let (s, f) = data_parallel_production(&kitchen, 5).await;
        assert_eq!(s, 5);
        assert_eq!(f, 0);
    }

    #[tokio::test]
    async fn test_data_parallel_production_100() {
        let kitchen = Arc::new(Kitchen::default());
        let (s, f) = data_parallel_production(&kitchen, 100).await;
        assert_eq!(s, 100);
        assert_eq!(f, 0);
    }

    #[tokio::test]
    async fn test_data_parallel_production_not_enough_resources() {
        let kitchen = Arc::new(Kitchen::default());
        let (s, f) = data_parallel_production(&kitchen, 101).await;
        assert_eq!(s, 100);
        assert_eq!(f, 1);
    }
}
