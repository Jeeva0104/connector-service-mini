use domain_types::payment_method::PaymentMethodDataTypes;

use crate::connector_integration_v2::ConnectorIntegrationV2;
use domain_types::connector_flow;
use domain_types::connector_types::{PaymentFlowData, PaymentsAuthorizeData, PaymentsResponseData};

pub trait PaymentAuthorizeV2<T: PaymentMethodDataTypes>:
    ConnectorIntegrationV2<
    connector_flow::Authorize,
    PaymentFlowData,
    PaymentsAuthorizeData<T>,
    PaymentsResponseData,
>
{
}
pub trait ConnectorServiceTrait<T: PaymentMethodDataTypes>: PaymentAuthorizeV2<T> {}

pub type BoxedConnector<T> = Box<&'static (dyn ConnectorServiceTrait<T> + Sync)>;
