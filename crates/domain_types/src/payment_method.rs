use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
pub trait PaymentMethodDataTypes: Clone {
    type Inner: Default + Debug + Send + Eq + PartialEq + Serialize + DeserializeOwned + Clone;
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DefaultCardData {
    pub card_number: i64,
}

#[derive(Default, Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct DefaultPCIHolder;

impl PaymentMethodDataTypes for DefaultPCIHolder {
    type Inner = DefaultCardData;
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RawCardNumber<T: PaymentMethodDataTypes>(pub T::Inner);

impl DefaultCardData {
    pub fn peek(&self) -> &i64 {
        &self.card_number
    }
}

impl RawCardNumber<DefaultPCIHolder> {
    pub fn peek(&self) -> &i64 {
        self.0.peek()
    }
}
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Card<T: PaymentMethodDataTypes> {
    pub card_number: RawCardNumber<T>,
    pub card_issuer: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum PaymentMethodData<T: PaymentMethodDataTypes> {
    Card(Card<T>),
}

/*
Reference Code for the above one

use std::fmt::Debug;
trait PaymentMethodDataType{
    type Inner:Debug+Clone;
}


#[derive(Debug)]
pub struct DefaultPCIHolder;
impl PaymentMethodDataType for DefaultPCIHolder{
   type Inner=String;
}

#[derive(Debug)]
pub struct RawCard<T: PaymentMethodDataType>(pub T::Inner);

impl<T: PaymentMethodDataType> RawCard<T>{
    pub fn peek(&self)->T::Inner{
        return self.0.clone()
    }
}


#[derive(Debug)]
pub struct Card<T:PaymentMethodDataType>{
    card_number:RawCard<T>
}

#[derive(Debug)]
pub enum PaymentMethodData<T:PaymentMethodDataType>{
    Card(Card<T>)
}

#[derive(Debug)]
pub struct Authorize;

pub trait ConnectorService<T:PaymentMethodDataType>:PaymentAuthorizeV2<T>{}

pub trait PaymentAuthorizeV2<T:PaymentMethodDataType>:ConnectorIntegrationV2<Authorize>{}

pub trait ConnectorIntegrationV2<Flow>:ConnectorIntegrationAnyV2<Flow>+Debug {
   fn get_url(&self);
}
pub trait ConnectorIntegrationAnyV2<Flow>{
    fn get_connector_integration_v2(&self)-> BoxedConnectorIntegrationV2<'_, Flow>;
}

pub type BoxedConnectorIntegrationV2<'a, Flow> =
    Box<&'a (dyn ConnectorIntegrationV2<Flow>)>;
//If someone already knows how to “speak advanced English” (ConnectorIntegrationV2),
//then they automatically know how to “speak basic English” (ConnectorIntegrationAnyV2).

//So you don’t need to teach them twice!
//Now notice:

//We only implemented ConnectorIntegrationV2.

//We did not implement ConnectorIntegrationAnyV2.

//But because of the impl rule you wrote,
// Rust automatically adds ConnectorIntegrationAnyV2 for us!
impl <S,Flow> ConnectorIntegrationAnyV2<Flow> for S
where
S:ConnectorIntegrationV2<Flow>{
    fn get_connector_integration_v2(&self)->BoxedConnectorIntegrationV2<Flow>{
         Box::new(self)
    }
}

//example how the above implementation will be implemented is
// impl ConnectorIntegrationV2 for Stripe {
//  fn get_url(&self){
//   println!("Stripe: https://api.stripe.com");
//  }
// }

// if you see we have implemented only get_url method for struct Stripe but rust will automatically will allow us to use get_connector_integration_v2 method also






pub type BoxedConnector<T> = Box<dyn ConnectorService<T>>;

#[derive(Debug)]
pub enum ConnectorEnum{
    Stripe,
}


#[derive(Debug)]
pub struct ConnectorData<T: PaymentMethodDataType> {
    // who ever impl ConnectorService trait we can assign that value to connector
    pub connector: BoxedConnector<T>,
    pub connector_name: ConnectorEnum,
}

#[derive(Debug)]
struct Stripe;
// these implementation are written in macros create_all_prerequisites!
impl ConnectorIntegrationV2<Authorize> for Stripe {
    fn get_url(&self) {
        println!("Stripe: https://api.stripe.com");
    }
}

impl<T:PaymentMethodDataType> ConnectorService<T> for Stripe{
}

impl<T:PaymentMethodDataType> PaymentAuthorizeV2<T> for Stripe{
}

impl <T: PaymentMethodDataType> ConnectorData<T>{
    pub  fn get_connector_by_name(connector_name:ConnectorEnum)->Self{
        Self{
        // stripe is implementing ConnectorService trait so we can assing stripe to connector
          connector:Box::new(Stripe) ,
          connector_name:connector_name
        }
    }
}






fn main(){
    // let raw_card=RawCard::<DefaultPCIHolder>("123455".to_string());
    // let card_data=Card::<DefaultPCIHolder>{
    //     card_number:raw_card
    // };
    // let payment_method=PaymentMethodData::<DefaultPCIHolder>::Card(card_data);

    // let user_card_number = match &payment_method {
    //     PaymentMethodData::Card(Card { card_number: data }) => data.peek()
    // };

    let connector_data=ConnectorData::<DefaultPCIHolder>::get_connector_by_name(ConnectorEnum::Stripe);
    let connector_integration:BoxedConnectorIntegrationV2<
    Authorize
    >
    =
    connector_data.connector.get_connector_integration_v2();


}

*/

