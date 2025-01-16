use crate::client::ElevenLabsClient;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;
use crate::conversational_ai::error::ConvAIError;

/// A module that provides a websocket client for interacting with an ElevenLabs' Conversational AI Agent.
pub mod client;
/// A module that provides websocket messages that can be sent to the server.
pub mod client_messages;
/// A module that provides an error type for the ElevenLabs' Conversational AI.
pub mod error;
/// A module that provides websocket messages that are sent to the client from the server.
pub mod server_messages;


/// An error type for the ElevenLabs Conversational AI.
pub type Result<T> = std::result::Result<T, ConvAIError>;

