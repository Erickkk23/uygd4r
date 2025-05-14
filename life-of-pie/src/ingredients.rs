//! # Pizza Ingredients Module
//!
//! This module defines all ingredients used in the pizza-making process, along with
//! their possible states. Each ingredient follows a state transition pattern that
//! reflects the real-world transformation of ingredients during cooking.
//!
//! The state transitions enforce a particular order of operations in the kitchen:
//! - Flour must be hydrated to become dough
//! - Dough must go through specific stages (mixed, fermented, proved, rounded)
//! - Tomatoes must be pureed before use
//! - Cheese must be grated before use
//!
//! These state machines help demonstrate how to model real-world processes
//! in software and ensure correctness through type safety.

/// # Flour
///
/// The basic starting ingredient for making pizza dough.
///
/// Flour is represented as a simple struct without state since it doesn't
/// undergo transformations before being incorporated into dough.
///
/// # Examples
///
/// ```
/// use life_of_pie::ingredients::Flour;
///
/// let flour = Flour;
/// ```
pub struct Flour;

/// # Dough
///
/// Represents pizza dough at various stages of preparation.
///
/// Dough goes through a series of transformations as it's prepared:
/// 1. `Hydrated` - Flour mixed with water
/// 2. `Mixed` - Kneaded to develop gluten structure
/// 3. `Fermented` - Left to ferment for flavor development
/// 4. `Proved` - Left to rise and develop volume
/// 5. `Rounded` - Shaped into a pizza base
///
/// This enum enforces the correct sequence of dough preparation, preventing
/// operations from being performed out of order.
///
/// # Examples
///
/// ```
/// use life_of_pie::ingredients::Dough;
///
/// let hydrated_dough = Dough::Hydrated;
/// assert_ne!(hydrated_dough, Dough::Mixed);
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Dough {
    /// Flour that has been mixed with water to form a basic dough
    Hydrated,
    /// Dough that has been kneaded to develop gluten structure
    Mixed,
    /// Dough that has undergone fermentation for flavor development
    Fermented,
    /// Dough that has been proved (allowed to rise)
    Proved,
    /// Dough that has been shaped into a pizza base
    Rounded,
}

/// # Tomato
///
/// Represents tomatoes in various states of preparation for pizza sauce.
///
/// Tomatoes must be pureed before they can be used as sauce on a pizza.
///
/// # Examples
///
/// ```
/// use life_of_pie::ingredients::Tomato;
///
/// let fresh_tomato = Tomato::Fresh;
/// assert_ne!(fresh_tomato, Tomato::Pureed);
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Tomato {
    /// Unprocessed tomato
    Fresh,
    /// Tomato that has been pureed into sauce
    Pureed,
}

/// # Cheese
///
/// Represents cheese in various states of preparation for pizza topping.
///
/// Cheese must be grated before it can be used as a topping on a pizza.
///
/// # Examples
///
/// ```
/// use life_of_pie::ingredients::Cheese;
///
/// let fresh_cheese = Cheese::Fresh;
/// assert_ne!(fresh_cheese, Cheese::Grated);
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Cheese {
    /// Unprocessed block of cheese
    Fresh,
    /// Cheese that has been grated for topping
    Grated,
}

/// # Pizza
///
/// Represents a pizza at different stages of cooking.
///
/// A pizza starts as `Uncooked` after being topped with ingredients,
/// and becomes `Cooked` after being baked in the oven.
///
/// # Examples
///
/// ```
/// use life_of_pie::ingredients::Pizza;
///
/// let uncooked_pizza = Pizza::Uncooked;
/// assert_ne!(uncooked_pizza, Pizza::Cooked);
/// ```
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Pizza {
    /// Pizza that has been assembled but not yet cooked
    Uncooked,
    /// Pizza that has been cooked in the oven
    Cooked,
}
