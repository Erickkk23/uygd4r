use life_of_pie::{
    parallel::{data_parallel_production, task_parallel_production},
    Kitchen,
};
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: {} <mode> <num_chefs_or_pizzas>", args[0]);
        println!("Modes:");
        println!("  task <num_pizzas> - Task parallel mode with fixed stations");
        println!("  data <num_chefs>  - Data parallel mode with specified number of chefs");
        return;
    }

    let mode = &args[1];
    let num = args[2].parse().expect("Second argument must be a number");

    println!("Running {} pizza production", mode);

    let kitchen = Arc::new(Kitchen::default());

    match mode.as_str() {
        "task" => task_parallel_production(&kitchen, num).await,
        "data" => {
            data_parallel_production(&kitchen, num).await;
        }
        _ => println!("Unknown mode: {}", mode),
    }
}
