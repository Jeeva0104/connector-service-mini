use crate::errors::{ApiError, ApplicationErrorResponse};
// use crate::types::PaymentServiceAuthorizeRequest;
use crate::connector_types::{MandateIds, PaymentFlowData, PaymentsAuthorizeData};
use crate::payment_method::{
    Card, DefaultCardData, DefaultPCIHolder, PaymentMethodData, PaymentMethodDataTypes,
    RawCardNumber,
};
use crate::utils::ForeignTryFrom;
use error_stack::ResultExt;
use grpc::types::{CardDetails, PaymentMethod, PaymentServiceAuthorizeRequest};
use serde::{Deserialize, Serialize};
#[derive(Clone, Deserialize, Serialize, Debug, Default)]
pub struct ConnectorParams {
    /// base url
    #[serde(default)]
    pub base_url: String,
    #[serde(default)]
    pub dispute_base_url: Option<String>,
    #[serde(default)]
    pub secondary_base_url: Option<String>,
    #[serde(default)]
    pub third_base_url: Option<String>,
}

impl ConnectorParams {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            dispute_base_url: None,
            secondary_base_url: None,
            third_base_url: None,
        }
    }
}

#[derive(Clone, serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct Connectors {
    // Added pub
    pub adyen: ConnectorParams,
}

impl ForeignTryFrom<(PaymentServiceAuthorizeRequest, Connectors, &String)> for PaymentFlowData {
    type Error = ApplicationErrorResponse;
    fn foreign_try_from(
        (value, connectors, metadata): (PaymentServiceAuthorizeRequest, Connectors, &String),
    ) -> Result<Self, error_stack::Report<Self::Error>> {
        Ok(PaymentFlowData {
            payment_id: "1244".to_string(),
            attempt_id: "1244".to_string(),
            amount_captured: None,
            connectors,
        })
    }
}

impl<
        T: PaymentMethodDataTypes
            + Default
            + Send
            + Eq
            + PartialEq
            + serde::Serialize
            + serde::de::DeserializeOwned
            + Clone
            + CardConversionHelper<T>,
    > ForeignTryFrom<PaymentServiceAuthorizeRequest> for PaymentsAuthorizeData<T>
{
    type Error = ApplicationErrorResponse;

    fn foreign_try_from(
        value: PaymentServiceAuthorizeRequest,
    ) -> Result<Self, error_stack::Report<Self::Error>> {
        Ok(Self {
            // payment_method_data: todo!(),
            payment_method_data: PaymentMethodData::<T>::foreign_try_from(
                value.payment_method.clone(),
            )
            .change_context(ApplicationErrorResponse::BadRequest(ApiError {
                sub_code: "INVALID_PAYMENT_METHOD_DATA".to_owned(),
                error_identifier: 400,
                error_message: "Payment method data construction failed".to_owned(),
                error_object: None,
            }))?,
            confirm: true,
            mandate_id: MandateIds {
                mandate_id: None,
                mandate_reference_id: None,
            },
        })
    }
}
pub trait CardConversionHelper<T: PaymentMethodDataTypes> {
    fn convert_card_details(
        card: CardDetails,
    ) -> Result<Card<T>, error_stack::Report<ApplicationErrorResponse>>;
}

impl CardConversionHelper<DefaultPCIHolder> for DefaultPCIHolder {
    fn convert_card_details(
        card: CardDetails,
    ) -> Result<Card<DefaultPCIHolder>, error_stack::Report<ApplicationErrorResponse>> {
        Ok(Card {
            card_number: RawCardNumber::<DefaultPCIHolder>(DefaultCardData {
                card_number: card.card_number,
            }),
            // card_exp_month: card
            //     .card_exp_month
            //     .ok_or(ApplicationErrorResponse::BadRequest(ApiError {
            //         sub_code: "MISSING_EXP_MONTH".to_owned(),
            //         error_identifier: 400,
            //         error_message: "Missing Card Expiry Month".to_owned(),
            //         error_object: None,
            //     }))?,
            // card_exp_year: card
            //     .card_exp_year
            //     .ok_or(ApplicationErrorResponse::BadRequest(ApiError {
            //         sub_code: "MISSING_EXP_YEAR".to_owned(),
            //         error_identifier: 400,
            //         error_message: "Missing Card Expiry Year".to_owned(),
            //         error_object: None,
            //     }))?,
            // card_cvc: card
            //     .card_cvc
            //     .ok_or(ApplicationErrorResponse::BadRequest(ApiError {
            //         sub_code: "MISSING_CVC".to_owned(),
            //         error_identifier: 400,
            //         error_message: "Missing CVC".to_owned(),
            //         error_object: None,
            //     }))?,
            card_issuer: card.card_issuer,
            // card_network,
            // card_type: card.card_type,
            // card_issuing_country: card.card_issuing_country_alpha2,
            // bank_code: card.bank_code,
            // nick_name: card.nick_name.map(|name| name.into()),
            // card_holder_name: card.card_holder_name,
            // co_badged_card_data: None,
        })
    }
}

impl<T> ForeignTryFrom<CardDetails> for Card<T>
where
    T: PaymentMethodDataTypes
        + Default
        // + Debug
        + Send
        + Eq
        + PartialEq
        + serde::Serialize
        + serde::de::DeserializeOwned
        + Clone
        + CardConversionHelper<T>,
{
    type Error = ApplicationErrorResponse;
    fn foreign_try_from(card: CardDetails) -> Result<Self, error_stack::Report<Self::Error>> {
        T::convert_card_details(card)
    }
}

impl<
        T: PaymentMethodDataTypes
            + Default
            // + Debug
            + Send
            + Eq
            + PartialEq
            + serde::Serialize
            + serde::de::DeserializeOwned
            + Clone
            + CardConversionHelper<T>,
    > ForeignTryFrom<PaymentMethod> for PaymentMethodData<T>
{
    type Error = ApplicationErrorResponse;

    fn foreign_try_from(value: PaymentMethod) -> Result<Self, error_stack::Report<Self::Error>> {
        // tracing::info!("PaymentMethod data received: {:?}", value);

        match value {
            // Some(data) => match data {
            // ============================================================================
            // CARD METHODS
            // ============================================================================
            PaymentMethod::Card(card_details) => {
                let card = Card::<T>::foreign_try_from(card_details)?;
                Ok(PaymentMethodData::Card(card))
            } // },
        }
    }
}
