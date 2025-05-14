use crate::{Cheese, Chef, Dough, Kitchen, Pizza, Tomato};
use heapless::mpmc::MpMcQueue;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// Define capacity for our MPMC queues
const QUEUE_CAPACITY: usize = 16;

/// # Station Completion Flags
/// 
/// This structure manages coordination between different processing stations using
/// atomic boolean flags to signal when each station has completed its work.
///
/// ## Key Concurrency Concepts Demonstrated:
/// * **Atomic operations**: Thread-safe flags without locks using AtomicBool
/// * **Signaling mechanism**: One stage signals others when it's done
/// * **Termination detection**: Each stage can check if upstream stages are completed
///
/// Atomic types are a lock-free synchronization primitive that allow for safe concurrent
/// access without the overhead of mutexes. They're ideal for simple flags like these
/// that don't require more complex synchronization.
///
/// The `Ordering` parameter (SeqCst) defines the memory ordering guarantees,
/// with Sequential Consistency being the strongest but most expensive ordering.
struct StationFlags {
    hydration_complete: AtomicBool,
    mixing_complete: AtomicBool,
    fermentation_complete: AtomicBool,
    proving_complete: AtomicBool,
    base_prep_complete: AtomicBool,
    sauce_complete: AtomicBool,
    cheese_complete: AtomicBool,
}

impl StationFlags {
    fn new() -> Self {
        Self {
            hydration_complete: AtomicBool::new(false),
            mixing_complete: AtomicBool::new(false),
            fermentation_complete: AtomicBool::new(false),
            proving_complete: AtomicBool::new(false),
            base_prep_complete: AtomicBool::new(false),
            sauce_complete: AtomicBool::new(false),
            cheese_complete: AtomicBool::new(false),
        }
    }
}

/// # Station Configuration
/// 
/// This structure defines how many worker threads are allocated to each stage of the
/// pizza production pipeline. It allows for flexible scaling of different stages
/// based on their processing speed and resource requirements.
///
/// ## Key System Design Concepts:
/// * **Resource allocation**: Distributing processing resources based on workload requirements
/// * **Concurrency tuning**: Adjusting parallelism levels at each stage independently
/// * **Bottleneck management**: Adding more workers to slower stages to balance the pipeline
///
/// Determining the optimal number of workers per stage is a complex performance tuning
/// problem. Factors to consider include:
/// - The computational intensity of each stage
/// - Available system resources (CPU cores, memory)
/// - Dependencies between stages
/// - Potential bottlenecks in the pipeline
///
/// Experimenting with different configurations can demonstrate how system throughput
/// changes based on resource allocation decisions.
struct StationConfig {
    hydration_workers: usize,
    mixing_workers: usize,
    fermentation_workers: usize,
    proving_workers: usize,
    base_prep_workers: usize,
    sauce_workers: usize,
    cheese_workers: usize,
    assembly_workers: usize,
}

impl Default for StationConfig {
    fn default() -> Self {
        Self {
            hydration_workers: 1,
            mixing_workers: 1,
            fermentation_workers: 1,
            proving_workers: 1,
            base_prep_workers: 1,
            sauce_workers: 1,
            cheese_workers: 1,
            assembly_workers: 1,
        }
    }
}

/// # Task Parallel Production
///
/// In this approach, the pizza-making process is divided into specialized stations,
/// with each station performing a specific step in the process. This models a production
/// line where different workers specialize in different tasks.
///
/// ## Key Concurrency Concepts Demonstrated:
/// * **Pipeline parallelism**: Work flows through a series of stages
/// * **Multiple workers per stage**: Each stage can have multiple workers processing items in parallel
/// * **Producer-consumer pattern**: Each stage produces items for the next stage to consume
/// * **Work queues**: Communication between stages happens via MPMC (Multi-Producer Multi-Consumer) queues
/// * **Termination coordination**: Each stage signals when it has completed its work
///
/// ## Workflow:
/// 1. Multiple stations work simultaneously: hydration, mixing, fermentation, proving, etc.
/// 2. Each station takes input from the previous stage via a queue
/// 3. After processing, each station places output in the next stage's queue
/// 4. Stations coordinate via atomic flags to ensure proper termination
///
/// This approach maximizes throughput by allowing specialization and pipelining, but
/// may introduce bottlenecks if stages operate at different speeds. The slowest stage
/// determines the overall system throughput (known as the "critical path").
pub async fn task_parallel_production(kitchen: &Arc<Kitchen>, num_pizzas: usize) {
    println!(
        "🍕 Starting multi-station task-parallel pizza production for {} pizzas",
        num_pizzas
    );

    let config = StationConfig::default();
    let flags = Arc::new(StationFlags::new());

    // Create MPMC queues for each stage
    let hydrated_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>> = Arc::new(MpMcQueue::new());
    let mixed_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>> = Arc::new(MpMcQueue::new());

    // Updated fermented_queue to pass receivers instead of dough
    let fermented_queue: Arc<MpMcQueue<tokio::sync::oneshot::Receiver<Dough>, QUEUE_CAPACITY>> =
        Arc::new(MpMcQueue::new());

    let proved_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>> = Arc::new(MpMcQueue::new());
    let base_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>> = Arc::new(MpMcQueue::new());
    let sauce_queue: Arc<MpMcQueue<Tomato, QUEUE_CAPACITY>> = Arc::new(MpMcQueue::new());
    let cheese_queue: Arc<MpMcQueue<Cheese, QUEUE_CAPACITY>> = Arc::new(MpMcQueue::new());
    let completed_queue: Arc<MpMcQueue<Pizza, QUEUE_CAPACITY>> = Arc::new(MpMcQueue::new());

    let mut handles = Vec::new();

    // Spawn multiple instances of each station
    for id in 1..=config.hydration_workers {
        handles.push(spawn_hydration_station(
            Arc::clone(kitchen),
            Arc::clone(&hydrated_queue),
            num_pizzas / config.hydration_workers
                + (if id <= num_pizzas % config.hydration_workers {
                    1
                } else {
                    0
                }),
            id,
            Arc::clone(&flags),
        ));
    }

    for id in 1..=config.mixing_workers {
        handles.push(spawn_mixing_station(
            Arc::clone(kitchen),
            Arc::clone(&hydrated_queue),
            Arc::clone(&mixed_queue),
            id,
            Arc::clone(&flags),
        ));
    }

    for id in 1..=config.fermentation_workers {
        handles.push(spawn_fermentation_station(
            Arc::clone(kitchen),
            Arc::clone(&mixed_queue),
            Arc::clone(&fermented_queue),
            id,
            Arc::clone(&flags),
        ));
    }

    for id in 1..=config.proving_workers {
        handles.push(spawn_proving_station(
            Arc::clone(kitchen),
            Arc::clone(&fermented_queue),
            Arc::clone(&proved_queue),
            id,
            Arc::clone(&flags),
        ));
    }

    for id in 1..=config.base_prep_workers {
        handles.push(spawn_base_preparation_station(
            Arc::clone(kitchen),
            Arc::clone(&proved_queue),
            Arc::clone(&base_queue),
            id,
            Arc::clone(&flags),
        ));
    }

    for id in 1..=config.sauce_workers {
        handles.push(spawn_sauce_station(
            Arc::clone(kitchen),
            Arc::clone(&sauce_queue),
            num_pizzas / config.sauce_workers
                + (if id <= num_pizzas % config.sauce_workers {
                    1
                } else {
                    0
                }),
            id,
            Arc::clone(&flags),
        ));
    }

    for id in 1..=config.cheese_workers {
        handles.push(spawn_cheese_station(
            Arc::clone(kitchen),
            Arc::clone(&cheese_queue),
            num_pizzas / config.cheese_workers
                + (if id <= num_pizzas % config.cheese_workers {
                    1
                } else {
                    0
                }),
            id,
            Arc::clone(&flags),
        ));
    }

    for id in 1..=config.assembly_workers {
        handles.push(spawn_assembly_station(
            Arc::clone(kitchen),
            Arc::clone(&base_queue),
            Arc::clone(&sauce_queue),
            Arc::clone(&cheese_queue),
            Arc::clone(&completed_queue),
            id,
            Arc::clone(&flags),
        ));
    }

    // Start a task to count completed pizzas
    let completed_counter = tokio::spawn(async move {
        let mut count = 0;
        while count < num_pizzas {
            // Non-blocking check for completed pizzas
            if let Some(_) = completed_queue.dequeue() {
                count += 1;
            } else {
                // Small sleep to avoid busy waiting
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            }
        }
        count
    });

    // Wait for all pizzas to be completed
    let completed = completed_counter.await.unwrap_or(0);
    println!(
        "📊 Kitchen produced {} out of {} desired pizzas",
        completed, num_pizzas
    );
}

// Updated station functions to use MPMC queues

fn spawn_hydration_station(
    kitchen: Arc<Kitchen>,
    output_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>>,
    target_pizzas: usize,
    id: usize,
    flags: Arc<StationFlags>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let chef = Chef;
        let mut count = 0;
        println!("👨‍🍳 Hydration station #{} starting work", id);

        for _ in 0..target_pizzas {
            match chef.create_dough(&kitchen).await {
                Ok(mut dough) => {
                    count += 1;
                    println!(
                        "🌊 Hydration station #{} created hydrated dough #{}",
                        id, count
                    );

                    // Try to enqueue with backoff if queue is full
                    loop {
                        match output_queue.enqueue(dough) {
                            Ok(_) => break,
                            Err(returned_dough) => {
                                // Queue is full, wait a bit and try again
                                dough = returned_dough;
                                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                            }
                        }
                    }
                }

                Err(e) => {
                    flags.hydration_complete.store(true, Ordering::SeqCst);
                    println!("❌ Hydration station #{} error: {}", id, e)
                }
            }
        }
        println!(
            "🏁 Hydration station #{} finished, produced {} doughs",
            id, count
        );
        flags.hydration_complete.store(true, Ordering::SeqCst);
    })
}

fn spawn_mixing_station(
    kitchen: Arc<Kitchen>,
    input_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>>,
    output_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>>,
    id: usize,
    flags: Arc<StationFlags>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let chef = Chef;
        let mut count = 0;
        println!("👨‍🍳 Mixing station #{} starting work", id);

        loop {
            // Try to dequeue with non-blocking approach
            if let Some(dough) = input_queue.dequeue() {
                match chef.develop_dough(&kitchen, dough).await {
                    Ok(mixed) => {
                        count += 1;
                        println!("🧠 Mixing station #{} developed dough #{}", id, count);

                        // Try to enqueue with backoff if queue is full
                        let mut mixed_dough = mixed;
                        loop {
                            match output_queue.enqueue(mixed_dough) {
                                Ok(_) => break,
                                Err(returned_dough) => {
                                    // Queue is full, wait a bit and try again
                                    mixed_dough = returned_dough;
                                    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        flags.mixing_complete.store(true, Ordering::SeqCst);
                        println!("❌ Mixing station #{} error: {}", id, e)
                    }
                }
            } else {
                // Check if we should exit
                if count > 0 && flags.hydration_complete.load(Ordering::SeqCst) {
                    break;
                }
                // No dough available, wait a bit before checking again
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            }
        }
        println!("🏁 Mixing station #{} finished, mixed {} doughs", id, count);
        flags.mixing_complete.store(true, Ordering::SeqCst);
    })
}

// Modified to pass fermentation receivers through the queue
fn spawn_fermentation_station(
    kitchen: Arc<Kitchen>,
    input_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>>,
    output_queue: Arc<MpMcQueue<tokio::sync::oneshot::Receiver<Dough>, QUEUE_CAPACITY>>,
    id: usize,
    flags: Arc<StationFlags>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let chef = Chef;
        let mut count = 0;
        println!("👨‍🍳 Fermentation station #{} starting work", id);

        loop {
            // Try to dequeue with non-blocking approach
            if let Some(dough) = input_queue.dequeue() {
                // Start fermentation with station ID and get the receiver for the fermented dough
                match chef.ferment_dough(&kitchen, dough, Some(id)).await {
                    Ok(fermentation_receiver) => {
                        count += 1;
                        println!(
                            "🔄 Fermentation station #{} started fermenting dough #{}",
                            id, count
                        );

                        // Pass the fermentation receiver to the next station through the queue
                        let mut receiver = fermentation_receiver;
                        loop {
                            match output_queue.enqueue(receiver) {
                                Ok(_) => break,
                                Err(returned_receiver) => {
                                    // Queue is full, wait a bit and try again
                                    receiver = returned_receiver;
                                    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        flags.fermentation_complete.store(true, Ordering::SeqCst);
                        println!("❌ Fermentation station #{} error: {}", id, e)
                    }
                }
            } else {
                // Check if we should exit
                if count > 0 && flags.mixing_complete.load(Ordering::SeqCst) {
                    break;
                }
                // No dough available, wait a bit before checking again
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            }
        }
        println!(
            "🏁 Fermentation station #{} finished, fermented {} doughs",
            id, count
        );
        flags.fermentation_complete.store(true, Ordering::SeqCst);
    })
}

// Modified to receive fermentation receivers from the queue
fn spawn_proving_station(
    kitchen: Arc<Kitchen>,
    input_queue: Arc<MpMcQueue<tokio::sync::oneshot::Receiver<Dough>, QUEUE_CAPACITY>>,
    output_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>>,
    id: usize,
    flags: Arc<StationFlags>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let chef = Chef;
        let mut count = 0;
        println!("👨‍🍳 Proving station #{} starting work", id);

        loop {
            // Try to dequeue a fermentation receiver with non-blocking approach
            if let Some(fermentation_receiver) = input_queue.dequeue() {
                // Pass the fermentation receiver directly to the chef's prove_dough method
                match chef.prove_dough(&kitchen, fermentation_receiver).await {
                    Ok(proving_receiver) => {
                        // Await the proved dough from the receiver
                        match proving_receiver.await {
                            Ok(proved_dough) => {
                                count += 1;
                                println!("👀 Proving station #{} proved dough #{}", id, count);

                                // Try to enqueue with backoff if queue is full
                                let mut dough = proved_dough;
                                loop {
                                    match output_queue.enqueue(dough) {
                                        Ok(_) => break,
                                        Err(returned_dough) => {
                                            // Queue is full, wait a bit and try again
                                            dough = returned_dough;
                                            tokio::time::sleep(tokio::time::Duration::from_millis(
                                                1,
                                            ))
                                            .await;
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                flags.proving_complete.store(true, Ordering::SeqCst);
                                println!("❌ Proving station #{} error receiving proved dough", id);
                            }
                        }
                    }
                    Err(e) => {
                        flags.proving_complete.store(true, Ordering::SeqCst);
                        println!("❌ Proving station #{} error: {}", id, e);
                    }
                }
            } else {
                // Check if we should exit - modified to allow exit even when count = 0
                if flags.fermentation_complete.load(Ordering::SeqCst) {
                    // If fermentation is complete, this station should exit
                    // regardless of whether it processed any dough
                    break;
                }
                // No fermentation receivers available, wait a bit before checking again
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            }
        }
        println!(
            "🏁 Proving station #{} finished, proved {} bases",
            id, count
        );

        // Only mark the proving stage as complete when ALL stations have finished
        static mut FINISHED_STATIONS: std::sync::atomic::AtomicUsize =
            std::sync::atomic::AtomicUsize::new(0);
        unsafe {
            let prev_count = FINISHED_STATIONS.fetch_add(1, Ordering::SeqCst);
            // Check if we're the last station to finish (assuming 10 stations from default config)
            if prev_count + 1 >= 10 {
                flags.proving_complete.store(true, Ordering::SeqCst);
                println!("🏁 All proving stations have finished their work");
            }
        }
    })
}

fn spawn_base_preparation_station(
    kitchen: Arc<Kitchen>,
    input_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>>,
    output_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>>,
    id: usize,
    flags: Arc<StationFlags>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let chef = Chef;
        let mut count = 0;
        println!("👨‍🍳 Base preparation station #{} starting work", id);

        loop {
            // Try to dequeue with non-blocking approach
            if let Some(dough) = input_queue.dequeue() {
                // Directly use the proved dough from the queue
                match chef.prepare_pizza_base(&kitchen, dough).await {
                    Ok(rounded_dough) => {
                        count += 1;
                        println!(
                            "🥖 Base preparation station #{} prepared base #{}",
                            id, count
                        );

                        // Try to enqueue with backoff if queue is full
                        let mut dough = rounded_dough;
                        loop {
                            match output_queue.enqueue(dough) {
                                Ok(_) => break,
                                Err(returned_dough) => {
                                    // Queue is full, wait a bit and try again
                                    dough = returned_dough;
                                    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        flags.base_prep_complete.store(true, Ordering::SeqCst);
                        println!("❌ Base preparation station #{} error: {}", id, e)
                    }
                }
            } else {
                // Check if we should exit - modified to better handle completion
                if flags.proving_complete.load(Ordering::SeqCst) {
                    // If all proving stations are done AND the queue is empty, we should exit
                    // Small delay to ensure any items in transit have time to arrive
                    tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
                    if input_queue.dequeue().is_none() {
                        break;
                    }
                }
                // No dough available, wait a bit before checking again
                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
            }
        }
        println!(
            "🏁 Base preparation station #{} finished, prepared {} bases",
            id, count
        );
        flags.base_prep_complete.store(true, Ordering::SeqCst);
    })
}

fn spawn_sauce_station(
    kitchen: Arc<Kitchen>,
    output_queue: Arc<MpMcQueue<Tomato, QUEUE_CAPACITY>>,
    target_pizzas: usize,
    id: usize,
    flags: Arc<StationFlags>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let chef = Chef;
        let mut count = 0;
        println!("👨‍🍳 Sauce station #{} starting work", id);

        for _ in 0..target_pizzas {
            match chef.puree_sauce(&kitchen).await {
                Ok(sauce) => {
                    count += 1;
                    println!("🍅 Sauce station #{} prepared sauce #{}", id, count);

                    // Try to enqueue with backoff if queue is full
                    let mut sauce_item = sauce;
                    loop {
                        match output_queue.enqueue(sauce_item) {
                            Ok(_) => break,
                            Err(returned_sauce) => {
                                // Queue is full, wait a bit and try again
                                sauce_item = returned_sauce;
                                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    flags.sauce_complete.store(true, Ordering::SeqCst);
                    println!("❌ Sauce station #{} error: {}", id, e)
                }
            }
        }
        println!(
            "🏁 Sauce station #{} finished, prepared {} sauces",
            id, count
        );
        flags.sauce_complete.store(true, Ordering::SeqCst);
    })
}

fn spawn_cheese_station(
    kitchen: Arc<Kitchen>,
    output_queue: Arc<MpMcQueue<Cheese, QUEUE_CAPACITY>>,
    target_pizzas: usize,
    id: usize,
    flags: Arc<StationFlags>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let chef = Chef;
        let mut count = 0;
        println!("👨‍🍳 Cheese station #{} starting work", id);

        for _ in 0..target_pizzas {
            match chef.shred_cheese(&kitchen).await {
                Ok(cheese) => {
                    count += 1;
                    println!("🧀 Cheese station #{} shredded cheese #{}", id, count);

                    // Try to enqueue with backoff if queue is full
                    let mut cheese_item = cheese;
                    loop {
                        match output_queue.enqueue(cheese_item) {
                            Ok(_) => break,
                            Err(returned_cheese) => {
                                // Queue is full, wait a bit and try again
                                cheese_item = returned_cheese;
                                tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                            }
                        }
                    }
                }
                Err(e) => {
                    flags.cheese_complete.store(true, Ordering::SeqCst);
                    println!("❌ Cheese station #{} error: {}", id, e)
                }
            }
        }
        println!(
            "🏁 Cheese station #{} finished, shredded {} cheeses",
            id, count
        );
        flags.cheese_complete.store(true, Ordering::SeqCst);
    })
}

fn spawn_assembly_station(
    kitchen: Arc<Kitchen>,
    base_queue: Arc<MpMcQueue<Dough, QUEUE_CAPACITY>>,
    sauce_queue: Arc<MpMcQueue<Tomato, QUEUE_CAPACITY>>,
    cheese_queue: Arc<MpMcQueue<Cheese, QUEUE_CAPACITY>>,
    completed_queue: Arc<MpMcQueue<Pizza, QUEUE_CAPACITY>>,
    id: usize,
    flags: Arc<StationFlags>,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let chef = Chef;
        let mut count = 0;
        println!("👨‍🍳 Assembly station #{} starting work", id);

        loop {
            // Try to get all ingredients
            let base = match base_queue.dequeue() {
                Some(b) => b,
                None => {
                    // Check if we should exit
                    if count > 0 && flags.base_prep_complete.load(Ordering::SeqCst) {
                        break;
                    }
                    // No base available, wait a bit before checking again
                    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                    continue;
                }
            };

            let sauce = match sauce_queue.dequeue() {
                Some(s) => s,
                None => {
                    // If we got a base but no sauce, put the base back
                    let _ = base_queue.enqueue(base);
                    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                    continue;
                }
            };

            let cheese = match cheese_queue.dequeue() {
                Some(c) => c,
                None => {
                    // If we got a base and sauce but no cheese, put them back
                    let _ = base_queue.enqueue(base);
                    let _ = sauce_queue.enqueue(sauce);
                    tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;
                    continue;
                }
            };

            println!(
                "🧩 Assembly station #{} has all ingredients for pizza #{}",
                id,
                count + 1
            );

            match chef
                .combine_cold_ingredients_to_form_uncooked_pizza(&kitchen, base, sauce, cheese)
                .await
            {
                Ok(uncooked) => {
                    println!("🧩 Assembly station #{} assembled pizza #{}", id, count + 1);

                    match chef.cook_pizza(&kitchen, uncooked).await {
                        Ok(cooked) => {
                            count += 1;
                            println!("🔥 Assembly station #{} cooked pizza #{}", id, count);

                            // Try to enqueue completed pizza with backoff if queue is full
                            let mut pizza = cooked;
                            loop {
                                match completed_queue.enqueue(pizza) {
                                    Ok(_) => {
                                        println!(
                                            "🎉 Assembly station #{} completed pizza #{}",
                                            id, count
                                        );
                                        break;
                                    }
                                    Err(returned_pizza) => {
                                        // Queue is full, wait a bit and try again
                                        pizza = returned_pizza;
                                        tokio::time::sleep(tokio::time::Duration::from_millis(1))
                                            .await;
                                    }
                                }
                            }
                        }
                        Err(e) => println!("❌ Assembly station #{} cooking error: {}", id, e),
                    }
                }
                Err(e) => println!("❌ Assembly station #{} assembly error: {}", id, e),
            }
        }
        println!(
            "🏁 Assembly station #{} finished, completed {} pizzas",
            id, count
        );
    })
}
