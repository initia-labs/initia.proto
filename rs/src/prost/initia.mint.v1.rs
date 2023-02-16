// @generated
/// Params defines the set of mint parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag="1")]
    pub mint_denom: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub inflation_rate: ::prost::alloc::string::String,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryInflationRequest is the request type for the Query/Inflation RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInflationRequest {
}
/// QueryInflationResponse is the response type for the Query/Inflation RPC
/// method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryInflationResponse {
    /// inflation is the current minting inflation value.
    #[prost(bytes="vec", tag="1")]
    pub inflation: ::prost::alloc::vec::Vec<u8>,
}
/// QueryAnnualProvisionsRequest is the request type for the
/// Query/AnnualProvisions RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnualProvisionsRequest {
}
/// QueryAnnualProvisionsResponse is the response type for the
/// Query/AnnualProvisions RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnualProvisionsResponse {
    /// annual_provisions is the current minting annual provisions value.
    #[prost(bytes="vec", tag="1")]
    pub annual_provisions: ::prost::alloc::vec::Vec<u8>,
}
/// GenesisState defines the mint module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// Params defines all the paramaters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, optional, tag="2")]
    pub last_mint_timestamp: ::core::option::Option<::prost_types::Timestamp>,
}
// @@protoc_insertion_point(module)
