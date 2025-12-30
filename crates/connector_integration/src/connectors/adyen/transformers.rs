use super::AdyenRouterData;
use domain_types::connector_flow::Authorize;
use domain_types::connector_types::{PaymentFlowData, PaymentsAuthorizeData, PaymentsResponseData};
use domain_types::errors;
use domain_types::payment_method::{
    Card, PaymentMethodData, PaymentMethodDataTypes, RawCardNumber,
};
use domain_types::router_data_v2::RouterDataV2;
use hyperswitch_masking::Secret;
use serde::{Deserialize, Serialize};
type Error = error_stack::Report<domain_types::errors::ConnectorError>;
#[derive(Debug, Clone, Serialize)]
// #[serde(rename_all = "camelCase")]
pub struct AdyenCard<
    T: PaymentMethodDataTypes
        + std::fmt::Debug
        + std::marker::Sync
        + std::marker::Send
        + Serialize
        + 'static,
> {
    number: RawCardNumber<T>,
    card_holder_name: Option<Secret<String>>,
}

#[derive(Debug, Clone, Serialize)]
pub enum AdyenPaymentMethod<
    T: PaymentMethodDataTypes
        + std::fmt::Debug
        + std::marker::Sync
        + std::marker::Send
        + Serialize
        + 'static,
> {
    AdyenCard(Box<AdyenCard<T>>),
}

#[derive(Debug, Clone, Serialize)]
pub enum PaymentMethod<
    T: PaymentMethodDataTypes
        + std::fmt::Debug
        + std::marker::Sync
        + std::marker::Send
        + Serialize
        + 'static,
> {
    AdyenPaymentMethod(Box<AdyenPaymentMethod<T>>),
}

#[derive(Debug, Serialize)]
pub struct AdyenPaymentRequest<
    T: PaymentMethodDataTypes
        + std::fmt::Debug
        + std::marker::Sync
        + std::marker::Send
        + Serialize
        + 'static,
> {
    amount: i64,
    merchant_account: String,
    payment_method: PaymentMethod<T>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardBrand {
    Visa,
    MC,
    Amex,
    Jcb,
    Diners,
    Discover,
    Cartebancaire,
    Cup,
    Maestro,
    Rupay,
    Star,
    Accel,
    Pulse,
    Nyce,
}

impl<
        T: PaymentMethodDataTypes
            + std::fmt::Debug
            + std::marker::Sync
            + std::marker::Send
            + 'static
            + Serialize,
    > TryFrom<(&Card<T>, Option<Secret<String>>)> for AdyenPaymentMethod<T>
{
    type Error = Error;
    fn try_from(
        (card, card_holder_name): (&Card<T>, Option<Secret<String>>),
    ) -> Result<Self, Self::Error> {
        let adyen_card = AdyenCard {
            number: card.card_number.clone(),
            card_holder_name,
        };
        Ok(AdyenPaymentMethod::AdyenCard(Box::new(adyen_card)))
    }
}

impl<
        T: PaymentMethodDataTypes
            + std::fmt::Debug
            + std::marker::Sync
            + std::marker::Send
            + 'static
            + Serialize,
    >
    TryFrom<(
        AdyenRouterData<
            RouterDataV2<
                Authorize,
                PaymentFlowData,
                PaymentsAuthorizeData<T>,
                PaymentsResponseData,
            >,
            T,
        >,
        &Card<T>,
    )> for AdyenPaymentRequest<T>
{
    type Error = Error;
    fn try_from(
        value: (
            AdyenRouterData<
                RouterDataV2<
                    Authorize,
                    PaymentFlowData,
                    PaymentsAuthorizeData<T>,
                    PaymentsResponseData,
                >,
                T,
            >,
            &Card<T>,
        ),
    ) -> Result<Self, Self::Error> {
        let (item, card_data) = value;
        let payment_method = PaymentMethod::AdyenPaymentMethod(Box::new(
            AdyenPaymentMethod::try_from((card_data, None))?,
        ));
        Ok(AdyenPaymentRequest {
            amount: 10,
            merchant_account: "Testing".to_string(),
            payment_method,
        })
    }
}

impl<
        T: PaymentMethodDataTypes
            + std::fmt::Debug
            + std::marker::Sync
            + std::marker::Send
            + 'static
            + Serialize,
    >
    TryFrom<
        AdyenRouterData<
            RouterDataV2<
                Authorize,
                PaymentFlowData,
                PaymentsAuthorizeData<T>,
                PaymentsResponseData,
            >,
            T,
        >,
    > for AdyenPaymentRequest<T>
{
    type Error = Error;
    fn try_from(
        item: AdyenRouterData<
            RouterDataV2<
                Authorize,
                PaymentFlowData,
                PaymentsAuthorizeData<T>,
                PaymentsResponseData,
            >,
            T,
        >,
    ) -> Result<Self, Self::Error> {
        // match item
        //     .router_data
        //     .request
        //     .mandate_id
        //     .to_owned()
        //     .and_then(|mandate_ids| mandate_ids.mandate_reference_id)
        // {
        //     Some(_mandate_ref) => Err(domain_types::errors::ConnectorError::NotImplemented(
        //         "payment_method".into(),
        //     ))?,
        //     None => match item.router_data.request.payment_method_data.clone() {
        //         PaymentMethodData::Card(ref card) => AdyenPaymentRequest::try_from((item, card))
        //     }
        // }
        match item.router_data.request.payment_method_data.clone() {
            PaymentMethodData::Card(ref card) => AdyenPaymentRequest::try_from((item, card)),
        }
    }
}
