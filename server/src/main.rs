use tokio::time::Duration;
use tokio::task;

#[tokio::main]
async fn main() {
    // Create a Tokio runtime and run the asynchronous task.
    let result = task::spawn(async {
        // This asynchronous block will run concurrently.
        println!("Hello, World from Tokio!");

        // Simulate some async work using Tokio's sleep function.
        tokio::time::sleep(Duration::from_secs(2)).await;

        println!("Delayed Hello, World!");
    })
    .await;

    // Handle any errors.
    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
    }
}