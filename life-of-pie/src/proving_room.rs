//! # Proving Room Module
//!
//! This module implements a specialized room for proving dough, a critical step in pizza-making
//! where the dough rises and develops its texture. The proving room demonstrates several important
//! async programming concepts:
//!
//! - Resource limitations with fixed-capacity trays
//! - Long-running operations with `sleep`
//! - Safe state transitions with proper validation
//! - Shared mutable state protected by synchronization primitives
//! - Asynchronous communication using oneshot channels
//!
//! In a real kitchen, proving dough takes significant time and requires dedicated space.
//! This process is modeled asynchronously to simulate the waiting time without blocking
//! other kitchen operations.

use crate::equipment::TIME_TO_PROVE_DOUGH_WITH_TABLE;
use crate::ingredients::Dough;
use std::sync::Arc;
use tokio::sync::{oneshot, Semaphore};
use tokio::time::sleep;

/// # Proving Room
///
/// A specialized area in the kitchen where dough is placed to rise (prove).
///
/// The proving room contains multiple trays, each capable of holding one piece of dough.
/// The proving process transforms fermented dough into proved dough, which is necessary
/// before shaping into a pizza base.
///
/// This structure demonstrates resource management in async programming:
/// - Fixed number of trays (limited resource)
/// - Process that takes time but doesn't require active attention
/// - Asynchronous communication using oneshot channels
pub struct ProvingRoom {
    /// Semaphore limiting the number of chefs who can prove dough simultaneously.
    /// We use Arc<Semaphore> so we can clone and share it across async tasks
    trays: Arc<Semaphore>,
}

impl ProvingRoom {
    /// Creates a new proving room with the specified number of trays.
    ///
    /// # Arguments
    /// * `num_trays` - The number of proving trays available
    ///
    /// # Returns
    /// A new `ProvingRoom` with the specified number of trays
    pub fn new(num_trays: usize) -> Self {
        ProvingRoom {
            trays: Arc::new(Semaphore::new(num_trays)),
        }
    }

    /// # Asynchronous Proving Process
    ///
    /// Starts the asynchronous proving process for dough, demonstrating several
    /// critical concurrency patterns in a single operation.
    ///
    /// ## Key Concurrency Concepts Demonstrated:
    /// * **Resource acquisition with semaphores**: Uses permits to control access to limited resources
    /// * **Asynchronous waiting**: Waits for a tray without blocking the thread
    /// * **Task spawning**: Creates a background task that runs independently
    /// * **Channel-based communication**: Uses oneshot channels to communicate completion
    /// * **RAII-based cleanup**: Automatically releases resources when the task completes
    ///
    /// ## Implementation Details:
    /// 1. Validates input state (dough must be fermented)
    /// 2. Creates a channel for returning the result
    /// 3. Acquires a semaphore permit (waits if no trays are available)
    /// 4. Spawns a background task to handle the proving process
    /// 5. The background task simulates proving time with sleep
    /// 6. When complete, sends the proved dough through the channel
    /// 7. Automatically releases the permit using Rust's drop system
    ///
    /// ## Educational Notes:
    /// This method demonstrates how Rust's ownership system and RAII principles
    /// work together with asynchronous code. The semaphore permit is automatically
    /// released when it goes out of scope (when the task completes), showing how
    /// Rust helps prevent resource leaks even in complex async scenarios.
    ///
    /// # Arguments
    /// * `dough` - The dough to prove, must be in the Fermented state
    ///
    /// # Returns
    /// A `oneshot::Receiver` that will receive the proved dough when ready
    ///
    /// # Errors
    /// Returns an error if the dough is not in the fermented state
    pub async fn leave_dough_to_prove(
        &self,
        dough: Dough,
    ) -> Result<oneshot::Receiver<Dough>, &'static str> {
        // Validate the dough state before proceeding
        if dough != Dough::Fermented {
            return Err("You can only prove Fermented Dough!");
        }

        // Create a channel for this proving process
        let (tx, rx) = oneshot::channel();

        // Clone the Arc<Semaphore> and acquire an owned permit
        // This will wait asynchronously if no trays are available
        let permit = match self.trays.clone().acquire_owned().await {
            Ok(permit) => permit,
            Err(_) => return Err("Failed to acquire a proving tray"),
        };

        // Spawn a task to prove the dough asynchronously
        // This no longer captures `self`
        tokio::spawn(async move {
            // Asynchronously wait for proving to complete
            sleep(TIME_TO_PROVE_DOUGH_WITH_TABLE).await;

            // Return the proved dough through the channel
            tx.send(Dough::Proved).unwrap_or_else(|_| {
                println!("❌ Proving process failed: receiver dropped");
            });

            // The permit is automatically dropped here, releasing the tray
            drop(permit);
        });

        // Return the receiver so the chef can await the proved dough
        Ok(rx)
    }
}
