//! # Life of Pie
//! Right as your long term contract with the local pizza restaurant
//! is comming to an end, you get a call from the owner of the
//! restaurant. They tell you that they would like to extend your
//! contract for a few more months! The work on their file system
//! has been great, but the owner has a new idea for you!
//!
//! Recently, the owner has been watching a lot of cooking shows
//! and has become obsessed with the idea of an optimized kitchen.
//! They tell you that they want you to help them make the best use
//! of their current kitchen space and equipment. They also want you
//! to help figure out whether it would be worth it to invest in
//! more staff.
//!
//! Whilst you aren't quite sure how to run a kitchen yourself, you
//! figure that you could whip up a simulator for the kitchen that you
//! can tweak and modify to gain insight into the kitchen's operations.
//!
//! The owner is ecstatic! They tell you that they have a few ideas
//! for how they want the kitchen to run and they direct you to one of
//! the shows they have been watching. (At this point ensure you have watched the video
//! linked on the course page about making pizza from scratch with Babish).
//!
//! You decide to take some notes on the show and below you produce the
//! following workflow for making a pizza:
//!
//! 1. Create hydrated dough
//! 2. Develop the hydrated dough via mixing
//! 3. Ferment the mixed dough in the fridge
//! 4. Prove the fermented dough in the proving room
//! 5. Shape the proved dough into the base of a pizza
//! 6. Add sauce to the pizza base
//! 7. Add cheese to the pizza
//! 8. Bake the pizza in the oven
//!
//! The owner is very happy with your notes and they tell you that
//! you are well on your way to becoming a master chef! They also tell you
//! to be careful as the notes explain the process of making a pizza, they do
//! not explain how to make one whilst others are getting in your way!
//!
//! With your ideation complete and your notes in hand, you decide to get to work!
//!
//! # Assignment: Concurrent Object Production Simulator
//!
//! The use of threads and async programming is crucial to developing code for operating systems
//! that is efficient and responsive. Not only that, but without threads and concurrency, we would
//! not be able to run multiple applications at once!
//!
//! Additionally, as programmers, we often are tasked with "accurately" modeling the world around us.
//! Whether it be a file system, a kitchen, or even a car, we need to be able to
//! fully express tasks and their interactions with one another in as much detail as is needed.
//! Many, if not all, of the systems we interact with on a daily basis are built
//! using these two concepts. In this assignment, we will borrow the simple idea of a kitchen
//! making pizzas to help us understand how to build a system that can run multiple
//! tasks at once, and how to model the interactions between those tasks.
//!
//! The goal of this assignment is to implement a simulator that is extraordinarily
//! granular in its modeling of the kitchen. You will need to consider all that is
//! happening in the kitchen at once, and how other interactions may affect
//! the tasks you are trying to perform.
//!
//! As an example, we would nonchalantly assume we could just access the fridge
//! and take out some dough whenever we needed it. It might look something like below:
//!
//! ```rust
//! struct Refrigerator {
//!     dough: i32,
//! }
//!
//! impl Refrigerator {
//!    fn take_dough(&mut self) -> i32 {
//!        if self.dough > 0 {
//!           self.dough -= 1;
//!            return 1;
//!       }
//!        panic!("No dough left in the fridge!");
//!   }
//! }
//!
//! fn main() {
//!    let mut fridge = Refrigerator { dough: 10 };
//!    let dough = fridge.take_dough();
//! }
//! ```
//!
//! However, in reality, the fridge is a shared resource and if we are not careful, we may end up with two chefs
//! trying to access the fridge at the same time. This means we could
//! bump shoulders with another chef (Race Condition), so we need to ensure that we are the only ones
//! accessing the fridge at any given time. So a fridge is actually a resource that
//! must be protected with Mutual Exclusion (Mutex) to ensure that only one chef can access it at a time.
//!
//! Then, we need to consider that every piece of dough we take out of the fridge
//! has its own degree of fermentation it is undergoing and that each dough will be in its own
//! asynchronous process of becoming ready to be used.
//!
//! Once we've found dough that is ready to be used, we need to them give up our
//! control of the fridge to let other chefs use it! So overall the process of obtaining
//! fermented dough from the fridge looks something like this instead:
//!
//! ```rust
//! use tokio::sync::Mutex;
//! use std::sync::Arc;
//! use tokio::time;
//! use std::time::Duration;
//!
//! struct Dough {
//!     is_fermented: bool,
//! }
//! impl Dough {
//!     fn default() -> Self {
//!        Dough {
//!            is_fermented: false,
//!        }
//!    }
//!
//!    fn is_fermented(&self) -> bool {
//!         self.is_fermented
//!     }
//!
//!     async fn set_fermented(&mut self) {
//!          time::sleep(Duration::from_secs(2)).await;
//!          self.is_fermented = true;
//!     }
//!
//!   }
//! struct Refrigerator {
//!      dough: Vec<Dough>,
//! }
//!
//! impl Refrigerator {
//!    fn take_dough(&mut self) -> Dough {
//!        if self.dough.len() > 0 {
//!            let dough = self.dough.pop().unwrap();
//!            if dough.is_fermented() {
//!                return dough;
//!            }
//!        }
//!        panic!("No dough left in the fridge!");
//!    }
//!   }
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut dough = Dough::default();
//!     dough.set_fermented().await;
//!     let fridge = Arc::new(Mutex::new(Refrigerator { dough: vec![ dough ] }));
//!     let fermented_dough = {
//!         let dough = fridge.lock().await.take_dough();
//!     };
//! }
//! ```
//!
//! However even this is not quite right! Consider where we are waiting for the dough
//! in the example above. We are staring at the fridge waiting for the dough to be
//! ready. This is not a good use of our time! We should be able to
//! continue working on other tasks while we wait for the dough to be ready!
//!
//! Above is a good example of the kind of thinking you will need to do
//! to complete this assignment. You will need to consider how the kitchen is
//! modeled, how a chef interacts with the kitchen, and how the kitchen
//! interacts with the chefs!
//!
//! Once the simulator is complete, you will have the foundation ready to
//! produce a pizza! You will then be resposible for implementing the
//! parallelization of the kitchen. This means you will need to consider
//! how to run multiple chefs at once, how to share resources between
//! them, and how to ensure that they do not interfere with each other.
//!
//! You will do this by implementing two different parallelization strategies:
//! 1. Task parallelism - where each chef has their own station and they
//!   work on their own tasks independently.
//! 2. Data parallelism - where multiple chefs work on the same task
//!  at the same time, but they are all working on different pieces of
//!  data.
//!
//! The goal of this assignment is to help you understand how to
//! model a system that is
//! concurrent and parallel, how resources are interacted with in that
//! context, and how to ensure that the system is safe and efficient.
//! Then, we use it to teach ourselves about the two different types parallel
//! partitioning. Finally, you will be asked to tweak the hyper-parameters
//! of the system to see how they affect the performance of the kitchen!
//!
//! To run the simulator you will use the command `cargo run <PARALLELISM TYPE> <NUM OF PIZZAS>`.
//! The parallelism type can be either `task` or `data`.
//! The number of pizzas is the number amount of pizzas that will be made via the
//! selected parallelism type using the kitchen.
//!
//! Example:
//! ```bash
//! cargo run task 10
//! ```
//! This will run the task parallelism simulation with 10 pizzas.
//!
//! ```bash
//! cargo run data 10
//! ```
//! This will run the data parallelism simulation with 10 pizzas.
//!
//! ## Goal:
//! Complete all unimplemented methods in their respective modules such that
//! running `cargo test` passes all unit tests. Accomplishing this results
//! in full points.
//!
//! ## Tasks:
//! 0. Read all associated documentation on the doc site about each module before
//! beginning programming. (This is important as this assignment relies on a careful
//! understanding of each of the components of the simulator! (Kitchen, Chef, Fridge, etc.))
//! 1. Complete all foundational methods the simulator relies on in chef.rs
//! 2. Ensure all functions pass their dedicated unit tests defined at the bottom
//! of chef.rs
//! 3. Once the foundational methods are complete, move on to data_parallel_production.rs
//! and complete the implementation of the data parallelism simulation using the components
//! you have already implemented.
//! 4. Ensure the implementation for data parallel productions passes its dedicated unit
//! tests defined at the bottom of the file.
//!
//! ## Useful commands to remember:
//! Run `cargo doc --open` to view the documentation and find areas to complete.
//!
//! Run `cargo run` to run the main function in main.rs
//!
//! Run `cargo fmt` to automatically format all files to convention.
//!
//! Run `cargo test` to run all unit tests.
//!
//! ## Hints/Tips
//! - There are many doc comments all over the code base filled with useful insight and
//! purpose for how each is meant to be used. Make sure to read them all!

// Re-export the types and functions that are needed by consumers of the library
pub use crate::chef::Chef;
pub use crate::error::PizzaError;
pub use crate::ingredients::{Cheese, Dough, Pizza, Tomato};
pub use crate::kitchen::Kitchen;

// Make modules available within the crate
pub mod chef;
pub mod equipment;
pub mod error;
pub mod fridge;
pub mod ingredients;
pub mod kitchen;
pub mod parallel;
pub mod proving_room;

// Re-export the result type alias for convenience
pub type PizzaResult<T> = Result<T, PizzaError>;
