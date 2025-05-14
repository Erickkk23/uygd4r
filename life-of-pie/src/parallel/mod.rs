//! # Parallel Pizza Production Strategies
//!
//! This module provides different parallelization strategies for pizza production,
//! demonstrating fundamental parallel programming patterns using Rust's async/await
//! functionality and the Tokio runtime.
//!
//! ## Overview of Parallelization Strategies
//!
//! This module implements two distinct parallelization strategies that represent
//! fundamental patterns in concurrent programming:
//!
//! 1. **Data Parallelism** ([`data_parallel`]): Multiple chefs work independently,
//!    each making entire pizzas from start to finish. This demonstrates how to
//!    distribute identical workloads across multiple workers who share resources.
//!
//! 2. **Task Parallelism** ([`task_parallel`]): Specialized stations focus on
//!    specific tasks, forming a pipeline where each part of the process runs
//!    concurrently. This demonstrates how to divide work into specialized steps
//!    that can happen simultaneously.
//!
//! ## Connection to Pizza Kitchen Simulation
//!
//! Both parallelization strategies build upon the core pizza kitchen simulator:
//!
//! - They use the same [`Kitchen`](crate::Kitchen) struct with shared equipment
//! - They work with the same [`Chef`](crate::Chef) implementations
//! - They handle the same [`ingredients`](crate::ingredients) and [`equipment`](crate::equipment)
//! - They leverage the same error handling via [`PizzaError`](crate::PizzaError)
//!
//! The difference is in how they organize and coordinate multiple tasks to achieve
//! maximum throughput while managing resource constraints.
//!
//! ## Pedagogical Value
//!
//! These implementations demonstrate several key concepts in concurrent programming:
//!
//! - **Resource sharing** - How to handle multiple tasks accessing shared resources
//! - **Synchronization primitives** - Using Mutexes, RwLocks and other concurrency tools
//! - **Task coordination** - Managing the flow of work between concurrent tasks
//! - **Error handling** - Proper propagation and management of errors in concurrent contexts
//! - **Performance tradeoffs** - Understanding the strengths and weaknesses of different approaches
//!
//! ## Implementation Guide
//!
//! As a student, your task is to implement both strategies according to the guidelines
//! provided in each submodule:
//!
//! 1. For data parallelism, you'll implement independent chef workers that each make
//!    complete pizzas while sharing kitchen resources.
//!
//! 2. For task parallelism, you'll implement specialized stations that each perform one
//!    step in the pizza-making process, forming a pipeline.
//!
//! The implementation details and expected behaviors are documented in each module.

pub mod data_parallel;
pub mod task_parallel;

// Re-export the main functions for easier access
pub use data_parallel::data_parallel_production;
pub use task_parallel::task_parallel_production;
