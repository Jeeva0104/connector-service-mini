use std::fmt::Debug;

// use super::macros;
use common_utils::errors::CustomResult;
use connector_integration::types::ConnectorData;
use domain_types::{
    connector_flow::Authorize,
    connector_types::{
        ConnectorEnum, PaymentFlowData, PaymentsAuthorizeData, PaymentsResponseData,
    },
    errors::ConnectorError,
    payment_method::{DefaultPCIHolder, PaymentMethodDataTypes},
    router_data::ErrorResponse,
    router_data_v2::{ConnectorAuthType, RouterDataV2},
    types::{ConnectorParams, Connectors},
    utils::ForeignTryFrom,
};
use grpc::errors::PaymentAuthorizationError;
use grpc::payments::PaymentStatus;
use grpc::types::PaymentServiceAuthorizeRequest;

use interfaces::connector_integration_v2::BoxedConnectorIntegrationV2;

#[derive(Debug)]
pub struct StringOutput {
    name: String,
}
#[derive(Debug)]
pub struct IntOutput {
    amount: i64,
}

pub struct Payment;

pub trait PaymentOperationInternal {
    async fn internal_void(&self) -> IntOutput;
    async fn internal_authorize(&self) -> StringOutput;
}

pub trait PaymentService {
    async fn authorize(
        &self,
        payload: PaymentServiceAuthorizeRequest,
    ) -> Result<(), PaymentAuthorizationError>;
}
impl Payment {
    #[allow(clippy::too_many_arguments)]
    async fn process_authorization_internal<
        T: PaymentMethodDataTypes
            + Default
            + Eq
            + Send
            + serde::Serialize
            + serde::de::DeserializeOwned
            + Clone
            + Sync
            + Debug
            + domain_types::types::CardConversionHelper<T>
            + 'static,
    >(
        &self,
        payload: PaymentServiceAuthorizeRequest, // grpc Request
        connector: ConnectorEnum,
    ) -> Result<(), PaymentAuthorizationError> {
        let connector_data: ConnectorData<T> = ConnectorData::get_connector_by_name(&connector);
        // let url: CustomResult<String, ConnectorError> = connector_data.connector.get_url();
        let connector_integration: BoxedConnectorIntegrationV2<
            '_,
            Authorize,
            PaymentFlowData,
            PaymentsAuthorizeData<T>,
            PaymentsResponseData,
        > = connector_data.connector.get_connector_integration_v2();
        // let _ = connector_integration.get_url();
        let base_url = "https://apitest.cybersource.com/pts/v2/payments/".to_string();
        let connector_params = ConnectorParams::new(base_url);

        let connector = Connectors {
            adyen: connector_params,
        };
        let metadata = "metadata".to_string();

        let payment_flow_data =
            PaymentFlowData::foreign_try_from((payload.clone(), connector, &metadata)).map_err(
                |err| {
                    PaymentAuthorizationError::new(
                        PaymentStatus::Pending.to_string(),
                        Some("Failed to process payment flow data".to_string()),
                        Some("PAYMENT_FLOW_ERROR".to_string()),
                        None,
                    )
                },
            )?;

        let payment_authorize_data = PaymentsAuthorizeData::<T>::foreign_try_from(payload.clone())
            .map_err(|err| {
                PaymentAuthorizationError::new(
                    PaymentStatus::Pending.to_string(),
                    Some("Failed to process payment authorize data".to_string()),
                    Some("PAYMENT_AUTHORIZE_DATA_ERROR".to_string()),
                    None,
                )
            })?;

        let auth = ConnectorAuthType::HeaderKey {
            api_key: "api_key".to_string(),
        };

        let router_data = RouterDataV2::<
            Authorize,
            PaymentFlowData,
            PaymentsAuthorizeData<T>,
            PaymentsResponseData,
        > {
            flow: std::marker::PhantomData,
            resource_common_data: payment_flow_data.clone(),
            connector_auth_type: auth.clone(),
            request: payment_authorize_data,
            response: Err(ErrorResponse::default()),
        };
        let response = external_services::service::execute_connector_processing_step(
            // &config.proxy,
            connector_integration,
            router_data,
            None,
            // event_params,
            // token_data,
            common_enums::CallConnectorAction::Trigger,
            // test_context,
            // api_tag,
        )
        .await;
        Ok(())
        // let _ = connector_data.connector.test();
    }
}

impl PaymentService for Payment {
    async fn authorize(
        &self,
        payload: PaymentServiceAuthorizeRequest,
    ) -> Result<(), PaymentAuthorizationError> {
        // println!("Payload {:?}", payload);
        let connector = ConnectorEnum::Adyen;
        let _ = self
            .process_authorization_internal::<DefaultPCIHolder>(payload, connector)
            .await;
        Ok(())
    }
}

/*
1. process_authorization_internal function can be called only by the T who is implementing the PaymentMethodDataTypes and CardConversionHelper
2. DefaultPCIHolder is implementing both PaymentMethodDataTypes and CardConversionHelper
3. So T is nothing but the DefaultPCIHolder
*/
