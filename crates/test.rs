use std::marker::PhantomData;

// ----------------------------
// TRAITS
// ----------------------------
trait FlowTypes {
    type Flow;
    type FlowCommonData;
    type Request;
    type Response;
}



trait BridgeRequestResponse {
    type RequestBody;
    type ResponseBody;
    type ConnectorInputData;
}



struct AdyenRouterData<RD: FlowTypes, T: PaymentMethodDataTypes + 'static> {
    pub connector:  Adyen<T>, 
    pub router_data: RD,
}

impl <RD: FlowTypes, T: PaymentMethodDataTypes> FlowTypes for AdyenRouterData<RD, T> {
    type Flow = RD::Flow;
    type FlowCommonData = RD::FlowCommonData;
    type Request = RD::Request;
    type Response = RD::Response;
}

struct AdyenPaymentRequestTemplating;
struct StringTemplating;

struct Bridge<ReqT, ResT, T> (PhantomData<(ReqT, ResT, T)>);


impl <T: PaymentMethodDataTypes + 'static> BridgeRequestResponse for Bridge<AdyenPaymentRequestTemplating, StringTemplating, T>
{
    type RequestBody = AdyenPaymentRequest<T>;
    type ResponseBody = String;

    type ConnectorInputData = AdyenRouterData<
        RouterDataV2<
            Authorize,
            PaymentFlowData,
            PaymentsAuthorizeData<T>,
            PaymentsResponseData,
        >,
        T,
    >;
}



struct Adyen <T: PaymentMethodDataTypes + 'static> {
    authorize: &'static dyn BridgeRequestResponse<
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
    >,
}

impl<T: PaymentMethodDataTypes + 'static> Adyen<T> {
    pub const fn new() -> Self { 
        Self {
            authorize: &Bridge::<AdyenPaymentRequestTemplating, StringTemplating, T>(PhantomData,),
        }
    }
}












// -----------------------------
// FAKE FLOW & REQUEST TYPES
// -----------------------------

struct RouterDataV2<F, CD, Req, Resp> {
    pub flow: PhantomData<F>,
    pub common: PhantomData<CD>,
    pub request: Req,
    pub response: PhantomData<Resp>,
}

impl<F, CD, Req, Resp> FlowTypes for RouterDataV2<F, CD, Req, Resp> {
    type Flow = F;
    type FlowCommonData = CD;
    type Request = Req;
    type Response = Resp;
}
trait PaymentMethodDataTypes {}
struct Authorize;
struct PaymentFlowData;
struct PaymentsResponseData;

#[derive(Debug)]
struct PaymentsAuthorizeData<T: PaymentMethodDataTypes> {
    amount: u32,
    method: PhantomData<T>,
}

#[derive(Debug)]
struct AdyenPaymentRequest<T: PaymentMethodDataTypes> {
    amount: u32,
    method: PhantomData<T>,
}


struct Card;
impl PaymentMethodDataTypes for Card {}


fn main() {
    // -------------------------
    // 1. Build RouterDataV2
    // -------------------------
    let rd = RouterDataV2::<
        Authorize,
        PaymentFlowData,
        PaymentsAuthorizeData<Card>,
        PaymentsResponseData,
    > {
        flow: PhantomData,
        common: PhantomData,
        request: PaymentsAuthorizeData::<Card> {
            amount: 1000,
            method: PhantomData,
        },
        response: PhantomData,
    };

    // -------------------------
    // 2. Build Adyen Connector
    // -------------------------
    // The Adyen::new() function now returns Adyen<Card>, not &'static Adyen<Card>
    let adyen = Adyen::<Card>::new();

    // -------------------------
    // 3. Wrap inside AdyenRouterData
    // -------------------------
    let input = AdyenRouterData {
        connector: adyen, // Fix 3: Borrowed adyen with '&'
        router_data: rd,
    };

    // -------------------------
    // 4. Bridge object
    // -------------------------
    let bridge = Bridge::<AdyenPaymentRequestTemplating, StringTemplating, Card>(PhantomData);

    // -------------------------
    // 5. Use Bridge::RequestBody type
    // -------------------------
    let req: <Bridge::
    <AdyenPaymentRequestTemplating, StringTemplating, Card>
        as BridgeRequestResponse>::RequestBody = AdyenPaymentRequest {
        amount: 1000,
        method: PhantomData,
    };

    println!("Built Request: {:?}", req.amount);
    println!("ConnectorInputData built successfully!");
}