//! # Fridge Module
//!
//! This module implements the kitchen's refrigerator, which stores ingredients and manages
//! the asynchronous dough fermentation process. The fridge demonstrates several important
//! async programming concepts:
//!
//! - Resource management (ingredients are stored and retrieved)
//! - Long-running background tasks (fermentation process)
//! - One-shot channels for communicating between tasks
//! - Async task spawning for concurrent operations
//!
//! The fridge is protected by a mutex in the Kitchen to prevent race conditions during
//! concurrent access by multiple chefs.

use std::collections::HashMap;
use tokio::sync::oneshot;
use tokio::time::sleep;

use crate::equipment::TIME_TO_FERMENT_DOUGH_WITH_FRIDGE;
use crate::ingredients::{Cheese, Dough, Flour, Tomato};
use crate::PizzaError;

/// # Fridge
///
/// Represents a refrigerator that stores ingredients and manages dough fermentation.
///
/// The fridge maintains inventories of basic ingredients (flour, tomatoes, cheese) and
/// provides methods to retrieve them. It also manages the dough fermentation process,
/// which runs asynchronously in the background.
///
/// In a real kitchen, dough fermentation is a time-consuming process that can run for
/// many hours. This is simulated using asynchronous tasks that run independently and
/// notify completion via channels.
///
/// # Examples
///
/// ```
/// use life_of_pie::fridge::Fridge;
/// use life_of_pie::ingredients::{Flour, Tomato, Cheese, Dough};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a new fridge with some ingredients
///     let mut fridge = Fridge::new(
///         vec![Flour, Flour], // 2 portions of flour
///         vec![Tomato::Fresh, Tomato::Fresh], // 2 tomatoes
///         vec![Cheese::Fresh] // 1 block of cheese
///     );
///
///     // Get ingredients
///     let flour = fridge.get_flour()?;
///
///     // Start fermenting some dough (assume we have mixed dough)
///     let mixed_dough = Dough::Mixed;
///     let fermentation_receiver = fridge.leave_dough_to_ferment(mixed_dough, 1).await;
///
///     // Later, we can await the fermented dough
///     let fermented_dough = fermentation_receiver.await?;
///     assert_eq!(fermented_dough, Dough::Fermented);
///
///     Ok(())
/// }
/// ```
pub struct Fridge {
    /// Available portions of flour for making dough
    pub portions_of_flour: Vec<Flour>,

    /// Storage for fermented dough ready to be proved
    pub fermented_dough: Vec<Dough>,

    /// Fresh tomatoes available for sauce
    pub tomatoes: Vec<Tomato>,

    /// Blocks of cheese available for grating
    pub block_of_cheese: Vec<Cheese>,

    /// Tracking of active fermentation processes by station ID
    /// The HashMap tracks which stations have dough fermenting but doesn't store
    /// the actual channels to avoid ownership complications
    fermenting_processes: HashMap<usize, ()>,
}

impl Fridge {
    /// Creates a new fridge with the specified ingredients.
    ///
    /// # Arguments
    /// * `flour` - A collection of flour portions
    /// * `tomatoes` - A collection of tomatoes
    /// * `cheese` - A collection of cheese blocks
    ///
    /// # Returns
    /// A new `Fridge` instance with the specified ingredients and empty fermented dough storage
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::fridge::Fridge;
    /// use life_of_pie::ingredients::{Flour, Tomato, Cheese};
    ///
    /// let fridge = Fridge::new(
    ///     vec![Flour, Flour], // 2 portions of flour
    ///     vec![Tomato::Fresh, Tomato::Fresh], // 2 tomatoes
    ///     vec![Cheese::Fresh] // 1 block of cheese
    /// );
    /// ```
    pub fn new(flour: Vec<Flour>, tomatoes: Vec<Tomato>, cheese: Vec<Cheese>) -> Self {
        Fridge {
            portions_of_flour: flour,
            fermented_dough: Vec::new(),
            tomatoes,
            block_of_cheese: cheese,
            fermenting_processes: HashMap::new(),
        }
    }

    /// Retrieves a portion of flour from the fridge.
    ///
    /// # Returns
    /// * `Ok(Flour)` - A portion of flour if available
    /// * `Err(PizzaError::OutOfFlour)` - If there's no flour left
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::fridge::Fridge;
    /// use life_of_pie::ingredients::Flour;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut fridge = Fridge::new(vec![Flour], vec![], vec![]);
    ///     let flour = fridge.get_flour()?;
    ///     
    ///     // If we try again, we'll get an error
    ///     let result = fridge.get_flour();
    ///     assert!(result.is_err());
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub fn get_flour(&mut self) -> Result<Flour, PizzaError> {
        // Check if we have flour available
        if self.portions_of_flour.is_empty() {
            return Err(PizzaError::OutOfFlour);
        }

        // Return a portion of flour
        Ok(self.portions_of_flour.pop().unwrap())
    }

    /// Starts the asynchronous fermentation process for dough.
    ///
    /// This method demonstrates a key async programming pattern: starting a long-running
    /// background task and providing a way for the caller to be notified when it completes.
    /// The caller receives a `oneshot::Receiver` which will receive the fermented dough
    /// once the fermentation process completes.
    ///
    /// # Arguments
    /// * `dough` - The dough to ferment, must be in the Mixed state
    /// * `station_id` - An identifier for the station that's fermenting this dough
    ///
    /// # Returns
    /// A `oneshot::Receiver` that will receive the fermented dough when ready
    ///
    /// # Panics
    /// Panics if the dough is not in the Mixed state
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::fridge::Fridge;
    /// use life_of_pie::ingredients::Dough;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let mut fridge = Fridge::new(vec![], vec![], vec![]);
    ///     let mixed_dough = Dough::Mixed;
    ///     let station_id = 1;
    ///
    ///     // Start the fermentation process
    ///     let receiver = fridge.leave_dough_to_ferment(mixed_dough, station_id).await;
    ///
    ///     // Later, wait for the fermentation to complete
    ///     let fermented_dough = receiver.await?;
    ///     assert_eq!(fermented_dough, Dough::Fermented);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn leave_dough_to_ferment(
        &mut self,
        dough: Dough,
        station_id: usize,
    ) -> oneshot::Receiver<Dough> {
        // Validate the dough state before proceeding
        if dough != Dough::Mixed {
            panic!("You can only ferment Mixed Dough!")
        }

        // Create a channel for this fermentation process
        let (tx, rx) = oneshot::channel();

        // Track that this station has started fermentation
        self.fermenting_processes.insert(station_id, ());

        // Spawn a task to ferment the dough asynchronously
        tokio::spawn(async move {
            // Wait for the fermentation time
            sleep(TIME_TO_FERMENT_DOUGH_WITH_FRIDGE).await;

            // Return the fermented dough through the channel
            // If the receiver was dropped, that's fine, the dough is lost
            tx.send(Dough::Fermented).unwrap_or_else(|_| {
                println!("❌ Fermentation process for station {} Failed", station_id);
            });
        });

        // Return the receiver so the chef can await the fermented dough
        rx
    }
}
