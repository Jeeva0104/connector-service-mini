use std::fmt::Debug;
pub mod transformers;
use super::macros;
use common_utils::errors::CustomResult;
use common_utils::request::RequestContent;
use domain_types::{
    connector_flow::Authorize,
    connector_types::{PaymentFlowData, PaymentsAuthorizeData, PaymentsResponseData},
    errors::ConnectorError,
    payment_method::PaymentMethodDataTypes,
    router_data_v2::RouterDataV2,
};
use interfaces::{
    connector_integration_v2::ConnectorIntegrationV2,
    connector_types::{ConnectorServiceTrait, PaymentAuthorizeV2},
};
use serde::Serialize;
use std::marker::PhantomData;
use transformers::AdyenPaymentRequest;
impl<T: PaymentMethodDataTypes + Debug + Sync + Send + Serialize + 'static> ConnectorServiceTrait<T>
    for Adyen<T>
{
}

impl<T: PaymentMethodDataTypes + Debug + Sync + Send + Serialize + 'static> PaymentAuthorizeV2<T>
    for Adyen<T>
{
}

// macros::create_all_prerequisites!(
//     connector_name:Adyen,
//     generic_type: T,
//     api: [
//         (
//             flow: Authorize,
//             request_body: AdyenPaymentRequest<T>,
//             response_body: String,
//             router_data: RouterDataV2<Authorize, PaymentFlowData, PaymentsAuthorizeData<T>, PaymentsResponseData>,
//         )
//       ]
// );
#[derive(Clone)]
pub struct Adyen<
    T: PaymentMethodDataTypes
        + std::fmt::Debug
        + std::marker::Sync
        + std::marker::Send
        + serde::Serialize
        + 'static,
> {
    authorize: &'static (dyn macros::BridgeRequestResponse<
        RequestBody = AdyenPaymentRequest<T>,
        ResponseBody = String,
        ConnectorInputData = AdyenRouterData<
            RouterDataV2<
                Authorize,
                PaymentFlowData,
                PaymentsAuthorizeData<T>,
                PaymentsResponseData,
            >,
            T,
        >,
    >),
}
pub struct AdyenRouterData<
    RD: macros::FlowTypes,
    T: PaymentMethodDataTypes
        + std::fmt::Debug
        + std::marker::Sync
        + std::marker::Send
        + Serialize
        + 'static,
> {
    pub connector: Adyen<T>,
    pub router_data: RD,
}

pub struct AdyenPaymentRequestTemplating;

pub struct StringTemplating;

impl<
        RD: macros::FlowTypes,
        T: PaymentMethodDataTypes
            + std::fmt::Debug
            + std::marker::Sync
            + std::marker::Send
            + 'static
            + serde::Serialize,
    > macros::FlowTypes for AdyenRouterData<RD, T>
{
    type Flow = RD::Flow;
    type FlowCommonData = RD::FlowCommonData;
    type Request = RD::Request;
    type Response = RD::Response;
}

impl<
        T: PaymentMethodDataTypes
            + std::fmt::Debug
            + std::marker::Sync
            + std::marker::Send
            + 'static
            + serde::Serialize,
    > macros::BridgeRequestResponse
    for macros::Bridge<AdyenPaymentRequestTemplating, StringTemplating, T>
{
    type RequestBody = AdyenPaymentRequest<T>;
    type ResponseBody = String;
    type ConnectorInputData = AdyenRouterData<
        RouterDataV2<Authorize, PaymentFlowData, PaymentsAuthorizeData<T>, PaymentsResponseData>,
        T,
    >;
}

impl<
        T: PaymentMethodDataTypes
            + std::fmt::Debug
            + std::marker::Sync
            + std::marker::Send
            + 'static
            + serde::Serialize,
    > Adyen<T>
{
    pub const fn new() -> &'static Self {
        &Self {
            authorize: &macros::Bridge::<AdyenPaymentRequestTemplating, StringTemplating, T>(
                PhantomData,
            ),
        }
    }
}

impl<T: PaymentMethodDataTypes + Debug + Sync + Send + Serialize + 'static>
    ConnectorIntegrationV2<
        Authorize,
        PaymentFlowData,
        PaymentsAuthorizeData<T>,
        PaymentsResponseData,
    > for Adyen<T>
{
    fn get_url(
        &self,
        req: &RouterDataV2<
            Authorize,
            PaymentFlowData,
            PaymentsAuthorizeData<T>,
            PaymentsResponseData,
        >,
    ) -> CustomResult<String, ConnectorError> {
        let url = req.resource_common_data.connectors.adyen.base_url.clone();
        Ok(url)
    }

    fn get_request_body(
        &self,
        req: &RouterDataV2<
            Authorize,
            PaymentFlowData,
            PaymentsAuthorizeData<T>,
            PaymentsResponseData,
        >,
    ) -> CustomResult<Option<RequestContent>, ConnectorError> {
        let bridge = self.authorize;
        let input_data = AdyenRouterData {
            connector: self.to_owned(),
            router_data: req.clone(),
        };
        let request = bridge.request_body(input_data)?;
        Ok(Some(RequestContent::Json(Box::new(request))))
    }
}
