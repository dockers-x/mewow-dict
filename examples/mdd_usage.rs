/// Example demonstrating how to use the MDD parser
/// 
/// This example shows:
/// 1. How to load an MDD file
/// 2. How to iterate through resources
/// 3. How to extract specific resources
/// 
/// To run this example, you need to have an MDD file available.
/// Place it in the resources/mdx directory and update the filename below.
/// 
/// Run with: cargo run --example mdd_usage

use std::fs;

fn main() {
    println!("MDD File Parser Example");
    println!("=======================\n");

    // Example 1: Loading an MDD file
    // Uncomment and modify the path to test with a real MDD file
    /*
    let mdd_path = "resources/mdx/example.mdd";
    println!("Loading MDD file: {}", mdd_path);
    
    match fs::read(mdd_path) {
        Ok(data) => {
            println!("File loaded: {} bytes", data.len());
            
            // Parse the MDD file
            let mdd = mewow_dict::mdict::mdd::Mdd::new(&data);
            
            println!("Encoding: {}", mdd.encoding);
            println!("Encrypted: {}", mdd.encrypted);
            println!("Total resources: {}", mdd.resources_offset.len());
            
            // Example 2: List first 10 resources
            println!("\nFirst 10 resources:");
            for (i, entry) in mdd.entries().take(10).enumerate() {
                println!("  {}. {}", i + 1, entry.path);
            }
            
            // Example 3: Get a specific resource
            let resource_path = "example.png"; // Change this to an actual resource path
            if let Some(data) = mdd.get_resource_by_path(resource_path) {
                println!("\nFound resource: {}", resource_path);
                println!("Size: {} bytes", data.len());
                
                // Save to file for verification
                fs::write(format!("output_{}", resource_path), data)
                    .expect("Failed to write resource to file");
                println!("Saved to: output_{}", resource_path);
            } else {
                println!("\nResource not found: {}", resource_path);
            }
        }
        Err(e) => {
            println!("Error loading file: {}", e);
        }
    }
    */
    
    println!("\nTo use this example:");
    println!("1. Uncomment the code above");
    println!("2. Place an MDD file in resources/mdx/");
    println!("3. Update the mdd_path variable");
    println!("4. Update the resource_path variable to match a resource in your MDD file");
    println!("5. Run: cargo run --example mdd_usage");
}
