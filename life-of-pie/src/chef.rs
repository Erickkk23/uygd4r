//! # Chef Module
//!
//! This module implements the chef who performs all pizza-making operations in the kitchen.
//! The chef coordinates between different equipment and ingredients, demonstrating advanced
//! patterns in asynchronous programming:
//!
//! - Resource acquisition with non-blocking waits
//! - Race-based resource selection (first available wins)
//! - Error handling and propagation with Result types
//! - Complex workflow orchestration with proper sequencing
//! - Concurrent task execution for independent operations
//!
//! The chef's methods represent a complete workflow for making pizza, from creating dough
//! to cooking the final product, with proper error handling at each step.

use futures::{future::select_all, FutureExt};
use tokio::sync::{MutexGuard, RwLock, RwLockWriteGuard};

use crate::error::PizzaError;
use crate::ingredients::{Cheese, Dough, Pizza, Tomato};
use crate::kitchen::Kitchen;

/// Type alias for our Result type with PizzaError
pub type PizzaResult<T> = Result<T, PizzaError>;

/// # Resource Acquisition Helper: Mutex
///
/// Acquires the first available mutex-protected resource from a collection.
///
/// This function demonstrates the "racing" pattern for resource acquisition,
/// where multiple futures race to acquire different resources, and the first
/// one to succeed is used. This approach minimizes waiting time in scenarios
/// where any resource from a pool will suffice.
///
/// # Arguments
/// * `mutex_protected_resources` - A slice of mutex-protected resources
///
/// # Returns
/// A MutexGuard for the first resource that was successfully locked
///
/// # Examples
///
/// ```
/// use life_of_pie::chef::mutex_acquire_one_of_these;
/// use tokio::sync::Mutex;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a collection of mutex-protected resources
///     let resources = [
///         Mutex::new("Resource 1"),
///         Mutex::new("Resource 2"),
///         Mutex::new("Resource 3"),
///     ];
///
///     // Acquire the first available resource
///     let guard = mutex_acquire_one_of_these(&resources).await;
///     println!("Acquired: {}", *guard);
/// }
/// ```
pub async fn mutex_acquire_one_of_these<T>(
    mutex_protected_resources: &[tokio::sync::Mutex<T>],
) -> MutexGuard<T>
where
    T: Send,
{
    // Create a collection of futures that will race to acquire locks
    let mut future_locks = Vec::new();
    for (index, mutex) in mutex_protected_resources.iter().enumerate() {
        // For each resource, create a future that will try to lock it
        // The future will resolve with the index of the resource once locked
        let future_lock = async move {
            let _ = mutex.lock().await;
            index
        }
        .fuse() // Mark the future as fuseable for select_all
        .boxed(); // Box the future for type erasure
        future_locks.push(future_lock);
    }

    // Race all futures and get the index of the first one to complete
    // This is an efficient way to acquire the first available resource
    let (acquired_index, _, _) = select_all(future_locks).await;

    // Finally acquire and return the lock for the winning resource
    mutex_protected_resources[acquired_index].lock().await
}

/// # Resource Acquisition Helper: RwLock
///
/// Acquires write access to the first available RwLock-protected resource from a collection.
///
/// This function is similar to `mutex_acquire_one_of_these`, but for RwLock resources
/// with write access. It uses the same racing pattern to minimize waiting time when
/// any resource from a pool will suffice.
///
/// # Arguments
/// * `rwlock_protected_resources` - A slice of RwLock-protected resources
///
/// # Returns
/// A RwLockWriteGuard for the first resource that was successfully locked for writing
///
/// # Examples
///
/// ```
/// use life_of_pie::chef::rwlock_modify_one_of_these;
/// use tokio::sync::RwLock;
///
/// #[tokio::main]
/// async fn main() {
///     // Create a collection of RwLock-protected resources
///     let resources = [
///         RwLock::new("Resource 1"),
///         RwLock::new("Resource 2"),
///         RwLock::new("Resource 3"),
///     ];
///
///     // Acquire write access to the first available resource
///     let mut guard = rwlock_modify_one_of_these(&resources).await;
///     *guard = "Modified Resource";
/// }
/// ```
pub async fn rwlock_modify_one_of_these<T>(
    rwlock_protected_resources: &[RwLock<T>],
) -> RwLockWriteGuard<T>
where
    T: Send + Sync,
{
    // Same pattern as mutex_acquire_one_of_these but for write locks
    let mut future_locks = Vec::new();
    for (index, rwlock) in rwlock_protected_resources.iter().enumerate() {
        let future_lock = async move {
            let _ = rwlock.write().await;
            index
        }
        .fuse()
        .boxed();
        future_locks.push(future_lock);
    }

    let (acquired_index, _, _) = select_all(future_locks).await;
    rwlock_protected_resources[acquired_index].write().await
}

/// # Chef
///
/// Represents a chef who performs all pizza-making operations in the kitchen.
///
/// The Chef struct is responsible for orchestrating the entire pizza-making process,
/// coordinating between different pieces of equipment and ingredients to create pizzas.
/// Each method represents a specific step in the process and demonstrates proper
/// resource acquisition, error handling, and asynchronous operation.
///
/// # Examples
///
/// ```
/// use std::sync::Arc;
/// use life_of_pie::{Chef, Kitchen};
/// use life_of_pie::ingredients::Pizza;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a kitchen with default equipment and ingredients
///     let kitchen = Arc::new(Kitchen::default());
///     
///     // Create a chef
///     let chef = Chef;
///     
///     // Make a pizza through all the steps
///     // In a real application, you'd handle all the intermediary steps and errors
///     
///     // Step 1: Create dough
///     let hydrated_dough = chef.create_dough(&kitchen).await?;
///     
///     // ... (other steps)
///     
///     // Final step would be cooking the pizza
///     // let cooked_pizza = chef.cook_pizza(&kitchen, uncooked_pizza).await?;
///     
///     Ok(())
/// }
/// ```
pub struct Chef;

impl Chef {
    /// # Create Dough
    ///
    /// Creates hydrated dough from flour.
    ///
    /// This method demonstrates proper resource acquisition:
    /// 1. Acquire a lock on the fridge to get flour
    /// 2. Release the fridge lock as soon as possible
    /// 3. Acquire a table to mix the flour into dough
    ///
    /// # Arguments
    /// * `kitchen` - Reference to the kitchen containing all necessary equipment and ingredients
    ///
    /// # Returns
    /// * `Ok(Dough::Hydrated)` - If dough was created successfully
    /// * `Err(PizzaError)` - If there's no flour or another error occurs
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::{Chef, Kitchen};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kitchen = Kitchen::default();
    ///     let chef = Chef;
    ///     
    ///     let dough = chef.create_dough(&kitchen).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn create_dough(&self, kitchen: &Kitchen) -> PizzaResult<Dough> {

        let mut fridge = kitchen.fridge.lock().await;
        let flour = fridge.portions_of_flour.pop().ok_or(PizzaError::OutOfFlour)?;
        drop(fridge); 
    
        let mut table = rwlock_modify_one_of_these(&kitchen.tables).await;
    
        let hydrated_dough = table.mix_flour_into_dough(flour).await?;
    
        Ok(hydrated_dough)
    }
    

    /// # Develop Dough
    ///
    /// Develops hydrated dough into mixed dough by kneading it in a mixer.
    ///
    /// This method demonstrates:
    /// 1. Input validation before acquiring resources
    /// 2. Resource acquisition for specialized equipment (mixer)
    /// 3. Error propagation from equipment operations
    ///
    /// # Arguments
    /// * `kitchen` - Reference to the kitchen containing necessary equipment
    /// * `dough_to_mix` - The dough to be developed, must be in Hydrated state
    ///
    /// # Returns
    /// * `Ok(Dough::Mixed)` - If dough was developed successfully
    /// * `Err(PizzaError::IncorrectDoughState)` - If dough is not in Hydrated state
    /// * `Err(PizzaError)` - If another error occurs during mixing
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::{Chef, Kitchen};
    /// use life_of_pie::ingredients::Dough;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kitchen = Kitchen::default();
    ///     let chef = Chef;
    ///     
    ///     // First create hydrated dough
    ///     let hydrated_dough = chef.create_dough(&kitchen).await?;
    ///     
    ///     // Then develop it into mixed dough
    ///     let mixed_dough = chef.develop_dough(&kitchen, hydrated_dough).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn develop_dough(
        &self,
        kitchen: &Kitchen,
        dough_to_mix: Dough,
    ) -> PizzaResult<Dough> {
        if dough_to_mix != Dough::Hydrated {
            return Err(PizzaError::IncorrectDoughState(
                "You can only put Hydrated Dough in the Mixer".to_string(),
            ));
        }

        let mixer = mutex_acquire_one_of_these(&kitchen.mixers).await;

        let mixed_dough = mixer.mix_dough(dough_to_mix).await?;

        Ok(mixed_dough)
    }


    /// # Ferment Dough
    ///
    /// Starts the fermentation process for mixed dough.
    ///
    /// This method demonstrates:
    /// 1. Input validation
    /// 2. Asynchronous background processing with oneshot channels
    /// 3. Optional station assignment for different processing models
    ///
    /// # Arguments
    /// * `kitchen` - Reference to the kitchen containing necessary equipment
    /// * `dough_to_ferment` - The dough to ferment, must be in Mixed state
    /// * `station_id` - Optional station ID for tracking in task parallel mode
    ///
    /// # Returns
    /// * `Ok(oneshot::Receiver<Dough>)` - A receiver that will provide the fermented dough when ready
    /// * `Err(PizzaError::IncorrectDoughState)` - If dough is not in Mixed state
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::{Chef, Kitchen};
    /// use life_of_pie::ingredients::Dough;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kitchen = Kitchen::default();
    ///     let chef = Chef;
    ///     
    ///     // Create mixed dough (assuming we've already created and developed dough)
    ///     let mixed_dough = Dough::Mixed; // In reality, this would come from develop_dough
    ///     
    ///     // Start fermentation and get a receiver
    ///     let receiver = chef.ferment_dough(&kitchen, mixed_dough, None).await?;
    ///     
    ///     // Later, wait for the fermentation to complete
    ///     let fermented_dough = receiver.await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn ferment_dough(
        &self,
        kitchen: &Kitchen,
        dough_to_ferment: Dough,
        station_id: Option<usize>,
    ) -> PizzaResult<tokio::sync::oneshot::Receiver<Dough>> {
        if dough_to_ferment != Dough::Mixed {
            return Err(PizzaError::IncorrectDoughState(
                "You can only ferment Mixed Dough!".to_string(),
            ));
        }
    
        let mut fridge = kitchen.fridge.lock().await;
    
        let receiver = fridge
            .leave_dough_to_ferment(dough_to_ferment, station_id.unwrap_or(0))
            .await?;
    
        Ok(receiver)
    }
    

    /// # Prove Dough
    ///
    /// Proves fermented dough in the proving room.
    ///
    /// This method demonstrates:
    /// 1. Input validation for dough state (from receiver)
    /// 2. Resource acquisition for proving trays
    /// 3. Long-running async operation (proving takes time)
    /// 4. Asynchronous result handling with oneshot channel
    ///
    /// # Arguments
    /// * `kitchen` - Reference to the kitchen containing the proving room
    /// * `fermented_dough_receiver` - Receiver that will provide the fermented dough
    ///
    /// # Returns
    /// * `Ok(oneshot::Receiver<Dough>)` - A receiver that will get the proved dough when ready
    /// * `Err(PizzaError)` - If an error occurs during proving
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::{Chef, Kitchen};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kitchen = Kitchen::default();
    ///     let chef = Chef;
    ///     
    ///     let hydrated_dough = chef.create_dough(&kitchen).await?;
    ///     let mixed_dough = chef.develop_dough(&kitchen, hydrated_dough).await?;
    ///     let ferment_receiver = chef.ferment_dough(&kitchen, mixed_dough, None).await?;
    ///     
    ///     let prove_receiver = chef.prove_dough(&kitchen, ferment_receiver).await?;
    ///     let proved_dough = prove_receiver.await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn prove_dough(
        &self,
        dough_receiver: tokio::sync::oneshot::Receiver<Dough>,
        kitchen: &Kitchen,
    ) -> PizzaResult<Dough> {
        let dough = dough_receiver.await.map_err(|_| {
            PizzaError::FermentationFailed("Fermentation process was dropped!".to_string())
        })?;
    
        if dough != Dough::Fermented {
            return Err(PizzaError::IncorrectDoughState(
                "Expected Fermented Dough!".to_string(),
            ));
        }
    
        let mut proving_room = rwlock_modify_one_of_these(&kitchen.proving_rooms).await;
    
        let proved_dough = proving_room.prove_dough(dough).await?;
    
        Ok(proved_dough)
    }
    

    /// # Prepare Pizza Base
    ///
    /// Prepares a pizza base from proved dough.
    ///
    /// This method demonstrates:
    /// 1. Proper resource acquisition for pizza base preparation
    /// 2. Error handling with proper validation
    /// 3. Direct processing of proved dough without polling
    ///
    /// # Arguments
    /// * `kitchen` - Reference to the kitchen containing necessary equipment
    /// * `proved_dough` - The proved dough to shape into a pizza base
    ///
    /// # Returns
    /// * `Ok(Dough::Rounded)` - If the pizza base was prepared successfully
    /// * `Err(PizzaError::IncorrectDoughState)` - If the dough is not properly proved
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::{Chef, Kitchen, Dough};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kitchen = Kitchen::default();
    ///     let chef = Chef;
    ///     
    ///     // Assuming we have proved dough
    ///     let proved_dough = Dough::Proved;
    ///     
    ///     let pizza_base = chef.prepare_pizza_base(&kitchen, proved_dough).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn prepare_pizza_base(
        &self,
        kitchen: &Kitchen,
        proved_dough: Dough,
    ) -> PizzaResult<Dough> {
        // Validate the dough state
        if dough != Dough::Proved {
            return Err(PizzaError::IncorrectDoughState(
                "Only Proved Dough can be used to create a Pizza Base!".to_string(),
            ));
        }
    
        // Acquire a roller
        let roller = mutex_acquire_one_of_these(&kitchen.rollers).await;
    
        // Flatten the dough into a pizza base
        let base = roller.flatten_dough_into_base(dough).await?;
    
        Ok(base)
    }
    

    /// # Puree Sauce
    ///
    /// Creates tomato sauce by pureeing a fresh tomato.
    ///
    /// This method demonstrates:
    /// 1. Error handling for missing ingredients
    /// 2. Resource acquisition for specialized equipment (mixer)
    ///
    /// # Arguments
    /// * `kitchen` - Reference to the kitchen containing necessary equipment and ingredients
    ///
    /// # Returns
    /// * `Ok(Tomato::Pureed)` - If the sauce was prepared successfully
    /// * `Err(PizzaError::OutOfTomatoes)` - If there are no tomatoes available
    /// * `Err(PizzaError)` - If another error occurs during pureeing
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::{Chef, Kitchen};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kitchen = Kitchen::default();
    ///     let chef = Chef;
    ///     
    ///     let tomato_sauce = chef.puree_sauce(&kitchen).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn puree_sauce(&self, kitchen: &Kitchen) -> PizzaResult<Sauce> {
        // Step 1: Access the fridge to get tomatoes
        let mut fridge = kitchen.fridge.lock().await;
        let tomatoes = fridge.tomatoes.pop().ok_or(PizzaError::OutOfTomatoes)?;
        drop(fridge); // release fridge lock
    
        // Step 2: Acquire a blender
        let blender = mutex_acquire_one_of_these(&kitchen.blenders).await;
    
        // Step 3: Puree the tomatoes
        let sauce = blender.puree_tomatoes_into_sauce(tomatoes).await?;
    
        Ok(sauce)
    }
    

    /// # Shred Cheese
    ///
    /// Shreds a block of cheese for pizza topping.
    ///
    /// This method demonstrates:
    /// 1. Error handling for missing ingredients
    /// 2. Resource acquisition for multi-purpose equipment (table)
    ///
    /// # Arguments
    /// * `kitchen` - Reference to the kitchen containing necessary equipment and ingredients
    ///
    /// # Returns
    /// * `Ok(Cheese::Grated)` - If the cheese was shredded successfully
    /// * `Err(PizzaError::OutOfCheese)` - If there's no cheese available
    /// * `Err(PizzaError)` - If another error occurs during shredding
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::{Chef, Kitchen};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kitchen = Kitchen::default();
    ///     let chef = Chef;
    ///     
    ///     let grated_cheese = chef.shred_cheese(&kitchen).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn shred_cheese(&self, kitchen: &Kitchen) -> PizzaResult<ShreddedCheese> {
        // Step 1: Access the fridge to get cheese
        let mut fridge = kitchen.fridge.lock().await;
        let cheese = fridge.cheeses.pop().ok_or(PizzaError::OutOfCheese)?;
        drop(fridge); // release fridge lock
    
        // Step 2: Acquire a grater
        let grater = mutex_acquire_one_of_these(&kitchen.graters).await;
    
        // Step 3: Shred the cheese
        let shredded = grater.shred_cheese(cheese).await?;
    
        Ok(shredded)
    }
    

    /// # Combine Ingredients to Form Pizza
    ///
    /// Combines pizza base, tomato sauce, and cheese to form an uncooked pizza.
    ///
    /// This method demonstrates:
    /// 1. Complex input validation for multiple ingredients
    /// 2. Resource acquisition for final assembly
    ///
    /// # Arguments
    /// * `kitchen` - Reference to the kitchen containing necessary equipment
    /// * `pizza_base` - The prepared pizza base, must be in Rounded state
    /// * `tomato_sauce` - The tomato sauce, must be in Pureed state
    /// * `cheese` - The cheese, must be in Grated state
    ///
    /// # Returns
    /// * `Ok(Pizza::Uncooked)` - If the pizza was assembled successfully
    /// * `Err(PizzaError::IncorrectIngredientState)` - If any ingredient is not in the correct state
    /// * `Err(PizzaError)` - If another error occurs during assembly
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::{Chef, Kitchen};
    /// use life_of_pie::ingredients::{Dough, Tomato, Cheese};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kitchen = Kitchen::default();
    ///     let chef = Chef;
    ///     
    ///     // Assuming we have prepared all the ingredients
    ///     let pizza_base = Dough::Rounded;
    ///     let tomato_sauce = Tomato::Pureed;
    ///     let cheese = Cheese::Grated;
    ///     
    ///     let uncooked_pizza = chef.combine_cold_ingredients_to_form_uncooked_pizza(
    ///         &kitchen, pizza_base, tomato_sauce, cheese
    ///     ).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn combine_cold_ingredients_to_form_uncooked_pizza(
        &self,
        kitchen: &Kitchen,
        pizza_base: Dough,
        tomato_sauce: Tomato,
        cheese: Cheese,
    ) -> PizzaResult<Pizza> {
        // Validate all ingredient states before proceeding
        if pizza_base != Dough::Rounded || tomato_sauce != Tomato::Pureed || cheese != Cheese::Grated {
            return Err(PizzaError::IncorrectIngredientState(
                "You can only combine Pureed Tomato, Grated Cheese, and Rounded Dough to form an Uncooked Pizza!".to_string(),
            ));
        }
    
        // Acquire a table to assemble the pizza
        let _table = rwlock_modify_one_of_these(&kitchen.tables).await;
    
        // Propagate errors from top_pizza_base_with_ingredients and return the result
        Pizza::top_pizza_base_with_ingredients(pizza_base, tomato_sauce, cheese)
    }
    
    
    /// # Cook Pizza
    ///
    /// Cooks an uncooked pizza in an oven.
    ///
    /// This method demonstrates:
    /// 1. Input validation for pizza state
    /// 2. Resource acquisition for specialized equipment (oven)
    ///
    /// # Arguments
    /// * `kitchen` - Reference to the kitchen containing necessary equipment
    /// * `uncooked_pizza` - The pizza to cook, must be in Uncooked state
    ///
    /// # Returns
    /// * `Ok(Pizza::Cooked)` - If the pizza was cooked successfully
    /// * `Err(PizzaError::IncorrectIngredientState)` - If the pizza is not in Uncooked state
    /// * `Err(PizzaError)` - If another error occurs during cooking
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::{Chef, Kitchen};
    /// use life_of_pie::ingredients::Pizza;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let kitchen = Kitchen::default();
    ///     let chef = Chef;
    ///     
    ///     // Assuming we have an uncooked pizza
    ///     let uncooked_pizza = Pizza::Uncooked;
    ///     
    ///     let cooked_pizza = chef.cook_pizza(&kitchen, uncooked_pizza).await?;
    ///     
    ///     Ok(())
    /// }
    /// ```
    pub async fn cook_pizza(
        &self,
        kitchen: &Kitchen,
        pizza: Pizza,
    ) -> PizzaResult<Pizza> {
        // Validate pizza state
        if pizza != Pizza::Uncooked {
            return Err(PizzaError::IncorrectPizzaState(
                "Expected Uncooked Pizza.".to_string(),
            ));
        }
    
        // Acquire an oven
        let oven = mutex_acquire_one_of_these(&kitchen.ovens).await;
    
        // Bake the pizza
        let cooked_pizza = oven.bake_pizza(pizza).await?;
    
        Ok(cooked_pizza)
    }
    
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ingredients::{Cheese, Dough, Pizza, Tomato};
    use crate::kitchen::Kitchen;

    #[tokio::test]
    async fn test_create_dough_works_as_expected() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        let dough = chef.create_dough(&kitchen).await.unwrap();
        assert_eq!(dough, Dough::Hydrated);
    }

    #[tokio::test]
    async fn test_create_dough_no_flour() {
        let kitchen = Kitchen::default();
        kitchen.fridge.lock().await.portions_of_flour.clear(); // No flour available
        let chef = Chef;

        let result = chef.create_dough(&kitchen).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PizzaError::OutOfFlour);
    }

    #[tokio::test]
    async fn test_develop_dough_works_as_expected() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        let hydrated_dough = chef.create_dough(&kitchen).await.unwrap();
        let mixed_dough = chef.develop_dough(&kitchen, hydrated_dough).await.unwrap();
        assert_eq!(mixed_dough, Dough::Mixed);
    }

    #[tokio::test]
    async fn test_develop_dough_no_hydrated_dough() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        let result = chef.develop_dough(&kitchen, Dough::Mixed).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PizzaError::IncorrectDoughState(
                "You can only put Hydrated Dough in the Mixer!".to_string()
            )
        );
    }

    #[tokio::test]
    async fn test_ferment_dough_works_as_expected() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        let hydrated_dough = chef.create_dough(&kitchen).await.unwrap();
        let mixed_dough = chef.develop_dough(&kitchen, hydrated_dough).await.unwrap();
        let receiver = chef
            .ferment_dough(&kitchen, mixed_dough, None)
            .await
            .unwrap();

        // Wait for the fermentation to complete
        let fermented_dough = receiver.await.unwrap();
        assert_eq!(fermented_dough, Dough::Fermented);
    }

    #[tokio::test]
    async fn test_ferment_dough_no_mixed_dough() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        let result = chef.ferment_dough(&kitchen, Dough::Hydrated, None).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PizzaError::IncorrectDoughState("You can only ferment Mixed Dough!".to_string())
        );
    }

    #[tokio::test]
    async fn test_prove_dough_works_as_expected() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        let fermented_dough_receiver = chef
            .ferment_dough(&kitchen, Dough::Mixed, Some(0))
            .await
            .unwrap();

        let proved_dough = chef
            .prove_dough(&kitchen, fermented_dough_receiver)
            .await
            .unwrap()
            .await
            .unwrap();
        assert_eq!(proved_dough, Dough::Proved);
    }

    #[tokio::test]
    async fn test_prepare_pizza_base_works_as_expected() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        let proved_dough = Dough::Proved;

        let pizza_base = chef
            .prepare_pizza_base(&kitchen, proved_dough)
            .await
            .unwrap();
        assert_eq!(pizza_base, Dough::Rounded);
    }

    #[tokio::test]
    async fn test_prepare_pizza_base_no_proved_dough() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        let result = chef.prepare_pizza_base(&kitchen, Dough::Hydrated).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PizzaError::IncorrectDoughState("The dough is not properly proved!".to_string())
        );
    }

    #[tokio::test]
    async fn test_puree_sauce_works_as_expected() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        let sauce = chef.puree_sauce(&kitchen).await.unwrap();
        assert_eq!(sauce, Tomato::Pureed);
    }

    #[tokio::test]
    async fn test_puree_sauce_no_tomatoes() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        // No tomatoes in the fridge
        kitchen.fridge.lock().await.tomatoes.clear(); // Clear tomatoes

        let result = chef.puree_sauce(&kitchen).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PizzaError::OutOfTomatoes);
    }

    #[tokio::test]
    async fn test_shred_cheese_works_as_expected() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        let shredded_cheese = chef.shred_cheese(&kitchen).await.unwrap();
        assert_eq!(shredded_cheese, Cheese::Grated);
    }

    #[tokio::test]
    async fn test_shred_cheese_no_cheese() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        // No cheese in the fridge
        kitchen.fridge.lock().await.block_of_cheese.clear(); // Clear cheese

        let result = chef.shred_cheese(&kitchen).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PizzaError::OutOfCheese);
    }

    #[tokio::test]
    async fn test_combine_cold_ingredients_to_form_uncooked_pizza_works_as_expected() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        // Assuming we have all the ingredients ready
        let pizza_base = Dough::Rounded;
        let tomato_sauce = Tomato::Pureed;
        let cheese = Cheese::Grated;

        let uncooked_pizza = chef
            .combine_cold_ingredients_to_form_uncooked_pizza(
                &kitchen,
                pizza_base,
                tomato_sauce,
                cheese,
            )
            .await
            .unwrap();

        assert_eq!(uncooked_pizza, Pizza::Uncooked);
    }

    #[tokio::test]
    async fn test_combine_cold_ingredients_to_form_uncooked_pizza_incorrect_state() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        // Incorrect states for ingredients
        let pizza_base = Dough::Hydrated;
        let tomato_sauce = Tomato::Fresh;
        let cheese = Cheese::Fresh;

        let result = chef
            .combine_cold_ingredients_to_form_uncooked_pizza(
                &kitchen,
                pizza_base,
                tomato_sauce,
                cheese,
            )
            .await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), PizzaError::IncorrectIngredientState(
            "You can only combine Pureed Tomato, Grated Cheese, and Rounded Dough to form an Uncooked Pizza!".to_string()
        ));
    }

    #[tokio::test]
    async fn test_cook_pizza_works_as_expected() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        // Assuming we have an uncooked pizza
        let uncooked_pizza = Pizza::Uncooked;

        let cooked_pizza = chef.cook_pizza(&kitchen, uncooked_pizza).await.unwrap();
        assert_eq!(cooked_pizza, Pizza::Cooked);
    }

    #[tokio::test]
    async fn test_cook_pizza_incorrect_state() {
        let kitchen = Kitchen::default();
        let chef = Chef;

        // Incorrect state for pizza
        let cooked_pizza = Pizza::Cooked;

        let result = chef.cook_pizza(&kitchen, cooked_pizza).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            PizzaError::IncorrectIngredientState(
                "You can only cook an Uncooked Pizza!".to_string()
            )
        );
    }
}
