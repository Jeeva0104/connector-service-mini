use common_utils::errors::CustomResult;
use common_utils::request::{Method, Request, RequestBuilder, RequestContent};
use domain_types::errors;
use domain_types::router_data_v2::RouterDataV2;
use hyperswitch_masking::Maskable;
pub type BoxedConnectorIntegrationV2<'a, Flow, ResourceCommonData, Req, Resp> =
    Box<&'a (dyn ConnectorIntegrationV2<Flow, ResourceCommonData, Req, Resp> + Send + Sync)>;
pub trait ConnectorIntegrationAnyV2<Flow, ResourceCommonData, Req, Resp>:
    Send + Sync + 'static
{
    /// function what returns BoxedConnectorIntegrationV2
    fn get_connector_integration_v2(
        &self,
    ) -> BoxedConnectorIntegrationV2<'_, Flow, ResourceCommonData, Req, Resp>;
}

pub trait ConnectorIntegrationV2<Flow, ResourceCommonData, Req, Resp>:
    ConnectorIntegrationAnyV2<Flow, ResourceCommonData, Req, Resp>
{
    fn get_headers(
        &self,
        _req: &RouterDataV2<Flow, ResourceCommonData, Req, Resp>,
    ) -> CustomResult<Vec<(String, Maskable<String>)>, domain_types::errors::ConnectorError> {
        Ok(vec![])
    }
    /// primarily used when creating signature based on request method of payment flow
    fn get_http_method(&self) -> Method {
        Method::Post
    }

    fn get_url(
        &self,
        _req: &RouterDataV2<Flow, ResourceCommonData, Req, Resp>,
    ) -> CustomResult<String, errors::ConnectorError> {
        // metrics::UNIMPLEMENTED_FLOW
        //     .add(1, router_env::metric_attributes!(("connector", self.id()))); // TODO: discuss env
        Ok(String::new())
    }

    // returns request body
    fn get_request_body(
        &self,
        _req: &RouterDataV2<Flow, ResourceCommonData, Req, Resp>,
    ) -> CustomResult<Option<RequestContent>, errors::ConnectorError> {
        Ok(None)
    }

    /// builds the request and returns it
    fn build_request_v2(
        &self,
        req: &RouterDataV2<Flow, ResourceCommonData, Req, Resp>,
    ) -> CustomResult<Option<Request>, domain_types::errors::ConnectorError> {
        println!("{:?}", self.get_request_body(req));
        Ok(Some(
            RequestBuilder::new()
                .method(self.get_http_method())
                .url(self.get_url(req)?.as_str())
                .attach_default_headers()
                .headers(self.get_headers(req)?)
                .set_optional_body(self.get_request_body(req)?)
                // .add_certificate(self.get_certificate(req)?)
                // .add_certificate_key(self.get_certificate_key(req)?)
                .build(),
        ))
    }
}

impl<S, Flow, ResourceCommonData, Req, Resp>
    ConnectorIntegrationAnyV2<Flow, ResourceCommonData, Req, Resp> for S
where
    S: ConnectorIntegrationV2<Flow, ResourceCommonData, Req, Resp> + Send + Sync,
{
    fn get_connector_integration_v2(
        &self,
    ) -> BoxedConnectorIntegrationV2<'_, Flow, ResourceCommonData, Req, Resp> {
        Box::new(self)
    }
}
