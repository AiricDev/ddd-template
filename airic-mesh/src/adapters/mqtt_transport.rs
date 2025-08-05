use crate::{
    core::{errors::Result, wire_message::WireMessage},
    use_cases::ports::network_transport::{MessageHandler, NetworkTransport},
};
use async_trait::async_trait;
use rumqttc::{self, AsyncClient, MqttOptions, QoS};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct MqttTransport {
    client: AsyncClient,
    my_device_id: Uuid,
    message_handler: Arc<Mutex<Option<MessageHandler>>>,
}

impl MqttTransport {
    pub async fn new(host: &str, port: u16, my_device_id: Uuid) -> Result<Self> {
        let mut mqttoptions = MqttOptions::new(my_device_id.to_string(), host, port);
        mqttoptions.set_keep_alive(std::time::Duration::from_secs(5));

        let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
        let message_handler: Arc<Mutex<Option<MessageHandler>>> = Arc::new(Mutex::new(None));
        let handler_clone = message_handler.clone();

        let topic = format!("/mesh/{}", my_device_id);
        client
            .subscribe(topic, QoS::AtLeastOnce)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

        tokio::spawn(async move {
            loop {
                match eventloop.poll().await {
                    Ok(rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish))) => {
                        if let Ok(wire_message) = serde_json::from_slice(&publish.payload) {
                            if let Some(handler) = &*handler_clone.lock().await {
                                if let Err(e) = handler(wire_message).await {
                                    eprintln!("Error handling incoming message: {:?}", e);
                                }
                            }
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("MQTT Error: {:?}", e);
                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                    }
                }
            }
        });

        Ok(Self {
            client,
            my_device_id,
            message_handler,
        })
    }
}

#[async_trait]
impl NetworkTransport for MqttTransport {
    async fn send(&self, destination_device_id: Uuid, message: WireMessage) -> Result<()> {
        let topic = format!("/mesh/{}", destination_device_id);
        let payload = serde_json::to_vec(&message)?;
        self.client
            .publish(topic, QoS::AtLeastOnce, false, payload)
            .await
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        Ok(())
    }

    fn set_message_handler(&mut self, handler: MessageHandler) {
        let mut guard = self.message_handler.blocking_lock();
        *guard = Some(handler);
    }
}
