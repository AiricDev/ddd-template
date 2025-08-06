use airic_mesh::{
    adapters::file_keystore::FileKeystore,
    core::{
        application_message::ApplicationMessage,
        errors::Result,
    },
    use_cases::{
        provision_new_device::ProvisionNewDevice,
        ports::mesh_repository::MeshRepository,
    },
};
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎯 Airic Mesh - Simple Demo");
    println!("============================");

    // Initialize storage
    let storage_path = "/tmp/airic-mesh-demo";
    let mesh_repo = Arc::new(FileKeystore::new(storage_path)?);
    println!("✅ Storage initialized at: {}", storage_path);

    // Check if we have existing devices
    let device_ids = mesh_repo.get_all_device_ids().await?;
    println!("📱 Found {} existing device(s)", device_ids.len());

    if device_ids.is_empty() {
        // Create a new device
        println!("🆕 Creating new device...");
        let provisioner = ProvisionNewDevice::new(mesh_repo.clone());
        let (device, invitation) = provisioner.execute("Demo Device".to_string()).await?;
        
        println!("✅ New device created:");
        println!("   ID: {}", device.id);
        println!("   Name: {}", device.name);
        println!("   Created: {}", device.created_at);
        
        println!("\n📋 Invitation (share this with other devices):");
        println!("{}", invitation);
        
        // Create a test message
        let test_message = ApplicationMessage {
            payload: serde_json::json!({
                "type": "demo",
                "content": "Hello from Airic Mesh!",
                "features": [
                    "End-to-end encryption",
                    "Perfect forward secrecy", 
                    "Device provisioning",
                    "Clean architecture"
                ]
            }),
            timestamp: chrono::Utc::now(),
        };
        
        println!("\n📤 Example message created:");
        println!("   Payload: {}", serde_json::to_string_pretty(&test_message.payload)?);
        println!("   Timestamp: {}", test_message.timestamp);
        
    } else {
        // Show existing devices
        println!("📱 Existing devices:");
        for (i, device_id) in device_ids.iter().enumerate() {
            let device = mesh_repo.load_device(*device_id).await?;
            println!("   {}. {} (ID: {})", i + 1, device.name, device.id);
        }
    }

    println!("\n💡 Next steps:");
    println!("   1. Share the invitation with another device");
    println!("   2. Run this demo on the other device with the invitation");
    println!("   3. Set up an MQTT broker to enable message exchange");
    println!("   4. Use the full application for real communication");

    println!("\n✅ Demo completed successfully!");
    Ok(())
} 