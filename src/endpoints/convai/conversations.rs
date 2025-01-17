//! Conversations endpoints

use super::*;
use crate::endpoints::convai::agents::{ConversationConfigOverride, LiteralJsonSchema};
use std::collections::HashMap;
use std::string::ToString;
use strum::Display;

/// Get all conversations of agents that user owns. With option to restrict to a specific agent.
///
/// # Example
/// ```no_run
/// use elevenlabs_rs::endpoints::convai::conversations::{
///     CallSuccessful, GetConversations, GetConversationsQuery,
/// };
/// use elevenlabs_rs::{ElevenLabsClient, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let client = ElevenLabsClient::from_env()?;
///
///     let query = GetConversationsQuery::default()
///         //.with_agent_id("agent_id")
///         .with_page_size(10)
///         .with_call_successful(CallSuccessful::Failure);
///
///     let endpoint = GetConversations::with_query(query);
///
///     let resp = client.hit(endpoint).await?;
///
///     println!("{:?}", resp);
///
///     Ok(())
/// }
/// ```
/// See [Get Conversations API reference](https://elevenlabs.io/docs/conversational-ai/api-reference/conversations/get-conversations)
#[derive(Clone, Debug, Default, Serialize)]
pub struct GetConversations {
    query: Option<GetConversationsQuery>,
}

impl ElevenLabsEndpoint for GetConversations {
    const PATH: &'static str = "/v1/convai/conversations";

    const METHOD: Method = Method::GET;

    type ResponseBody = GetConversationsResponse;

    fn query_params(&self) -> Option<QueryValues> {
        self.query.as_ref().map(|q| q.params.clone())
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

impl GetConversations {
    pub fn with_query(query: GetConversationsQuery) -> Self {
        Self { query: Some(query) }
    }
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct GetConversationsQuery {
    params: QueryValues,
}

impl GetConversationsQuery {
    pub fn with_agent_id(mut self, agent_id: impl Into<String>) -> Self {
        self.params.push(("agent_id", agent_id.into()));
        self
    }

    pub fn with_call_successful(mut self, call_successful: CallSuccessful) -> Self {
        self.params
            .push(("call_successful", call_successful.to_string()));
        self
    }

    pub fn with_cursor(mut self, cursor: impl Into<String>) -> Self {
        self.params.push(("cursor", cursor.into()));
        self
    }

    pub fn with_page_size(mut self, page_size: u32) -> Self {
        self.params.push(("page_size", page_size.to_string()));
        self
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetConversationsResponse {
    pub conversations: Vec<Conversation>,
    pub next_cursor: Option<String>,
    pub has_more: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Conversation {
    pub agent_id: String,
    pub agent_name: Option<String>,
    pub conversation_id: String,
    pub start_time_unix_secs: u64,
    pub call_duration_secs: u32,
    pub message_count: u32,
    pub status: ConvoStatus,
    pub call_successful: CallSuccessful,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConvoStatus {
    Done,
    Processing,
}

impl ConvoStatus {
    pub fn is_done(&self) -> bool {
        matches!(*self, ConvoStatus::Done)
    }
    pub fn is_processing(&self) -> bool {
        matches!(*self, ConvoStatus::Processing)
    }
}

#[derive(Clone, Debug, Display, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CallSuccessful {
    #[strum(to_string = "failure")]
    Failure,
    #[strum(to_string = "success")]
    Success,
    #[strum(to_string = "unknown")]
    Unknown,
}

impl CallSuccessful {
    pub fn is_failure(&self) -> bool {
        matches!(*self, CallSuccessful::Failure)
    }
    pub fn is_success(&self) -> bool {
        matches!(*self, CallSuccessful::Success)
    }
    pub fn is_unknown(&self) -> bool {
        matches!(*self, CallSuccessful::Unknown)
    }
}

/// Get the details of a particular conversation
///
/// # Example
/// ```no_run
/// use elevenlabs_rs::endpoints::convai::conversations::GetConversationDetails;
/// use elevenlabs_rs::{ElevenLabsClient, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///     let client = ElevenLabsClient::from_env()?;
///     let endpoint = GetConversationDetails::new("conversation_id");
///     let resp = client.hit(endpoint).await?;
///     println!("{:?}", resp);
///     Ok(())
/// }
/// ```
/// See [Get Conversation Details API reference](https://elevenlabs.io/docs/conversational-ai/api-reference/conversations/get-conversation)
#[derive(Clone, Debug)]
pub struct GetConversationDetails {
    conversation_id: String,
}

impl GetConversationDetails {
    pub fn new(conversation_id: impl Into<String>) -> Self {
        Self {
            conversation_id: conversation_id.into(),
        }
    }
}

impl ElevenLabsEndpoint for GetConversationDetails {
    const PATH: &'static str = "/v1/convai/conversations/:conversation_id";

    const METHOD: Method = Method::GET;

    type ResponseBody = GetConversationDetailsResponse;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![self.conversation_id.and_param(PathParam::ConversationID)]
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct GetConversationDetailsResponse {
    pub agent_id: String,
    pub conversation_id: String,
    pub status: ConvoStatus,
    pub transcript: Vec<Transcript>,
    pub metadata: Metadata,
    pub analysis: Option<Analysis>,
    pub conversation_initiation_client_data: Option<ConvoInitData>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Analysis {
    pub call_successful: CallSuccessful,
    pub data_collection_results: Option<HashMap<String, DataCollectionResult>>,
    pub evaluation_criteria_results: Option<HashMap<String, EvaluationResult>>,
    pub transcript_summary: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DataCollectionResult {
    pub data_collection_id: String,
    pub json_schema: Option<LiteralJsonSchema>,
    pub value: Option<Value>,
    pub rationale: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct EvaluationResult {
    pub criteria_id: String,
    pub result: CallSuccessful,
    pub rationale: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Metadata {
    pub start_time_unix_secs: u64,
    pub call_duration_secs: u32,
    pub cost: Option<u32>,
    pub feedback: Option<ConvoMetadataFeedback>,
    pub authorization_method: Option<AuthorizationMethod>,
    pub charging: Option<Charging>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConvoMetadataFeedback {
    pub overall_score: Option<Score>,
    pub likes: Option<u32>,
    pub dislikes: Option<u32>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorizationMethod {
    Public,
    AuthorizationHeader,
    SignedUrl,
    ShareableLink,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Charging {
    pub dev_discount: Option<bool>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Transcript {
    pub role: Role,
    pub message: Option<String>,
    pub time_in_call_secs: u32,
    pub tool_calls: Option<Vec<ToolCall>>,
    pub tool_results: Option<Vec<ToolResult>>,
    pub feedback: Option<TranscriptFeedback>,
    pub conversation_turn_metrics: Option<HashMap<String, Value>>,
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Agent,
    User,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToolCall {
    pub request_id: String,
    pub tool_name: String,
    pub params_as_json: String,
    pub tool_has_been_called: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ToolResult {
    pub request_id: String,
    pub tool_name: String,
    pub result_value: String,
    pub is_error: bool,
    pub tool_has_been_called: bool,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TranscriptFeedback {
    pub score: Score,
    pub time_in_call_secs: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Score {
    Like,
    Dislike,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ConvoInitData {
    pub conversation_config_override: Option<ConversationConfigOverride>,
    pub custom_llm_extra_body: Option<HashMap<String, Value>>,
}

/// Delete a particular conversation
///
/// # Example
/// ```no_run
/// use elevenlabs_rs::endpoints::convai::conversations::DeleteConversation;
/// use elevenlabs_rs::{ElevenLabsClient, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///    let client = ElevenLabsClient::from_env()?;
///    let endpoint = DeleteConversation::new("conversation_id");
///    let _ = client.hit(endpoint).await?;
///    Ok(())
/// }
/// ```
/// See [Delete Conversation API reference](https://elevenlabs.io/docs/conversational-ai/api-reference/conversations/delete-conversation)
#[derive(Clone, Debug)]
pub struct DeleteConversation {
    conversation_id: String,
}

impl DeleteConversation {
    pub fn new(conversation_id: impl Into<String>) -> Self {
        Self {
            conversation_id: conversation_id.into(),
        }
    }
}

impl ElevenLabsEndpoint for DeleteConversation {
    const PATH: &'static str = "/v1/convai/conversations/:conversation_id";

    const METHOD: Method = Method::DELETE;

    type ResponseBody = ();

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![self.conversation_id.and_param(PathParam::ConversationID)]
    }

    async fn response_body(self, _resp: Response) -> Result<Self::ResponseBody> {
        Ok(())
    }
}

/// Get the audio recording of a particular conversation
///
/// # Example
/// ```no_run
/// use elevenlabs_rs::endpoints::convai::conversations::GetConversationAudio;
/// use elevenlabs_rs::{ElevenLabsClient, Result};
/// use elevenlabs_rs::utils::play;
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///    let client = ElevenLabsClient::from_env()?;
///    let endpoint = GetConversationAudio::new("conversation_id");
///    let bytes = client.hit(endpoint).await?;
///    play(bytes)?;
///    Ok(())
/// }
/// ```
/// See [Get Conversation Audio API reference](https://elevenlabs.io/docs/conversational-ai/api-reference/conversations/get-conversation-audio)
#[derive(Clone, Debug, Serialize)]
pub struct GetConversationAudio {
    conversation_id: String,
}

impl GetConversationAudio {
    pub fn new(conversation_id: impl Into<String>) -> Self {
        Self {
            conversation_id: conversation_id.into(),
        }
    }
}

impl ElevenLabsEndpoint for GetConversationAudio {
    const PATH: &'static str = "/v1/convai/conversations/:conversation_id/audio";

    const METHOD: Method = Method::GET;

    type ResponseBody = Bytes;

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![self.conversation_id.and_param(PathParam::ConversationID)]
    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.bytes().await?)
    }
}

/// Get a signed url to start a conversation with an agent with an agent that requires authorization
///
/// # Example
/// ```no_run
/// use elevenlabs_rs::endpoints::convai::conversations::GetSignedUrl;
/// use elevenlabs_rs::{ElevenLabsClient, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///    let client = ElevenLabsClient::from_env()?;
///    let endpoint = GetSignedUrl::new("agent_id");
///    let resp = client.hit(endpoint).await?;
///    println!("{}", resp.signed_url);
///   Ok(())
/// }
/// ```
/// See [Get Signed URL API reference](https://elevenlabs.io/docs/conversational-ai/api-reference/conversations/get-signed-url)
#[derive(Clone, Debug, Serialize)]
pub struct GetSignedUrl {
    query: GetSignedUrlQuery,
}

impl GetSignedUrl {
    pub fn new(agent_id: impl Into<String>) -> Self {
        GetSignedUrl {
            query: GetSignedUrlQuery {
                params: vec![("agent_id", agent_id.into())],
            },
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct GetSignedUrlQuery {
    params: QueryValues,
}

impl ElevenLabsEndpoint for GetSignedUrl {
    const PATH: &'static str = "/v1/convai/conversation/get_signed_url";

    const METHOD: Method = Method::GET;

    type ResponseBody = SignedUrlResponse;

    fn query_params(&self) -> Option<QueryValues> {
        Some(self.query.params.clone())

    }

    async fn response_body(self, resp: Response) -> Result<Self::ResponseBody> {
        Ok(resp.json().await?)
    }
}

#[derive(Clone, Debug, Deserialize)]
pub struct SignedUrlResponse {
    pub signed_url: String,
}

/// Send the feedback for the given conversation
///
/// # Example
/// ```no_run
/// use elevenlabs_rs::endpoints::convai::conversations::{
///  SendConversationFeedback, SendConversationFeedbackBody, Score};
/// use elevenlabs_rs::{ElevenLabsClient, Result};
///
/// #[tokio::main]
/// async fn main() -> Result<()> {
///    let client = ElevenLabsClient::from_env()?;
///    let body = SendConversationFeedbackBody::new(Score::Like);
///    let endpoint = SendConversationFeedback::new("conversation_id", body);
///    let resp = client.hit(endpoint).await?;
///    println!("{:?}", resp);
///    Ok(())
/// }
/// ```
/// See [Send Conversation Feedback API reference](https://elevenlabs.io/docs/conversational-ai/api-reference/conversations/post-conversation-feedback)
#[derive(Clone, Debug)]
pub struct SendConversationFeedback {
    conversation_id: String,
    body: SendConversationFeedbackBody,
}

impl SendConversationFeedback {
    pub fn new(conversation_id: impl Into<String>, body: SendConversationFeedbackBody) -> Self {
        Self {
            conversation_id: conversation_id.into(),
            body,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct SendConversationFeedbackBody {
    pub feedback: Score,
}

impl SendConversationFeedbackBody {
    pub fn new(feedback: Score) -> Self {
        Self { feedback }
    }
}

impl ElevenLabsEndpoint for SendConversationFeedback {
    const PATH: &'static str = "/v1/convai/conversations/:conversation_id/feedback";

    const METHOD: Method = Method::POST;

    type ResponseBody = ();

    fn path_params(&self) -> Vec<(&'static str, &str)> {
        vec![self.conversation_id.and_param(PathParam::ConversationID)]
    }

    async fn request_body(&self) -> Result<RequestBody> {
        TryInto::try_into(&self.body)
    }

    async fn response_body(self, _resp: Response) -> Result<Self::ResponseBody> {
        Ok(())
    }
}

impl TryInto<RequestBody> for &SendConversationFeedbackBody {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_into(self) -> Result<RequestBody> {
        Ok(RequestBody::Json(serde_json::to_value(self)?))
    }
}

impl IntoIterator for GetConversationsResponse {
    type Item = Conversation;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.conversations.into_iter()
    }
}

impl<'a> IntoIterator for &'a GetConversationsResponse {
    type Item = &'a Conversation;
    type IntoIter = std::slice::Iter<'a, Conversation>;

    fn into_iter(self) -> Self::IntoIter {
        self.conversations.iter()
    }
}

impl IntoIterator for GetConversationDetailsResponse {
    type Item = Transcript;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.transcript.into_iter()
    }
}

impl<'a> IntoIterator for &'a GetConversationDetailsResponse {
    type Item = &'a Transcript;
    type IntoIter = std::slice::Iter<'a, Transcript>;

    fn into_iter(self) -> Self::IntoIter {
        self.transcript.iter()
    }
}
