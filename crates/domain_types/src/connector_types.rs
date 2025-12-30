use crate::payment_method::{PaymentMethodData, PaymentMethodDataTypes};
use crate::types::Connectors;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
#[derive(Debug, Clone)]
pub struct PaymentFlowData {
    pub payment_id: String,
    pub attempt_id: String,
    pub amount_captured: Option<i64>,
    pub connectors: Connectors,
}

#[derive(Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
pub enum MandateReferenceId {
    NetworkMandateId(String), // network_txns_id send by Issuer to connector, Used for PG agnostic mandate txns along with card data
}
#[derive(Default, Eq, PartialEq, Debug, serde::Deserialize, serde::Serialize, Clone)]
pub struct MandateIds {
    pub mandate_id: Option<String>,
    pub mandate_reference_id: Option<MandateReferenceId>,
}

#[derive(Debug, Clone)]
pub struct PaymentsAuthorizeData<T: PaymentMethodDataTypes> {
    pub payment_method_data: PaymentMethodData<T>,
    pub confirm: bool,
    pub mandate_id: MandateIds,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentsResponseData {
    TransactionResponse {
        connector_response_reference_id: Option<String>,
        incremental_authorization_allowed: Option<bool>,
        status_code: u16,
    },
}

#[derive(Clone, Copy, Debug, Display, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum ConnectorEnum {
    Adyen,
}
