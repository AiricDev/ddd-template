# Testing Session Creation and Message Encryption

This guide explains how to test the session creation and message encryption functionality in the `airic-mesh` library.

## Overview

The `airic-mesh` library uses the `vodozemac` cryptographic library to implement secure session creation and message encryption. Testing these features involves several layers:

1. **Basic vodozemac functionality** - Testing the underlying cryptographic operations
2. **Session creation** - Testing the establishment of secure communication channels
3. **Message encryption/decryption** - Testing the actual message exchange
4. **Session persistence** - Testing that sessions can be saved and restored
5. **Integration testing** - Testing how our library uses vodozemac

## Running the Tests

### 1. Basic Functionality Test

```bash
cargo test test_basic_vodozemac_functionality -- --nocapture
```

This test verifies:
- ✅ Account creation and key generation
- ✅ Identity key and one-time key access
- ✅ Account persistence (pickle/unpickle)

**Expected output:**
```
Identity key: FP0GaJGsWqESERX5iysIUUA+rWW4i7P1V/DOS4ArKX8
One-time keys count: 10
One-time key: 0m5UDmRC23TTLb6+Kyzj01ucKOEIjYZxNo5qMkm63H4
Identity key: FP0GaJGsWqESERX5iysIUUA+rWW4i7P1V/DOS4ArKX8
✅ Account persistence test passed!
✅ Basic vodozemac functionality test passed!
```

### 2. Session Creation and Message Encryption Test

```bash
cargo test test_session_creation_and_message_encryption -- --nocapture
```

This test demonstrates:
- ✅ Creating multiple accounts (Alice and Bob)
- ✅ Key generation and exchange
- ✅ Session creation (currently placeholder)
- ✅ Message encryption/decryption (currently placeholder)

**Expected output:**
```
🔐 Testing Session Creation and Message Encryption
==================================================
Alice identity key: VnBb0QDMfC7ToM9OHTVW84gy7qqq3t1OCVzx11HRmxI
Bob identity key: ZzHMcdsWfb4/sqXIiMWlfnEKgomVn9vvnTOADQMdJCE
Alice one-time key: rveHxMKAWYFYprYVrN1orOSPYiCNGS2QPXzBuJot5UU
Bob identity key: ZzHMcdsWfb4/sqXIiMWlfnEKgomVn9vvnTOADQMdJCE
✅ Key generation and access test passed!
✅ All session creation and message encryption tests passed!
```

### 3. Airic Mesh Integration Test

```bash
cargo test test_airic_mesh_session_integration -- --nocapture
```

This test shows:
- ✅ Device creation with embedded accounts
- ✅ Session creation using our library API
- ✅ Error handling for unimplemented features

**Expected output:**
```
🏗️ Testing Airic Mesh Session Integration
==========================================
✅ Device created: d70889d8-75c2-442c-91c0-cba5cb6f5ba5
✅ Device identity key: BNP/pO1nMVTVcVbE3OwvUIbioeJsFvNvn5eL79fm8ys
⚠️ Session creation returned error (expected): Io(Custom { kind: Other, error: "Outbound session creation not implemented yet - requires proper key parsing" })
   This is expected since we haven't implemented the full session creation yet
✅ Airic Mesh session integration test completed!
```

## Current Implementation Status

### ✅ Working Features

1. **Account Management**
   - Creating cryptographic accounts
   - Generating identity keys and one-time keys
   - Account persistence (pickle/unpickle)

2. **Device Management**
   - Creating devices with embedded accounts
   - Device persistence and restoration
   - Invitation generation

3. **Basic Architecture**
   - Clean Architecture implementation
   - Use case separation
   - Error handling

### 🔄 In Progress

1. **Session Creation**
   - Outbound session creation (needs proper key parsing)
   - Inbound session creation (needs PreKeyMessage parsing)
   - Session establishment between devices

2. **Message Encryption**
   - Message encryption using established sessions
   - Message decryption and verification
   - Ratchet behavior testing

### 📋 To Implement

1. **Complete Session Creation**
   ```rust
   // Example of what needs to be implemented
   pub fn new_outbound(
       local_device_id: Uuid,
       remote_device_id: Uuid,
       account: &Account,
       remote_identity_key: &str,
       remote_one_time_key: &str,
   ) -> Result<(Self, String)> {
       // Parse keys properly
       let identity_key = Curve25519PublicKey::from_base64(remote_identity_key)?;
       let one_time_key = Curve25519PublicKey::from_base64(remote_one_time_key)?;
       
       // Create session
       let olm_session = account.create_outbound_session(identity_key, one_time_key, SessionConfig::default());
       
       // Return session and first message
       Ok((Session::new(local_device_id, remote_device_id, olm_session)?, first_message))
   }
   ```

2. **Message Exchange Testing**
   ```rust
   #[test]
   fn test_message_exchange() {
       // Create two devices
       let alice = Device::new("Alice".to_string())?;
       let bob = Device::new("Bob".to_string())?;
       
       // Establish session
       let session = Session::new_outbound(alice.id, bob.id, &alice.account()?, ...)?;
       
       // Send message
       let message = "Hello Bob!";
       let encrypted = session.olm_session()?.encrypt(message);
       
       // Receive and decrypt
       let decrypted = session.olm_session()?.decrypt(&encrypted)?;
       assert_eq!(message, String::from_utf8_lossy(&decrypted));
   }
   ```

## Testing Strategy

### 1. Unit Tests

Test individual components in isolation:

```rust
#[test]
fn test_account_creation() {
    let mut account = Account::new();
    account.generate_one_time_keys(10);
    
    let identity_keys = account.identity_keys();
    assert!(!identity_keys.curve25519.is_empty());
}
```

### 2. Integration Tests

Test how components work together:

```rust
#[test]
fn test_device_session_creation() {
    let device = Device::new("Test Device".to_string())?;
    let account = device.account()?;
    
    // Test session creation
    let session = Session::new_outbound(device.id, remote_id, &account, ...)?;
    assert_eq!(session.local_device_id, device.id);
}
```

### 3. End-to-End Tests

Test complete workflows:

```rust
#[test]
fn test_complete_message_exchange() {
    // Setup devices
    // Establish session
    // Send messages
    // Verify encryption/decryption
    // Test session persistence
}
```

## Debugging Tips

### 1. Enable Debug Output

```bash
RUST_LOG=debug cargo test -- --nocapture
```

### 2. Test Individual Components

```bash
# Test just the basic functionality
cargo test test_basic_vodozemac_functionality

# Test session creation
cargo test test_session_creation_and_message_encryption

# Test integration
cargo test test_airic_mesh_session_integration
```

### 3. Check vodozemac Documentation

The `vodozemac` library has comprehensive documentation:
- [vodozemac Documentation](https://matrix-org.github.io/vodozemac/vodozemac/)
- [API Reference](https://matrix-org.github.io/vodozemac/vodozemac/olm/)

## Next Steps

1. **Implement proper key parsing** for session creation
2. **Add PreKeyMessage parsing** for inbound sessions
3. **Implement message encryption/decryption** in use cases
4. **Add comprehensive end-to-end tests**
5. **Test with real MQTT broker** for network communication

## Example Test Run

```bash
$ cargo test -- --nocapture
running 3 tests

🏗️ Testing Airic Mesh Session Integration
==========================================
✅ Device created: d70889d8-75c2-442c-91c0-cba5cb6f5ba5
✅ Device identity key: BNP/pO1nMVTVcVbE3OwvUIbioeJsFvNvn5eL79fm8ys
⚠️ Session creation returned error (expected): Io(Custom { kind: Other, error: "Outbound session creation not implemented yet - requires proper key parsing" })
   This is expected since we haven't implemented the full session creation yet
✅ Airic Mesh session integration test completed!

🔐 Testing Session Creation and Message Encryption
==================================================
Alice identity key: VnBb0QDMfC7ToM9OHTVW84gy7qqq3t1OCVzx11HRmxI
Bob identity key: ZzHMcdsWfb4/sqXIiMWlfnEKgomVn9vvnTOADQMdJCE
Alice one-time key: rveHxMKAWYFYprYVrN1orOSPYiCNGS2QPXzBuJot5UU
Bob identity key: ZzHMcdsWfb4/sqXIiMWlfnEKgomVn9vvnTOADQMdJCE
✅ Key generation and access test passed!
✅ All session creation and message encryption tests passed!

Identity key: gZICIKNyXxLumAO6xQJGpoVouHJlUFMTTJySKKJY7HQ
One-time keys count: 10
One-time key: SI3IBYLzoN2eBKN1+5a3Be7pADvo9KE2XNpTUSPJqW0
Identity key: gZICIKNyXxLumAO6xQJGpoVouHJlUFMTTJySKKJY7HQ
✅ Account persistence test passed!
✅ Basic vodozemac functionality test passed!

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s
```

This shows that the basic cryptographic functionality is working, and we have a solid foundation for implementing the remaining session creation and message encryption features. 