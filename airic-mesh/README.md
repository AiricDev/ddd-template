# Airic Mesh

A personal trusted mesh network library for secure device-to-device communication.

## Overview

Airic Mesh is a Rust library that implements a secure, end-to-end encrypted communication protocol for personal device networks. It uses the Double Ratchet algorithm (via `vodozemac`) to provide perfect forward secrecy and secure messaging between trusted devices.

## Features

- 🔐 **End-to-End Encryption**: All messages are encrypted using the Double Ratchet algorithm
- 🤝 **Device Provisioning**: Secure device onboarding with invitation-based trust establishment
- 📱 **Multi-Device Support**: Connect multiple devices to your personal mesh
- 🗂️ **Persistent Storage**: Device and session state is stored locally
- 🌐 **MQTT Transport**: Uses MQTT for message routing (BYOB - Bring Your Own Broker)
- 🏗️ **Clean Architecture**: Well-structured, testable code following Clean Architecture principles

## Quick Start

### Prerequisites

1. **MQTT Broker**: You'll need an MQTT broker running. You can use:
   - [Mosquitto](https://mosquitto.org/) (local development)
   - [Cloud MQTT](https://www.cloudmqtt.com/) (hosted)
   - Any other MQTT broker

2. **Rust**: Make sure you have Rust installed

### Running the Example Application

1. **Clone and build**:
   ```bash
   git clone <repository-url>
   cd airic-mesh
   cargo build
   ```

2. **Set up environment variables** (optional):
   ```bash
   export MQTT_HOST=localhost
   export MQTT_PORT=1883
   export MESH_STORAGE_PATH=~/.airic-mesh
   ```

3. **Run the application**:
   ```bash
   cargo run
   ```

### First Run

When you run the application for the first time, it will:

1. Create a new device identity
2. Generate an invitation string
3. Start listening for messages

The invitation string can be shared with other devices to add them to your mesh.

### Adding Another Device

To add a second device to your mesh:

1. Run the application on the second device
2. Copy the invitation string from the first device
3. Use the invitation to establish a secure connection

## Architecture

The library follows Clean Architecture principles with four layers:

### Layer 1: Core Entities
- `Device`: Represents a cryptographic identity
- `Session`: Secure communication channel between devices
- `ApplicationMessage`: User-level data to be sent
- `WireMessage`: Encrypted message format for transport

### Layer 2: Use Cases
- `ProvisionNewDevice`: Creates new devices and invitations
- `FinalizeProvisioning`: Accepts invitations from new devices
- `SendMessage`: Encrypts and sends messages
- `HandleIncomingMessage`: Decrypts and processes incoming messages
- `RevokeDevice`: Removes devices from the mesh

### Layer 3: Interface Adapters
- `FileKeystore`: Filesystem-based storage for devices and sessions
- `MqttTransport`: MQTT-based message transport

### Layer 4: Frameworks & Drivers
- `vodozemac`: Cryptographic library
- `rumqttc`: MQTT client
- `tokio`: Async runtime

## Configuration

The example application can be configured via environment variables:

- `MQTT_HOST`: MQTT broker hostname (default: localhost)
- `MQTT_PORT`: MQTT broker port (default: 1883)
- `MESH_STORAGE_PATH`: Local storage directory (default: ~/.airic-mesh)

## Security Model

- **Trust Establishment**: Devices are explicitly paired via invitation
- **Perfect Forward Secrecy**: Each message uses a new encryption key
- **Self-Healing**: Sessions automatically recover from key compromise
- **No Central Authority**: All security is enforced client-side

## Development Status

This is a proof-of-concept implementation. The core architecture is complete, but some features are still in development:

- ✅ Core entities and use cases
- ✅ File-based storage adapter
- ✅ MQTT transport adapter
- ✅ Basic device provisioning
- 🔄 Session creation (placeholder implementation)
- 🔄 Message encryption/decryption (placeholder implementation)

## Contributing

This project is in early development. Contributions are welcome!

## License

[Add your license here] 