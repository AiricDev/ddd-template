## Layer 1: Core Entities (Domain Layer)

This layer contains the core business objects and rules of the system. It is the most stable and has no dependencies on any other layer. We will use `vodozemac`'s data structures as the foundation for our entities.

**Entities to Implement:**
*   **`Device`**: A wrapper around a `vodozemac::Account`. This entity represents a single cryptographic identity in the mesh.
    *   **Attributes**: A unique `device_id`, the pickled (serialized) `vodozemac::Account` string, and metadata like a user-friendly name (e.g., "My Laptop").
    *   **Logic**: Methods to pickle/unpickle the account for persistence. It should not know *how* it's stored, only that it can be serialized.

*   **`Session`**: A wrapper around a `vodozemac::Session`. This represents a secure communication channel between two `Devices`.
    *   **Attributes**: A unique `session_id`, the IDs of the two devices involved, and the pickled `vodozemac::Session` string.
    *   **Logic**: Methods for serialization and deserialization.

*   **`ApplicationMessage`**: A simple struct representing the data your application wants to send.
    *   **Attributes**: `payload` (e.g., `Vec` or a JSON string), `timestamp`.

*   **`WireMessage`**: The encrypted message format that will actually be sent over the network.
    *   **Attributes**: The encrypted `ciphertext` from `vodozemac`, the necessary `header` information (ratchet key, message number, etc.), and the sender's `device_id`.

## Layer 2: Use Cases (Application Business Rules)

This layer orchestrates the flow of data using the core entities to perform specific tasks. It defines interfaces (ports) that are implemented by the outer layer.

**Interfaces (Ports) to Define (as Rust `traits`):**
*   **`MeshRepository`**: An interface for all persistence needs.
    *   **Methods**: `save_device(&Device)`, `load_device(device_id)`, `get_all_device_ids()`, `save_session(&Session)`, `load_session(session_id)`, `delete_device(device_id)`.
*   **`NetworkTransport`**: An interface for sending and receiving data.
    *   **Methods**: `send(destination_device_id, WireMessage)`, `set_message_handler(handler: fn(WireMessage))`.

**Use Case Interactors to Implement:**
*   **`ProvisionNewDevice`**:
    *   **Input**: User-provided device name.
    *   **Logic**:
        1.  Creates a new `vodozemac::Account` to establish a cryptographic identity.
        2.  Wraps it in your `Device` entity.
        3.  Saves the new `Device` using the `MeshRepository`.
        4.  For every other existing `Device` in the mesh (retrieved via `MeshRepository`), it establishes a new `vodozemac::Session` and saves it.
    *   **Output**: The new `Device`'s ID and an "invitation" (e.g., the public part of its identity key) to share with other devices.

*   **`FinalizeProvisioning`**:
    *   **Input**: An "invitation" from a new device.
    *   **Logic**:
        1.  Uses the invitation to create a new `Session` with the new device.
        2.  Saves the `Session` via the `MeshRepository`.

*   **`RevokeDevice`**:
    *   **Input**: `device_id` to be removed.
    *   **Logic**:
        1.  Loads the target `Device` and all related `Sessions` via the `MeshRepository`.
        2.  Deletes the `Device` and its `Sessions` from persistence.
        3.  Broadcasts a control message to all *remaining* devices via `NetworkTransport` to inform them to discard their `Session` with the revoked device.

*   **`SendMessage`**:
    *   **Input**: `recipient_device_id`, `ApplicationMessage` (plaintext).
    *   **Logic**:
        1.  Loads the `Session` for the given recipient using the `MeshRepository`.
        2.  Uses the `vodozemac::Session` to encrypt the `ApplicationMessage` into a `WireMessage`.
        3.  Sends the `WireMessage` using the `NetworkTransport`.
        4.  Saves the updated (ratcheted) `Session` state back to the `MeshRepository`.

*   **`HandleIncomingMessage`**:
    *   **Input**: A `WireMessage` received from the `NetworkTransport`.
    *   **Logic**:
        1.  Loads the `Session` corresponding to the sender.
        2.  Uses `vodozemac` to decrypt the `WireMessage` into an `ApplicationMessage`.
        3.  Saves the updated `Session` state.
        4.  Passes the decrypted `ApplicationMessage` up to the application layer.

## Layer 3: Interface Adapters

This layer contains concrete implementations of the interfaces (ports) defined in the use case layer. It's the bridge between your application and external tools.

**Adapters to Implement:**
*   **`FileKeystore` (implements `MeshRepository`)**:
    *   **Technology**: Uses the standard filesystem (`std::fs`).
    *   **Logic**: Serializes `Device` and `Session` entities into a structured format (like JSON or TOML) and stores them in a dedicated directory. Each device and session gets its own file, named by its ID (e.g., `~/.config/your_app/devices/{device_id}.json`). It handles file reading, writing, and deletion.

*   **`MqttTransport` (implements `NetworkTransport`)**:
    *   **Technology**: Uses an MQTT client library like `rumqttc`.
    *   **Logic**:
        *   **Initialization**: Connects to an MQTT broker and subscribes to a device-specific topic (e.g., `personal-mesh/devices/{my_device_id}`).
        *   **`send()`**: Publishes the serialized `WireMessage` to the recipient's topic (`personal-mesh/devices/{recipient_device_id}`).
        *   **`set_message_handler()`**: The MQTT client loop will call the provided handler function whenever a message arrives on the subscribed topic. This handler will then trigger the `HandleIncomingMessage` use case.

## Layer 4: Frameworks & Drivers

This is the outermost layer, containing the specific tools and frameworks that your adapters use. You don't write this code, but your adapters depend on it.
*   **Frameworks**: `vodozemac`, `rumqttc`, `serde`.
*   **Drivers**: The actual MQTT Broker (e.g., Mosquitto), the operating system's filesystem.

By structuring your project this way, you can test your entire application logic (entities and use cases) without needing a network connection or a real filesystem, simply by providing mock implementations of the `MeshRepository` and `NetworkTransport` traits.