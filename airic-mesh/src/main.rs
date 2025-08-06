use airic_mesh::{
    adapters::{file_keystore::FileKeystore, mqtt_transport::MqttTransport},
    core::{
        application_message::ApplicationMessage,
        errors::Result,
    },
    use_cases::{
        handle_incoming_message::HandleIncomingMessage,
        provision_new_device::ProvisionNewDevice,
        send_message::SendMessage,
        ports::{mesh_repository::MeshRepository, network_transport::NetworkTransport},
    },
};
use std::sync::Arc;
use tokio;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Airic Mesh Example Application");
    println!("==================================");

    // Configuration
    let storage_path = std::env::var("MESH_STORAGE_PATH").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        format!("{}/.airic-mesh", home)
    });
    
    let mqtt_host = std::env::var("MQTT_HOST").unwrap_or_else(|_| "localhost".to_string());
    let mqtt_port = std::env::var("MQTT_PORT")
        .unwrap_or_else(|_| "1883".to_string())
        .parse::<u16>()
        .expect("Invalid MQTT port");

    println!("📁 Storage path: {}", storage_path);
    println!("🌐 MQTT broker: {}:{}", mqtt_host, mqtt_port);

    // Initialize adapters
    let mesh_repo = Arc::new(FileKeystore::new(&storage_path)?);
    let transport = Arc::new(MqttTransport::new(&mqtt_host, mqtt_port, Uuid::new_v4()).await?);

    // Check if this is a new device or existing device
    let device_ids = mesh_repo.get_all_device_ids().await?;
    
    let local_device_id = if device_ids.is_empty() {
        println!("🆕 No existing devices found. Creating new device...");
        
        // Create a new device
        let provisioner = ProvisionNewDevice::new(mesh_repo.clone());
        let (device, invitation) = provisioner.execute("My Device".to_string()).await?;
        
        println!("✅ New device created with ID: {}", device.id);
        println!("📋 Invitation (share this with other devices):");
        println!("{}", invitation);
        
        device.id
    } else {
        println!("📱 Found {} existing device(s)", device_ids.len());
        device_ids[0] // Use the first device for simplicity
    };

    // Set up message handling
    let mut transport_with_handler = MqttTransport::new(&mqtt_host, mqtt_port, local_device_id).await?;
    let mesh_repo_for_handler = mesh_repo.clone();
    
    transport_with_handler.set_message_handler(Box::new(move |wire_message| {
        let mesh_repo = mesh_repo_for_handler.clone();
        let local_device_id = local_device_id;
        Box::pin(async move {
            let handler = HandleIncomingMessage::new(mesh_repo, local_device_id);
            match handler.execute(wire_message).await {
                Ok(app_message) => {
                    println!("📨 Received message: {:?}", app_message);
                    Ok(())
                }
                Err(e) => {
                    eprintln!("❌ Error handling message: {:?}", e);
                    Ok(())
                }
            }
        })
    }));

    // Example: Send a test message if there are other devices
    if device_ids.len() > 1 {
        let sender = SendMessage::new(
            mesh_repo.clone(),
            transport.clone(),
            local_device_id,
        );

        let test_message = ApplicationMessage {
            payload: serde_json::json!({
                "type": "test",
                "content": "Hello from Airic Mesh!"
            }),
            timestamp: chrono::Utc::now(),
        };

        // Send to the second device (if it exists)
        let recipient_id = device_ids[1];
        println!("📤 Sending test message to device: {}", recipient_id);
        
        match sender.execute(recipient_id, test_message).await {
            Ok(()) => println!("✅ Message sent successfully"),
            Err(e) => eprintln!("❌ Failed to send message: {:?}", e),
        }
    }

    // Keep the application running
    println!("⏳ Application running... Press Ctrl+C to exit");
    tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
    println!("👋 Goodbye!");

    Ok(())
} 