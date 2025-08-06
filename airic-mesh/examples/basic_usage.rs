use airic_mesh::{
    adapters::{file_keystore::FileKeystore, mqtt_transport::MqttTransport},
    core::{
        application_message::ApplicationMessage,
        errors::Result,
    },
    use_cases::{
        provision_new_device::ProvisionNewDevice,
        send_message::SendMessage,
    },
};
use std::sync::Arc;
use tokio;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    println!("📚 Airic Mesh - Basic Usage Example");
    println!("=====================================");

    // Initialize storage
    let storage_path = "/tmp/airic-mesh-example";
    let mesh_repo = Arc::new(FileKeystore::new(storage_path)?);
    println!("✅ Storage initialized at: {}", storage_path);

    // Create a new device
    let provisioner = ProvisionNewDevice::new(mesh_repo.clone());
    let (device, invitation) = provisioner.execute("Example Device".to_string()).await?;
    
    println!("✅ Device created:");
    println!("   ID: {}", device.id);
    println!("   Name: {}", device.name);
    println!("   Created: {}", device.created_at);
    
    println!("\n📋 Invitation:");
    println!("{}", invitation);

    // Initialize transport (this would normally connect to a real MQTT broker)
    let transport = Arc::new(MqttTransport::new("localhost", 1883, device.id).await?);
    println!("✅ Transport initialized");

    // Create a message sender
    let sender = SendMessage::new(
        mesh_repo.clone(),
        transport.clone(),
        device.id,
    );

    // Create a test message
    let test_message = ApplicationMessage {
        payload: serde_json::json!({
            "type": "greeting",
            "content": "Hello from the example!",
            "timestamp": chrono::Utc::now().to_rfc3339(),
        }),
        timestamp: chrono::Utc::now(),
    };

    println!("\n📤 Test message created:");
    println!("   Payload: {}", serde_json::to_string_pretty(&test_message.payload)?);
    println!("   Timestamp: {}", test_message.timestamp);

    // Note: In a real scenario, you would have another device to send to
    // For this example, we'll just show the message structure
    println!("\n💡 In a real scenario, you would:");
    println!("   1. Share the invitation with another device");
    println!("   2. Use FinalizeProvisioning to accept the invitation");
    println!("   3. Send messages using the SendMessage use case");

    println!("\n✅ Example completed successfully!");
    Ok(())
} 