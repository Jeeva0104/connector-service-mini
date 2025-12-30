#[derive(Debug, Clone)]
pub struct Authorize;

#[derive(strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum FlowName {
    Authorize,
}
