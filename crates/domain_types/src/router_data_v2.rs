use crate::router_data::ErrorResponse;
use std::fmt::Debug;
use std::marker::PhantomData;
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(tag = "auth_type")]
pub enum ConnectorAuthType {
    TemporaryAuth,
    HeaderKey { api_key: String },
    BodyKey { api_key: String, key1: String },
}

// #[derive(Clone, Debug, serde::Serialize)]
// pub struct ErrorResponse {
//     pub code: String,
//     pub message: String,
// }
// use crate::router_data::{ ErrorResponse};

#[derive(Debug, Clone)]
pub struct RouterDataV2<Flow, ResourceCommonData, FlowSpecificRequest, FlowSpecificResponse> {
    pub flow: PhantomData<Flow>,
    // pub tenant_id: id_type::TenantId, // TODO: Should we add this
    pub resource_common_data: ResourceCommonData,
    pub connector_auth_type: ConnectorAuthType,
    /// Contains flow-specific data required to construct a request and send it to the connector.
    pub request: FlowSpecificRequest,
    /// Contains flow-specific data that the connector responds with.
    pub response: Result<FlowSpecificResponse, ErrorResponse>,
}
