//! # Kitchen Equipment Module
//!
//! This module contains all the equipment used in our pizza kitchen simulation.
//! The equipment represents shared resources that must be properly managed in
//! an asynchronous environment to avoid conflicts and ensure efficient operations.
//!
//! Each piece of equipment has specific responsibilities and characteristics:
//! - `Oven`: For cooking pizzas, limited to one pizza at a time
//! - `Mixer`: For kneading dough and pureeing tomatoes
//! - `Table`: A multi-purpose workspace for various preparation tasks
//!
//! # Simulation Time Scale
//!
//! In this simulation, 1 millisecond represents 1 minute in the real world.
//! This time compression allows us to simulate long cooking processes quickly.

use std::time::Duration;
use tokio::time::sleep;

use crate::error::PizzaError;
use crate::ingredients::{Cheese, Dough, Flour, Pizza, Tomato};

/// # Time Constants
///
/// These constants define how long each kitchen operation takes in our simulation.
/// All times are in milliseconds, where 1ms = 1min in real-world time.
///
/// The values are chosen to reflect realistic time spans for pizza-making operations,
/// allowing you to experience the challenges of resource management across different
/// time scales.

/// Time to hydrate flour into dough (3 minutes)
pub static TIME_TO_HYDRATE_DOUGH: Duration = Duration::from_millis(3);

/// Time to knead dough with a mixer (8 minutes)
pub static TIME_TO_KNEED_DOUGH_WITH_MIXER: Duration = Duration::from_millis(8);

/// Time to ferment dough in a refrigerator (18 hours = 1080 minutes)
pub static TIME_TO_FERMENT_DOUGH_WITH_FRIDGE: Duration = Duration::from_millis(1_080);

/// Time to prove dough at room temperature (60 minutes)
pub static TIME_TO_PROVE_DOUGH_WITH_TABLE: Duration = Duration::from_millis(60);

/// Time to shape dough into a pizza base (3 minutes)
pub static TIME_TO_SHAPE_DOUGH_INTO_PIZZA_BASE_WITH_TABLE: Duration = Duration::from_millis(3);

/// Time to puree tomatoes in a mixer (1 minute)
pub static TIME_TO_PUREE_TOMATO_WITH_MIXER: Duration = Duration::from_millis(1);

/// Time to shred cheese on a work table (3 minutes)
pub static TIME_TO_SHRED_CHEESE_WITH_TABLE: Duration = Duration::from_millis(3);

/// Time to add toppings to a pizza (2 minutes)
pub static TIME_TO_TOP_PIZZA_WITH_INGREDIENTS_WITH_TABLE: Duration = Duration::from_millis(2);

/// Time to cook a pizza in an oven (1 minute)
pub static TIME_TO_COOK_PIZZA_WITH_OVEN: Duration = Duration::from_millis(1);

/// # Oven
///
/// The oven is a critical piece of equipment in our pizza kitchen. It represents
/// a resource with exclusive access requirements - only one pizza can be cooked at a time.
///
/// This structure demonstrates the concept of resource contention in async programming.
/// When multiple chefs need to use the oven simultaneously, they must wait their turn,
/// creating a natural bottleneck in the pizza-making process.
///
/// # Examples
///
/// ```
/// use life_of_pie::equipment::Oven;
/// use life_of_pie::ingredients::Pizza;
/// use life_of_pie::PizzaError;
///
/// async fn cook_a_pizza() -> Result<Pizza, PizzaError> {
///     let oven = Oven;
///     let uncooked_pizza = Pizza::Uncooked;
///     let cooked_pizza = oven.cook_pizza(uncooked_pizza).await?;
///     assert_eq!(cooked_pizza, Pizza::Cooked);
///     Ok(cooked_pizza)
/// }
/// ```
pub struct Oven;

impl Oven {
    /// # Cook a Pizza
    ///
    /// Transforms an uncooked pizza into a delicious cooked pizza!
    ///
    /// This method demonstrates exclusive resource access in async programming.
    /// Only one pizza can be cooked at a time, forcing other tasks to wait.
    /// The cooking process takes time, simulated by an async delay.
    ///
    /// # Arguments
    /// * `uncooked_pizza` - A Pizza in the Uncooked state
    ///
    /// # Returns
    /// A Result containing either:
    /// - Ok(Pizza): A Pizza in the Cooked state
    /// - Err(PizzaError): An error indicating the pizza was in the wrong state
    ///
    /// # Errors
    /// Returns an `IncorrectIngredientState` error if the input pizza is not in the Uncooked state
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::equipment::Oven;
    /// use life_of_pie::ingredients::Pizza;
    /// use life_of_pie::PizzaError;
    ///
    /// async fn example() -> Result<(), PizzaError> {
    ///     let oven = Oven;
    ///     let uncooked_pizza = Pizza::Uncooked;
    ///     
    ///     // Cook the pizza (this will take TIME_TO_COOK_PIZZA_WITH_OVEN)
    ///     let cooked_pizza = oven.cook_pizza(uncooked_pizza).await?;
    ///     
    ///     assert_eq!(cooked_pizza, Pizza::Cooked);
    ///     Ok(())
    /// }
    /// ```
    pub async fn cook_pizza(&self, uncooked_pizza: Pizza) -> Result<Pizza, PizzaError> {
        // Validate the pizza state before proceeding
        if uncooked_pizza != Pizza::Uncooked {
            return Err(PizzaError::IncorrectIngredientState(
                "You can only cook an Uncooked Pizza!".to_string(),
            ));
        }

        // Simulate the cooking process with an async wait
        sleep(TIME_TO_COOK_PIZZA_WITH_OVEN).await;
        Ok(Pizza::Cooked)
    }
}

/// # Mixer
///
/// A kitchen mixer used for kneading dough and pureeing tomatoes.
///
/// The mixer represents a specialized piece of equipment that can perform
/// specific tasks more efficiently than manual methods. Like the oven, it can
/// only be used by one chef at a time, demonstrating another form of resource
/// exclusivity in async environments.
///
/// # Examples
///
/// ```
/// use life_of_pie::equipment::Mixer;
/// use life_of_pie::ingredients::{Dough, Tomato};
/// use life_of_pie::PizzaError;
///
/// async fn use_mixer() -> Result<(), PizzaError> {
///     let mixer = Mixer;
///     
///     // Knead some dough
///     let hydrated_dough = Dough::Hydrated;
///     let mixed_dough = mixer.mix_dough(hydrated_dough).await?;
///     
///     // Puree a tomato
///     let fresh_tomato = Tomato::Fresh;
///     let tomato_puree = mixer.mix_tomato_into_puree(fresh_tomato).await?;
///     
///     Ok(())
/// }
/// ```
pub struct Mixer;

impl Mixer {
    /// # Mix Dough
    ///
    /// Transforms hydrated dough into properly mixed dough by kneading it.
    ///
    /// This method simulates the time-consuming process of kneading dough
    /// using a stand mixer. The mixer, being a specialized tool, performs this
    /// task more efficiently than manual methods, but still requires a significant
    /// amount of time.
    ///
    /// # Arguments
    /// * `dough` - Dough to be kneaded (typically in Hydrated state)
    ///
    /// # Returns
    /// A Result containing either:
    /// - Ok(Dough): Dough in the Mixed state
    /// - Err(PizzaError): An error indicating the dough was in the wrong state
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::equipment::Mixer;
    /// use life_of_pie::ingredients::Dough;
    /// use life_of_pie::PizzaError;
    ///
    /// async fn knead_dough() -> Result<(), PizzaError> {
    ///     let mixer = Mixer;
    ///     let hydrated_dough = Dough::Hydrated;
    ///     
    ///     let mixed_dough = mixer.mix_dough(hydrated_dough).await?;
    ///     assert_eq!(mixed_dough, Dough::Mixed);
    ///     Ok(())
    /// }
    /// ```
    pub async fn mix_dough(&self, dough: Dough) -> Result<Dough, PizzaError> {
        // Validate dough state if needed
        if dough != Dough::Hydrated {
            return Err(PizzaError::IncorrectDoughState(
                "You can only mix hydrated dough!".to_string(),
            ));
        }

        // Simulate the time-consuming process of kneading dough
        sleep(TIME_TO_KNEED_DOUGH_WITH_MIXER).await;
        Ok(Dough::Mixed)
    }

    /// # Mix Tomato into Puree
    ///
    /// Transforms fresh tomatoes into tomato puree for pizza sauce.
    ///
    /// This method demonstrates both state validation and async simulation.
    /// It checks that the tomato is in the correct initial state before
    /// processing and then simulates the time-consuming process of pureeing.
    ///
    /// # Arguments
    /// * `tomato` - A Tomato in the Fresh state
    ///
    /// # Returns
    /// A Result containing either:
    /// - Ok(Tomato): A Tomato in the Pureed state
    /// - Err(PizzaError): An error indicating the tomato was in the wrong state
    ///
    /// # Errors
    /// Returns an `IncorrectIngredientState` error if the input tomato is not in the Fresh state
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::equipment::Mixer;
    /// use life_of_pie::ingredients::Tomato;
    /// use life_of_pie::PizzaError;
    ///
    /// async fn puree_tomato() -> Result<(), PizzaError> {
    ///     let mixer = Mixer;
    ///     let fresh_tomato = Tomato::Fresh;
    ///     
    ///     let tomato_puree = mixer.mix_tomato_into_puree(fresh_tomato).await?;
    ///     assert_eq!(tomato_puree, Tomato::Pureed);
    ///     Ok(())
    /// }
    /// ```
    pub async fn mix_tomato_into_puree(&self, tomato: Tomato) -> Result<Tomato, PizzaError> {
        // Validate input state before proceeding
        if tomato != Tomato::Fresh {
            return Err(PizzaError::IncorrectIngredientState(
                "You can only puree Fresh Tomatoes!".to_string(),
            ));
        }

        // Simulate the blending process
        sleep(TIME_TO_PUREE_TOMATO_WITH_MIXER).await;
        Ok(Tomato::Pureed)
    }
}

/// # Table
///
/// A multipurpose work surface for various food preparation tasks.
///
/// Unlike the Oven and Mixer, the Table represents a shared resource that can potentially
/// be used by multiple chefs simultaneously for different preparation tasks. This
/// demonstrates how some resources in async programming can be accessed concurrently.
///
/// The Table can be used for:
/// - Mixing flour into dough
/// - Shaping dough into pizza bases
/// - Shredding cheese
/// - Adding toppings to pizza bases
///
/// # Examples
///
/// ```
/// use life_of_pie::equipment::Table;
/// use life_of_pie::ingredients::{Flour, Dough, Cheese, Tomato, Pizza};
/// use life_of_pie::PizzaError;
///
/// async fn use_table() -> Result<(), PizzaError> {
///     let table = Table::default();
///     
///     // Mix flour into dough
///     let flour = Flour;
///     let hydrated_dough = table.mix_flour_into_dough(flour).await?;
///     
///     // Later in the process...
///     // Shape proved dough into a pizza base
///     let proved_dough = Dough::Proved;
///     let pizza_base = table.shape_dough_into_pizza_base(proved_dough).await?;
///     
///     // Shred cheese
///     let fresh_cheese = Cheese::Fresh;
///     let grated_cheese = table.shred_cheese_on_table(fresh_cheese).await?;
///     
///     // Create a pizza with toppings
///     let rounded_dough = Dough::Rounded;
///     let tomato_puree = Tomato::Pureed;
///     let uncooked_pizza = table.top_pizza_base_with_ingredients(
///         rounded_dough,
///         tomato_puree,
///         grated_cheese
///     ).await?;
///     
///     Ok(())
/// }
/// ```
#[derive(Default)]
pub struct Table;

impl Table {
    /// # Mix Flour into Dough
    ///
    /// Transforms flour into hydrated dough by adding water and mixing.
    ///
    /// This method represents the initial step in pizza dough preparation.
    /// It simulates the process of combining flour and water to create hydrated dough,
    /// which is essential before kneading and further processing.
    ///
    /// # Arguments
    /// * `flour` - The flour to be hydrated
    ///
    /// # Returns
    /// A Result containing either:
    /// - Ok(Dough): Dough in the Hydrated state
    /// - Err(PizzaError): An error indicating a resource issue
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::equipment::Table;
    /// use life_of_pie::ingredients::{Flour, Dough};
    /// use life_of_pie::PizzaError;
    ///
    /// async fn hydrate_dough() -> Result<(), PizzaError> {
    ///     let table = Table::default();
    ///     let flour = Flour;
    ///     
    ///     let hydrated_dough = table.mix_flour_into_dough(flour).await?;
    ///     assert_eq!(hydrated_dough, Dough::Hydrated);
    ///     Ok(())
    /// }
    /// ```
    pub async fn mix_flour_into_dough(&self, _flour: Flour) -> Result<Dough, PizzaError> {
        // Simulate the hydration process
        sleep(TIME_TO_HYDRATE_DOUGH).await;
        Ok(Dough::Hydrated)
    }

    /// # Shape Dough into Pizza Base
    ///
    /// Transforms proved dough into a rounded pizza base ready for toppings.
    ///
    /// This method demonstrates state validation in async functions. It checks
    /// that the dough is in the correct state (proved) before proceeding,
    /// enforcing the proper sequence of operations in the pizza-making process.
    ///
    /// # Arguments
    /// * `dough` - Dough in the Proved state
    ///
    /// # Returns
    /// A Result containing either:
    /// - Ok(Dough): Dough in the Rounded state
    /// - Err(PizzaError): An error indicating the dough was in the wrong state
    ///
    /// # Errors
    /// Returns an `IncorrectDoughState` error if the input dough is not in the Proved state
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::equipment::Table;
    /// use life_of_pie::ingredients::Dough;
    /// use life_of_pie::PizzaError;
    ///
    /// async fn create_pizza_base() -> Result<(), PizzaError> {
    ///     let table = Table::default();
    ///     let proved_dough = Dough::Proved;
    ///     
    ///     let pizza_base = table.shape_dough_into_pizza_base(proved_dough).await?;
    ///     assert_eq!(pizza_base, Dough::Rounded);
    ///     Ok(())
    /// }
    /// ```
    pub async fn shape_dough_into_pizza_base(&self, dough: Dough) -> Result<Dough, PizzaError> {
        // Validate dough state before proceeding
        if dough != Dough::Proved {
            return Err(PizzaError::IncorrectDoughState(
                "You can only shape Proved Dough!".to_string(),
            ));
        }

        // Simulate the shaping process
        sleep(TIME_TO_SHAPE_DOUGH_INTO_PIZZA_BASE_WITH_TABLE).await;
        Ok(Dough::Rounded)
    }

    /// # Shred Cheese
    ///
    /// Transforms fresh cheese into grated cheese for pizza topping.
    ///
    /// This method simulates the process of manually grating cheese on a work table.
    /// It demonstrates both state validation and time simulation in async functions.
    ///
    /// # Arguments
    /// * `cheese` - Cheese in the Fresh state
    ///
    /// # Returns
    /// A Result containing either:
    /// - `Ok(Cheese)`: Cheese in the Grated state
    /// - `Err<PizzaError>`: An error indicating the cheese was in the wrong state
    ///
    /// # Errors
    /// Returns an `IncorrectIngredientState` error if the input cheese is not in the Fresh state
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::equipment::Table;
    /// use life_of_pie::ingredients::Cheese;
    /// use life_of_pie::PizzaError;
    ///
    /// async fn grate_cheese() -> Result<(), PizzaError> {
    ///     let table = Table::default();
    ///     let fresh_cheese = Cheese::Fresh;
    ///     
    ///     let grated_cheese = table.shred_cheese_on_table(fresh_cheese).await?;
    ///     assert_eq!(grated_cheese, Cheese::Grated);
    ///     Ok(())
    /// }
    /// ```
    pub async fn shred_cheese_on_table(&self, cheese: Cheese) -> Result<Cheese, PizzaError> {
        // Validate cheese state before proceeding
        if cheese != Cheese::Fresh {
            return Err(PizzaError::IncorrectIngredientState(
                "You can only shred Fresh Cheese!".to_string(),
            ));
        }

        // Simulate the shredding process
        sleep(TIME_TO_SHRED_CHEESE_WITH_TABLE).await;
        Ok(Cheese::Grated)
    }

    /// # Top Pizza with Ingredients
    ///
    /// Combines a rounded dough base, tomato sauce, and grated cheese to create an uncooked pizza.
    ///
    /// This method demonstrates complex precondition validation in async programming.
    /// It verifies that all ingredients are in the correct states before proceeding,
    /// enforcing the proper sequence of preparation steps.
    ///
    /// # Arguments
    /// * `pizza_base` - Dough in the Rounded state
    /// * `tomato_sauce` - Tomato in the Pureed state
    /// * `cheese` - Cheese in the Grated state
    ///
    /// # Returns
    /// A Result containing either:
    /// - Ok(Pizza): Pizza in the Uncooked state
    /// - Err(PizzaError): An error indicating one or more ingredients were in the wrong state
    ///
    /// # Errors
    /// Returns an `IncorrectIngredientState` error if any of the inputs are not in their required states
    ///
    /// # Examples
    ///
    /// ```
    /// use life_of_pie::equipment::Table;
    /// use life_of_pie::ingredients::{Dough, Tomato, Cheese, Pizza};
    /// use life_of_pie::PizzaError;
    ///
    /// async fn prepare_pizza() -> Result<(), PizzaError> {
    ///     let table = Table::default();
    ///     let pizza_base = Dough::Rounded;
    ///     let tomato_sauce = Tomato::Pureed;
    ///     let cheese = Cheese::Grated;
    ///     
    ///     let uncooked_pizza = table.top_pizza_base_with_ingredients(
    ///         pizza_base,
    ///         tomato_sauce,
    ///         cheese
    ///     ).await?;
    ///     
    ///     assert_eq!(uncooked_pizza, Pizza::Uncooked);
    ///     Ok(())
    /// }
    /// ```
    pub async fn top_pizza_base_with_ingredients(
        &self,
        pizza_base: Dough,
        tomato_sauce: Tomato,
        cheese: Cheese,
    ) -> Result<Pizza, PizzaError> {
        // Validate all ingredient states before proceeding
        if pizza_base != Dough::Rounded {
            return Err(PizzaError::IncorrectDoughState(
                "Pizza base must be in Rounded state!".to_string(),
            ));
        }

        if tomato_sauce != Tomato::Pureed {
            return Err(PizzaError::IncorrectIngredientState(
                "Tomato sauce must be Pureed!".to_string(),
            ));
        }

        if cheese != Cheese::Grated {
            return Err(PizzaError::IncorrectIngredientState(
                "Cheese must be Grated!".to_string(),
            ));
        }

        // Simulate the topping process
        sleep(TIME_TO_TOP_PIZZA_WITH_INGREDIENTS_WITH_TABLE).await;
        Ok(Pizza::Uncooked)
    }
}
