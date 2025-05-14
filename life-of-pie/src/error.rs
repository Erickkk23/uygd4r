//! # Pizza Kitchen Error Module
//!
//! This module defines all the error types that can occur during the pizza-making process.
//! Proper error handling is a critical aspect of robust async programming, and this module
//! demonstrates how to create custom error types for domain-specific problems.
//!
//! The `PizzaError` enum provides various error cases specific to kitchen operations:
//! - Resource availability errors (out of ingredients)
//! - State validation errors (incorrect ingredient states)
//! - System-level errors (resource contention, etc.)
//!
//! Using this error system allows functions to return meaningful error information when
//! operations cannot be completed, supporting the robust error propagation pattern with
//! the `?` operator in Rust.

use std::error::Error;
use std::fmt;

/// # Pizza Error
///
/// An enum representing all possible errors that can occur during pizza preparation.
///
/// This enum implements the standard `Error` and `Display` traits, making it suitable
/// for use with Rust's error handling patterns, including the `?` operator for error
/// propagation.
///
/// # Examples
///
/// ```
/// use life_of_pie::error::PizzaError;
/// use std::error::Error;
///
/// fn example() -> Result<(), Box<dyn Error>> {
///     // Example of creating and returning an error
///     if !enough_flour_available() {
///         return Err(PizzaError::OutOfFlour.into());
///     }
///     Ok(())
/// }
///
/// fn enough_flour_available() -> bool {
///     // Just for the example
///     true
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PizzaError {
    /// Indicates that there's no more flour available in the kitchen
    OutOfFlour,

    /// Indicates that there are no more tomatoes available in the kitchen
    OutOfTomatoes,

    /// Indicates that there's no more cheese available in the kitchen
    OutOfCheese,

    /// Indicates that there's no more fermented dough available in the kitchen
    OutOfFermentedDough,

    /// Indicates that all proving trays are currently in use
    NoProvingTraysAvailable,

    /// Indicates that a dough is not in the expected state for an operation
    /// The string provides details about the specific state mismatch
    IncorrectDoughState(String),

    /// Indicates that an ingredient is not in the expected state for an operation
    /// The string provides details about the specific state mismatch
    IncorrectIngredientState(String),

    /// A general resource error for other types of resource issues
    /// The string provides details about the specific resource error
    ResourceError(String),
}

impl fmt::Display for PizzaError {
    /// Formats the error message for human-readable output
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PizzaError::OutOfFlour => write!(f, "Out of flour"),
            PizzaError::OutOfTomatoes => write!(f, "Out of tomatoes"),
            PizzaError::OutOfCheese => write!(f, "Out of cheese"),
            PizzaError::OutOfFermentedDough => write!(f, "Out of fermented dough"),
            PizzaError::NoProvingTraysAvailable => write!(f, "No proving trays available"),
            PizzaError::IncorrectDoughState(msg) => write!(f, "Incorrect dough state: {}", msg),
            PizzaError::IncorrectIngredientState(msg) => {
                write!(f, "Incorrect ingredient state: {}", msg)
            }
            PizzaError::ResourceError(msg) => write!(f, "Resource error: {}", msg),
        }
    }
}

/// Implements the standard Error trait for PizzaError
impl Error for PizzaError {}
