//! Assetto Corsa SDK Example
//!
//! This example demonstrates basic usage of the Assetto Corsa SDK.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Assetto Corsa SDK - Simple Example");
    println!("Connecting to Assetto Corsa...");

    let mut ac = race_sdk::ac::AssettoCorsa::new();

    // Connect to AC
    match ac.connect() {
        Ok(()) => println!("Connected successfully!"),
        Err(e) => {
            println!("Failed to connect: {}", e);
            return Ok(());
        }
    }

    if ac.is_connected() {
        println!("Monitoring for 10 seconds...");

        // Run for 10 seconds with simple polling
        let start = std::time::Instant::now();

        while start.elapsed() < std::time::Duration::from_secs(10) {
            // Get physics data
            if let Some(physics) = ac.get_physics() {
                println!(
                    "[Physics] Speed: {:.1} km/h | RPM: {} | Gear: {}",
                    physics.speed_kmh, physics.rpms, physics.gear
                );
            }

            // Get graphics data
            if let Some(graphics) = ac.get_graphics() {
                println!(
                    "[Graphics] Status: {} | Position: {} | Laps: {}",
                    graphics.status, graphics.position, graphics.completed_laps
                );
            }

            // Get static info (once is enough)
            if let Some(info) = ac.get_static_info() {
                println!("[Static] Car: {} | Track: {}", info.car_model_str(), info.track_str());
            }

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }

    println!("Example completed.");
    Ok(())
}
