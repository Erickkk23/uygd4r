//! # Kitchen Module
//!
//! This module defines the central kitchen environment where all pizza-making occurs.
//! The kitchen represents a shared workspace with multiple resources that must be
//! properly synchronized to prevent race conditions and deadlocks.
//!
//! The kitchen demonstrates several key concepts in async Rust programming:
//! - Resource management with `Mutex` and `RwLock` for safe concurrent access
//! - Semaphores for limiting concurrent access to scarce resources
//! - Proper structuring of a shared environment for parallel tasks
//!
//! The kitchen contains all equipment needed for pizza production:
//! - Fridge: Stores ingredients and fermented dough
//! - Proving room: Specialized area for dough rising
//! - Ovens: For cooking pizzas (limited resource)
//! - Mixers: For kneading dough and pureeing ingredients (limited resource)
//! - Tables: Multi-purpose workspaces for various preparation tasks

use crate::equipment::{Mixer, Oven, Table};
use crate::fridge::Fridge;
use crate::ingredients::{Cheese, Flour, Tomato};
use crate::proving_room::ProvingRoom;
use tokio::sync::{Mutex, RwLock};

/// # Kitchen
///
/// The central environment containing all shared resources for pizza preparation.
/// This models a shared workspace that multiple concurrent actors (chefs) access simultaneously.
///
/// ## Key Concurrency Concepts Demonstrated:
/// * **Shared state management**: Safely sharing mutable resources across threads
/// * **Synchronization primitive selection**: Choosing the right tool for different access patterns
/// * **Deadlock prevention**: Organizing resources to prevent circular wait conditions
/// * **Concurrency granularity**: Balancing fine vs. coarse-grained synchronization
///
/// ## Synchronization Primitives Used:
/// * **`Mutex`**: For resources that need exclusive access (fridge, ovens, mixers)
///   - Provides mutual exclusion - only one task can access at a time
///   - Best for resources that are primarily modified and rarely just read
///
/// * **`RwLock`**: For resources that can have multiple readers but exclusive writers (tables)
///   - Allows multiple concurrent readers OR a single writer
///   - More efficient than Mutex when read operations are common
///
/// * **`Semaphore`**: For limiting the number of concurrent operations (proving trays)
///   - Represents a pool of permits that can be acquired and released
///   - Limits how many operations can occur simultaneously without specifying which resources
///   - Useful for managing access to a limited number of identical resources
/// 
/// This structure demonstrates a fundamental concept in concurrent system design: 
/// selecting appropriate synchronization primitives based on the access patterns
/// of different resources. Understanding these patterns is key to building efficient
/// concurrent systems that avoid both race conditions and unnecessary blocking.
///
/// # Examples
///
/// ```rust
/// use life_of_pie::Kitchen;
/// use std::sync::Arc;
///
/// async fn use_kitchen() {
///     // Create a kitchen with default equipment and ingredients
///     let kitchen = Arc::new(Kitchen::default());
///     
///     // Access a table for preparation work
///     let table_guard = kitchen.tables[0].write().await;
///     
///     // Access the fridge to get ingredients
///     let fridge_guard = kitchen.fridge.lock().await;
/// }
/// ```
pub struct Kitchen {
    /// Mutex-protected refrigerator for storing ingredients and fermented dough.
    /// The mutex ensures exclusive access to prevent race conditions when retrieving or storing items.
    pub fridge: Mutex<Fridge>,

    /// Mutex-protected proving room where dough rises.
    /// The mutex ensures exclusive access to prevent race conditions when placing or retrieving dough.
    pub proving_room: Mutex<ProvingRoom>,

    /// Array of ovens, each protected by a mutex for exclusive access.
    /// Ovens can only process one pizza at a time, making them a constrained resource.
    pub ovens: [Mutex<Oven>; 2],

    /// Array of mixers, each protected by a mutex for exclusive access.
    /// Mixers can only process one ingredient at a time, making them a constrained resource.
    pub mixers: [Mutex<Mixer>; 3],

    /// Array of tables protected by RwLock for shared/exclusive access.
    /// Tables can be read from by multiple chefs simultaneously, but require exclusive access for modifications.
    pub tables: [RwLock<Table>; 4],
}

impl Kitchen {
    /// Creates a new Kitchen with default equipment and ingredients.
    ///
    /// This method initializes:
    /// - A fridge stocked with the specified ingredients
    /// - An empty proving room with the specified number of trays
    /// - A set of ovens for cooking pizzas
    /// - A set of mixers for kneading dough and pureeing ingredients
    /// - A set of tables for various preparation tasks
    /// - A semaphore to control access to proving trays
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::Kitchen;
    ///
    /// let kitchen = Kitchen::default();
    /// assert_eq!(kitchen.ovens.len(), 2);
    /// assert_eq!(kitchen.mixers.len(), 3);
    /// assert_eq!(kitchen.tables.len(), 4);
    /// ```
    pub fn default() -> Self {
        // Number of proving trays (same as number of tables)
        let num_proving_trays = 4;

        // Create initial ingredient collections
        let flour = (0..100).map(|_| Flour).collect();
        let tomatoes = (0..200).map(|_| Tomato::Fresh).collect();
        let cheese = (0..300).map(|_| Cheese::Fresh).collect();

        Kitchen {
            fridge: Mutex::new(Fridge::new(flour, tomatoes, cheese)),
            proving_room: Mutex::new(ProvingRoom::new(num_proving_trays)),
            ovens: [Mutex::new(Oven), Mutex::new(Oven)],
            mixers: [Mutex::new(Mixer), Mutex::new(Mixer), Mutex::new(Mixer)],
            tables: [
                RwLock::new(Table::default()),
                RwLock::new(Table::default()),
                RwLock::new(Table::default()),
                RwLock::new(Table::default()),
            ],
        }
    }
}
