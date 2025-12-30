use crate::connectors::Adyen;
use domain_types::connector_types::ConnectorEnum;
use domain_types::payment_method::PaymentMethodDataTypes;
use interfaces::connector_types::BoxedConnector;
use std::fmt::Debug;

pub struct ConnectorData<T: PaymentMethodDataTypes + Debug + Default + Send + Sync + 'static> {
    pub connector: BoxedConnector<T>,
    pub connector_name: ConnectorEnum,
}

impl<T: PaymentMethodDataTypes + Debug + Default + Send + Sync + 'static + serde::Serialize>
    ConnectorData<T>
{
    pub fn get_connector_by_name(connector_name: &ConnectorEnum) -> Self {
        let connector = Self::convert_connector(*connector_name);
        Self {
            connector,
            connector_name: *connector_name,
        }
    }

    fn convert_connector(connector_name: ConnectorEnum) -> BoxedConnector<T> {
        match connector_name {
            ConnectorEnum::Adyen => Box::new(Adyen::new()),
        }
    }
}
