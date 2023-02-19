// @generated
/// Params defines the set of move parameters.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag="1")]
    pub base_denom: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub max_module_size: u64,
}
/// Resource is data for the stored move resource
/// ex) 0000000000000000000000000000000000000002/1/0x1::BasicCoin::Coin<0x1::BasicCoin::Initia>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub struct_tag: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub move_resource: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub raw_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// Module is data for the uploaded contract move code
/// ex) 0000000000000000000000000000000000000001/0/BasicCoin
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub module_name: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub abi: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="4")]
    pub raw_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration="UpgradePolicy", tag="5")]
    pub upgrade_policy: i32,
}
/// TableEntry is data stored under Table address
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableEntry {
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    #[prost(bytes="vec", tag="2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes="vec", tag="3")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// proto wrapper to store the value
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradePolicyProto {
    #[prost(enumeration="UpgradePolicy", tag="1")]
    pub policy: i32,
}
/// UpgradePolicy
/// * `arbitrary`
///      Whether unconditional code upgrade with no compatibility check is allowed. This
///      publication mode should only be used for modules which aren't shared with user others.
///      The developer is responsible for not breaking memory layout of any resources he already
///      stored on chain.
/// * `compatible`
///      Whether a compatibility check should be performed for upgrades. The check only passes if
///      a new module has (a) the same public functions (b) for existing resources, no layout change.
/// * `immutable`
///      Whether the modules in the package are immutable and cannot be upgraded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpgradePolicy {
    Arbitrary = 0,
    Compatible = 1,
    Immutable = 2,
}
impl UpgradePolicy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpgradePolicy::Arbitrary => "ARBITRARY",
            UpgradePolicy::Compatible => "COMPATIBLE",
            UpgradePolicy::Immutable => "IMMUTABLE",
        }
    }
}
/// GenesisState - genesis state of x/move
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(bytes="vec", tag="2")]
    pub execution_counter: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag="3")]
    pub stdlibs: ::prost::alloc::vec::Vec<Module>,
    #[prost(message, repeated, tag="4")]
    pub modules: ::prost::alloc::vec::Vec<Module>,
    #[prost(message, repeated, tag="5")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
    #[prost(message, repeated, tag="6")]
    pub table_entries: ::prost::alloc::vec::Vec<TableEntry>,
}
/// MsgPublish is the message to store compiled Move module
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPublish {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// CodeBytes is raw move module bytes code
    #[prost(bytes="vec", repeated, tag="2")]
    pub code_bytes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// UpgradePolicy defines upgrade rules which will be applied
    /// at next publish message.
    /// Upgrades in the direction of enhancing security are permitted.
    /// `ARBITRARY` => `COMPATIBLE`
    /// `ARBITRARY` => `IMMUTABLE`
    /// `COMPATIBLE` => `IMMUTABLE`
    /// but reverse ways are not allowed (ignored).
    #[prost(enumeration="UpgradePolicy", tag="3")]
    pub upgrade_policy: i32,
}
/// MsgPublishResponse returns store result data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPublishResponse {
}
/// MsgExecute is the message to execute the given module function
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecute {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// ModuleAddr is the address of the module deployer
    #[prost(string, tag="2")]
    pub module_address: ::prost::alloc::string::String,
    /// ModuleName is the name of module to execute
    #[prost(string, tag="3")]
    pub module_name: ::prost::alloc::string::String,
    /// FunctionName is the name of a function to execute
    #[prost(string, tag="4")]
    pub function_name: ::prost::alloc::string::String,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag="5")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute
    /// - number: little endian
    /// - string: base64 bytes
    #[prost(bytes="vec", repeated, tag="6")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MsgExecuteResponse returns execution result data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteResponse {
}
/// MsgScript is the message to execute script code with sender as signer
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgScript {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    /// CodeBytes is the script bytes code to execute
    #[prost(bytes="vec", tag="2")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag="3")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute
    /// - number: little endian
    /// - string: base64 bytes
    #[prost(bytes="vec", repeated, tag="4")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MsgScriptResponse returns execution result data.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgScriptResponse {
}
/// PublishProposal gov proposal content type to submit stdlib module to the system
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishProposal {
    /// Title is a short summary
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// CodeBytes raw move module bytes code
    #[prost(bytes="vec", tag="3")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// ExecuteProposal gov proposal content type to execute entry function to the system
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteProposal {
    /// Title is a short summary
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    /// Description is a human readable text
    #[prost(string, tag="2")]
    pub description: ::prost::alloc::string::String,
    /// ModuleAddress is the module address of entry function
    #[prost(string, tag="3")]
    pub module_address: ::prost::alloc::string::String,
    /// ModuleName is move module name
    #[prost(string, tag="4")]
    pub module_name: ::prost::alloc::string::String,
    /// FunctionName is move function name
    #[prost(string, tag="5")]
    pub function_name: ::prost::alloc::string::String,
    /// TypeArgs is move function type args
    #[prost(string, repeated, tag="6")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is move function args
    #[prost(bytes="vec", repeated, tag="7")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// QueryModuleRequest is the request type for the Query/Module RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleRequest {
    /// address is the owner address of the module to query
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// module_name is the module name to query
    #[prost(string, tag="2")]
    pub module_name: ::prost::alloc::string::String,
}
/// QueryModuleResponse is the response type for the Query/Module RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleResponse {
    #[prost(message, optional, tag="1")]
    pub module: ::core::option::Option<Module>,
}
/// QueryModulesRequest is the request type for the Query/Modules
/// RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModulesRequest {
    /// address is the owner address of the modules to query
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryModulesResponse is the response type for the
/// Query/Modules RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModulesResponse {
    #[prost(message, repeated, tag="1")]
    pub modules: ::prost::alloc::vec::Vec<Module>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryResourceRequest is the request type for the Query/Resource RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResourceRequest {
    /// address is the owner address of the module to query
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// struct_tag is the unique identifier of the resource to query
    #[prost(string, tag="2")]
    pub struct_tag: ::prost::alloc::string::String,
}
/// QueryResourceResponse is the response type for the Query/Resource RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResourceResponse {
    #[prost(message, optional, tag="1")]
    pub resource: ::core::option::Option<Resource>,
}
/// QueryResourcesRequest is the request type for the Query/Resources RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResourcesRequest {
    /// address is the owner address of the module to query
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryResourcesResponse is the response type for the Query/Resources RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResourcesResponse {
    #[prost(message, repeated, tag="1")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryEntryFunctionRequest is the request type for the Query/EntryFunction
/// RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEntryFunctionRequest {
    /// Address is the owner address of the module to query
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// ModuleName is the module name of the entry function to query
    #[prost(string, tag="2")]
    pub module_name: ::prost::alloc::string::String,
    /// FunctionName is the name of a function to query
    #[prost(string, tag="3")]
    pub function_name: ::prost::alloc::string::String,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag="4")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute
    /// - number: little endian
    /// - string: base64 bytes
    #[prost(bytes="vec", repeated, tag="5")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// QueryEntryFunctionResponse is the response type for the
/// Query/EntryFunction RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEntryFunctionResponse {
    #[prost(string, tag="1")]
    pub data: ::prost::alloc::string::String,
}
/// QueryScriptABIRequest is the request type for the Query/ScriptABI
/// RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryScriptAbiRequest {
    /// CodeBytes is the script code for query operation
    #[prost(bytes="vec", tag="1")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// QueryScriptABIResponse is the response type for the
/// Query/ScriptABI RPC method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryScriptAbiResponse {
    #[prost(bytes="vec", tag="1")]
    pub abi: ::prost::alloc::vec::Vec<u8>,
}
/// QueryStructTagRequest is the request type for the Query/StructTag RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStructTagRequest {
    /// denom is the denomination of the coin
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryStructTagResponse is the response type for the Query/StructTag RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStructTagResponse {
    #[prost(string, tag="1")]
    pub struct_tag: ::prost::alloc::string::String,
}
/// QueryDenomRequest is the request type for the Query/Denom RPC
/// method
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomRequest {
    /// struct tag is the move unique identifier of the coin denomination
    #[prost(string, tag="1")]
    pub struct_tag: ::prost::alloc::string::String,
}
/// QueryDenomResponse is the response type for the Query/Denom RPC method.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomResponse {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
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
include!("initia.move.v1.tonic.rs");
// @@protoc_insertion_point(module)
