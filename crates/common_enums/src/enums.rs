#[derive(
    Clone,
    Debug,
    Eq,
    Hash,
    PartialEq,
    serde::Deserialize,
    serde::Serialize,
    strum::Display,
    strum::VariantNames,
    strum::EnumIter,
    strum::EnumString,
)]
pub enum CardNetwork {
    #[serde(alias = "VISA")]
    Visa,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CallConnectorAction {
    Trigger,
    // HandleResponse(Vec<u8>),
}

#[derive(Debug, thiserror::Error, PartialEq, Clone)]
pub enum ApiClientError {
    #[error("Header map construction failed")]
    HeaderMapConstructionFailed,
    #[error("Invalid proxy configuration")]
    InvalidProxyConfiguration,
    #[error("Client construction failed")]
    ClientConstructionFailed,
    #[error("Certificate decode failed")]
    CertificateDecodeFailed,
    #[error("Request body serialization failed")]
    BodySerializationFailed,
    #[error("Unexpected state reached/Invariants conflicted")]
    UnexpectedState,

    #[error("Failed to parse URL")]
    UrlParsingFailed,
    #[error("URL encoding of request payload failed")]
    UrlEncodingFailed,
    #[error("Failed to send request to connector {0}")]
    RequestNotSent(String),
    #[error("Failed to decode response")]
    ResponseDecodingFailed,

    #[error("Server responded with Request Timeout")]
    RequestTimeoutReceived,

    #[error("connection closed before a message could complete")]
    ConnectionClosedIncompleteMessage,

    #[error("Server responded with Internal Server Error")]
    InternalServerErrorReceived,
    #[error("Server responded with Bad Gateway")]
    BadGatewayReceived,
    #[error("Server responded with Service Unavailable")]
    ServiceUnavailableReceived,
    #[error("Server responded with Gateway Timeout")]
    GatewayTimeoutReceived,
    #[error("Server responded with unexpected response")]
    UnexpectedServerResponse,
}
