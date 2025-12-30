// use common_utils::errors::CustomResult;
use common_utils::errors::CustomResult;
use domain_types::{errors, router_data_v2::RouterDataV2};
use std::marker::PhantomData;
pub trait FlowTypes {
    type Flow;
    type FlowCommonData;
    type Request;
    type Response;
}
impl<F, FCD, Req, Resp> FlowTypes for RouterDataV2<F, FCD, Req, Resp> {
    type Flow = F;
    type FlowCommonData = FCD;
    type Request = Req;
    type Response = Resp;
}

impl<F, FCD, Req, Resp> FlowTypes for &RouterDataV2<F, FCD, Req, Resp> {
    type Flow = F;
    type FlowCommonData = FCD;
    type Request = Req;
    type Response = Resp;
}
#[derive(Clone)]

pub struct Bridge<Q, T, S>(pub PhantomData<(Q, T, S)>);
pub trait BridgeRequestResponse: Send + Sync {
    type RequestBody;
    type ResponseBody;
    type ConnectorInputData: FlowTypes;
    fn request_body(
        &self,
        rd: Self::ConnectorInputData,
    ) -> CustomResult<Self::RequestBody, errors::ConnectorError>
    where
        Self::RequestBody:
            TryFrom<Self::ConnectorInputData, Error = error_stack::Report<errors::ConnectorError>>,
    {
        Self::RequestBody::try_from(rd)
    }
    // fn response(
    //     &self,
    //     bytes: bytes::Bytes,
    // ) -> CustomResult<Self::ResponseBody, errors::ConnectorError>
    // where
    //     Self::ResponseBody: for<'a> serde::Deserialize<'a>,
    // {
    //     if bytes.is_empty() {
    //         serde_json::from_str("{}")
    //             .change_context(errors::ConnectorError::ResponseDeserializationFailed)
    //     } else {
    //         bytes
    //             .parse_struct(std::any::type_name::<Self::ResponseBody>())
    //             .change_context(errors::ConnectorError::ResponseDeserializationFailed)
    //     }
    // }

    // fn router_data(
    //     &self,
    //     response: ResponseRouterDataType<Self::ConnectorInputData, Self::ResponseBody>,
    // ) -> CustomResult<RouterDataType<Self::ConnectorInputData>, errors::ConnectorError>
    // where
    //     RouterDataType<Self::ConnectorInputData>: TryFrom<
    //         ResponseRouterDataType<Self::ConnectorInputData, Self::ResponseBody>,
    //         Error = error_stack::Report<errors::ConnectorError>,
    //     >,
    // {
    //     RouterDataType::<Self::ConnectorInputData>::try_from(response)
    // }
}

macro_rules! expand_connector_input_data {
    ($connector: ident, $generics: tt) => {
        paste::paste! {
            pub struct [<$connector RouterData>]<RD: FlowTypes, $generics: PaymentMethodDataTypes + std::fmt::Debug + std::marker::Sync + std::marker::Send + 'static> {
                pub connector: $connector<$generics>,
                pub router_data: RD,
            }
            impl<RD: FlowTypes, $generics: PaymentMethodDataTypes + std::fmt::Debug + std::marker::Sync + std::marker::Send + 'static + serde::Serialize> FlowTypes for [<$connector RouterData>]<RD, $generics> { //here too
                type Flow = RD::Flow;
                type FlowCommonData = RD::FlowCommonData;
                type Request = RD::Request;
                type Response = RD::Response;
            }
        }
    };
}
pub(crate) use expand_connector_input_data;

macro_rules! expand_imports {
    () => {
        #[allow(unused_imports)]
        use crate::connectors::macros::{Bridge, BridgeRequestResponse, FlowTypes};

        #[allow(unused_imports)]
        mod macro_types {
            // pub(super) use domain_models::{
            //     AuthenticationInitiation, Confirmation, PostAuthenticationSync, PreAuthentication,
            // };
            pub(super) use common_utils::{errors::CustomResult, events, request::RequestContent};
            pub(super) use domain_types::{
                errors::ConnectorError, router_data::ErrorResponse, router_data_v2::RouterDataV2,
                router_response_types::Response,
            };
            pub(super) use hyperswitch_masking::Maskable;

            pub(super) use crate::types::*;
        }
    };
}
pub(crate) use expand_imports;

macro_rules! resolve_request_body_type {
    // Generic type like AdyenPaymentRequest<T>
    ($base_req: ident<$req_generic: ident>, $generic_type: tt) => {
        $base_req<$generic_type>
    };
    // Non-generic type like AdyenRedirectRequest
    ($base_req: ident, $generic_type: tt) => {
        $base_req
    };
}
pub(crate) use resolve_request_body_type;

macro_rules! create_all_prerequisites_resolve_request_body_type {
    // Pattern with request body
    (
        request_body: $flow_request: ident $(<$generic_param: ident>)?,
        generic_type: $generic_type: tt
    ) => {
        crate::connectors::macros::resolve_request_body_type!($flow_request $(<$generic_param>)?, $generic_type)
    };

    // Pattern without request body
    (
        generic_type: $generic_type: tt
    ) => {
        NoRequestBody
    };
}
pub(crate) use create_all_prerequisites_resolve_request_body_type;

macro_rules! resolve_templating_type {
    // Generic type like AdyenPaymentRequest<T>
    ($base_req: ident<$req_generic: ident>) => {
        paste::paste! { [<$base_req Templating>] }
    };
    // Non-generic type like AdyenRedirectRequest
    ($base_req: ident) => {
        paste::paste! { [<$base_req Templating>] }
    };
}
pub(crate) use resolve_templating_type;

macro_rules! create_all_prerequisites_resolve_templating_type {
    // Pattern with request body
    (
        request_body: $flow_request: ident $(<$generic_param: ident>)?,
    ) => {
        crate::connectors::macros::resolve_templating_type!($flow_request $(<$generic_param>)?)
    };

    // Pattern without request body
    () => {
        NoRequestBodyTemplating
    };
}
pub(crate) use create_all_prerequisites_resolve_templating_type;

macro_rules! impl_templating_mixed {
    // Pattern for generic request types like AdyenPaymentRequest<T>
    (
        connector: $connector: ident,
        curl_request: $base_req: ident<$req_generic: ident>,
        curl_response: $curl_res: ident,
        router_data: $router_data: ty,
        generic_type: $generic_type: tt,
    ) => {
        paste::paste!{
            pub struct [<$base_req Templating>];
            pub struct [<$curl_res Templating>];

            impl<$generic_type: PaymentMethodDataTypes + std::fmt::Debug + std::marker::Sync + std::marker::Send + 'static + serde::Serialize> BridgeRequestResponse for Bridge<[<$base_req Templating>], [<$curl_res Templating>], $generic_type>{
                type RequestBody = $base_req<$generic_type>;
                type ResponseBody = $curl_res;
                type ConnectorInputData = [<$connector RouterData>]<$router_data, $generic_type>;
            }
        }
    };
}
pub(crate) use impl_templating_mixed;

macro_rules! create_all_prerequisites_impl_templating {
    // Pattern with request body
    (
        connector: $connector: ident,
        request_body: $flow_request: ident $(<$generic_param: ident>)?,
        response_body: $flow_response: ident,
        router_data: $router_data_type: ty,
        generic_type: $generic_type: tt,
    ) => {
        crate::connectors::macros::impl_templating_mixed!(
            connector: $connector,
            curl_request: $flow_request $(<$generic_param>)?,
            curl_response: $flow_response,
            router_data: $router_data_type,
            generic_type: $generic_type,
        );
    };


}
pub(crate) use create_all_prerequisites_impl_templating;

macro_rules! create_all_prerequisites {
    (   connector_name: $connector: ident,
        generic_type: $generic_type:tt,
        api: [
            $(
                (
                    flow: $flow_name: ident,
                    $(request_body: $flow_request: ident $(<$generic_param: ident>)?,)?
                    response_body: $flow_response: ident,
                    router_data: $router_data_type: ty,
                )
            )*
        ]
    ) => {
        crate::connectors::macros::expand_imports!();
        crate::connectors::macros::expand_connector_input_data!($connector, $generic_type);
         $(
            crate::connectors::macros::create_all_prerequisites_impl_templating!(
                connector: $connector,
                $(request_body: $flow_request $(<$generic_param>)?,)?
                response_body: $flow_response,
                router_data: $router_data_type,
                generic_type: $generic_type,
            );
        )*
         paste::paste! {
            pub struct $connector <$generic_type: PaymentMethodDataTypes + std::fmt::Debug + std::marker::Sync + std::marker::Send + 'static >
            {
                // $(
                //     pub $converter_name: &'static (dyn common_utils::types::AmountConvertor<Output = $amount_unit> + Sync)
                // )*
                $(
                    [<$flow_name:snake>]: &'static (dyn crate::connectors::macros::BridgeRequestResponse<
                        RequestBody = crate::connectors::macros::create_all_prerequisites_resolve_request_body_type!($(request_body: $flow_request $(<$generic_param>)?,)? generic_type: $generic_type),
                        ResponseBody = $flow_response,
                        ConnectorInputData = [<$connector RouterData>]<$router_data_type, $generic_type>,
                    >),
                )*
            }


            impl<$generic_type: PaymentMethodDataTypes + std::fmt::Debug + std::marker::Sync + std::marker::Send + 'static + serde::Serialize>  $connector<$generic_type> {
                pub const fn new() -> &'static Self {
                    &Self{

                        $(
                            [<$flow_name:snake>]: &crate::connectors::macros::Bridge::<
                                    crate::connectors::macros::create_all_prerequisites_resolve_templating_type!($(request_body: $flow_request $(<$generic_param>)?,)?),
                                   [<$flow_response Templating>],
                                    $generic_type
                                >(PhantomData),
                        )*
                    }
                }

            }

        }
    };
}
pub(crate) use create_all_prerequisites;

macro_rules! implement_connector_operation {
    (
    fn_name: $fn_name:ident,
    generate_response_fn: $generate_response_fn:path,
    response_type: $response_type:ty
    ) => {
        async fn $fn_name(&self) -> $response_type {
            $generate_response_fn(self)
        }
    };
}

pub(crate) use implement_connector_operation;

/*

create_all_prerequisites

expansion and understanding

use std::marker::PhantomData;

#### PaymentMethodDataTypes is top level trait

trait PaymentMethodDataTypes {
    fn call_connector(&self);
}

#### Card Implements the PaymentMethodDataTypes

#[derive(Debug)]
struct Card;
impl PaymentMethodDataTypes for Card {
    fn call_connector(&self){
        println!("{}","I am calling connector")
    }
}

trait BridgeRequestResponse {
    type RequestBody;
    type ResponseBody;
}

struct Bridge<Req,Res>(pub PhantomData<(Req, Res)>);

impl <T:PaymentMethodDataTypes> BridgeRequestResponse for Bridge<AdyenPaymentRequest<T>,String> {
    type RequestBody=AdyenPaymentRequest<T>;
    type ResponseBody=String;
}

struct AdyenPaymentRequest<T:PaymentMethodDataTypes>{
    amount: u32,
    method: PhantomData<T>,
}

struct Adyen<T:PaymentMethodDataTypes> {
    authorize:Box<
                dyn BridgeRequestResponse<
                RequestBody=AdyenPaymentRequest<T>,
                ResponseBody=String>
                >
}



fn main(){
    let bridge=Box::new(Bridge::<AdyenPaymentRequest<Card>,String>(PhantomData));

    let t = Adyen::<Card> {
        authorize:bridge
    };

    let _=t.authorize;
}


*/

/*
use std::marker::PhantomData;

struct Bridge<Req,Res>(pub PhantomData<(Req,Res)>);

trait BridgeRequestResponse {
    type RequestBody;
    type ResponseBody;
}

impl BridgeRequestResponse for Bridge<String,String>{
    type RequestBody=String;
    type ResponseBody=String;
}

struct Adyen {
/* Any type that implements BridgeRequestResponse
   where RequestBody = String and ResponseBody = String
   can be used as authorize value
   Bridge<String, String> is one such type
*/
    authorize : Box< 
                    dyn BridgeRequestResponse<
                         RequestBody=String,
                         ResponseBody=String>
                         >
}

fn main() {
    let test:Bridge<String,String>=Bridge(PhantomData);
    let test2=Bridge::<String,String>(PhantomData);
    let t=Adyen {
        authorize:Box::new(test)
    };
}
*/


/*



// Who is doing Job for whom
// Doctor is doing Job for Human
// Mechaninc is doing Job for Car
// While doing the Job we need to work
// Doctor will do check_up
// Maid will do cooking

trait Job<Target>{
    type Target;
    fn work(&self ,val:&Self::Target);
}
#[derive(Debug)]
struct Doctor{
    name:String
}
impl Doctor {
    fn check_up(&self,human:&Human){
        println!("{:?}doctor will do check for this person",human.name);
    }
}
struct Maid;
#[derive(Debug)]
struct Human {
    name:String,
    geneder:String
}

struct Mechanic;
#[derive(Debug)]
struct Car {
    brand:String,
    color:String
}

impl Job<Human> for Doctor {
    type Target =Human;
    fn work(&self ,human:&Human){
        println!("{:?}",human);
        self.check_up(&human)
    }
    
}

impl Job<Human> for Maid {
    type Target=Human;
    fn work(&self,human:&Human){
        println!("{}","Maid is working for Human")
    }
} 

impl Job<Car> for Mechanic {
    type Target=Car;
    fn work(&self, car:&Car){
        println!("{:?}",car)
    }
}

struct HumanWorker {
// any type that implements Job whose target is equal to Human can be stored for worker key
// currenlty doctor is implementing the Job with target as Human
// analogy doctor is doing Job for Human
 worker: Box<dyn Job<Human,Target = Human>>
}

fn main(){
    let doctor=Doctor{
        name:"qwerty".to_string()
    };
    let human=Human{
        name:"Jeeva".to_string(),
        geneder:"male".to_string()
    };
    let d=HumanWorker{
        worker:Box::new(doctor)
    };
    let m=HumanWorker{
        worker:Box::new(Maid)
    };
    d.worker.work(&human);
    m.worker.work(&human);
}



*/