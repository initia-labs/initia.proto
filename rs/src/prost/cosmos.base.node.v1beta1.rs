// @generated
/// ConfigRequest defines the request structure for the Config gRPC query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigRequest {
}
/// ConfigResponse defines the response structure for the Config gRPC query.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConfigResponse {
    #[prost(string, tag="1")]
    pub minimum_gas_price: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
