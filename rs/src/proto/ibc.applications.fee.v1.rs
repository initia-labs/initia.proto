// @generated
/// Fee defines the ICS29 receive, acknowledgement and timeout fees
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Fee {
    /// the packet receive fee
    #[prost(message, repeated, tag="1")]
    pub recv_fee: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// the packet acknowledgement fee
    #[prost(message, repeated, tag="2")]
    pub ack_fee: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
    /// the packet timeout fee
    #[prost(message, repeated, tag="3")]
    pub timeout_fee: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// PacketFee contains ICS29 relayer fees, refund address and optional list of permitted relayers
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketFee {
    /// fee encapsulates the recv, ack and timeout fees associated with an IBC packet
    #[prost(message, optional, tag="1")]
    pub fee: ::core::option::Option<Fee>,
    /// the refund address for unspent fees
    #[prost(string, tag="2")]
    pub refund_address: ::prost::alloc::string::String,
    /// optional list of relayers permitted to receive fees
    #[prost(string, repeated, tag="3")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// PacketFees contains a list of type PacketFee
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PacketFees {
    /// list of packet fees
    #[prost(message, repeated, tag="1")]
    pub packet_fees: ::prost::alloc::vec::Vec<PacketFee>,
}
/// IdentifiedPacketFees contains a list of type PacketFee and associated PacketId
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentifiedPacketFees {
    /// unique packet identifier comprised of the channel ID, port ID and sequence
    #[prost(message, optional, tag="1")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
    /// list of packet fees
    #[prost(message, repeated, tag="2")]
    pub packet_fees: ::prost::alloc::vec::Vec<PacketFee>,
}
/// GenesisState defines the ICS29 fee middleware genesis state
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// list of identified packet fees
    #[prost(message, repeated, tag="1")]
    pub identified_fees: ::prost::alloc::vec::Vec<IdentifiedPacketFees>,
    /// list of fee enabled channels
    #[prost(message, repeated, tag="2")]
    pub fee_enabled_channels: ::prost::alloc::vec::Vec<FeeEnabledChannel>,
    /// list of registered relayer addresses
    #[prost(message, repeated, tag="3")]
    pub registered_relayers: ::prost::alloc::vec::Vec<RegisteredRelayerAddress>,
    /// list of forward relayer addresses
    #[prost(message, repeated, tag="4")]
    pub forward_relayers: ::prost::alloc::vec::Vec<ForwardRelayerAddress>,
}
/// FeeEnabledChannel contains the PortID & ChannelID for a fee enabled channel
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeEnabledChannel {
    /// unique port identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// unique channel identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
}
/// RegisteredRelayerAddress contains the address and counterparty address for a specific relayer (for distributing fees)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RegisteredRelayerAddress {
    /// the relayer address
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// the counterparty relayer address
    #[prost(string, tag="2")]
    pub counterparty_address: ::prost::alloc::string::String,
    /// unique channel identifier
    #[prost(string, tag="3")]
    pub channel_id: ::prost::alloc::string::String,
}
/// ForwardRelayerAddress contains the forward relayer address and PacketId used for async acknowledgements
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ForwardRelayerAddress {
    /// the forward relayer address
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// unique packet identifer comprised of the channel ID, port ID and sequence
    #[prost(message, optional, tag="2")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
}
/// QueryIncentivizedPacketsRequest defines the request type for the IncentivizedPackets rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPacketsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
    /// block height at which to query
    #[prost(uint64, tag="2")]
    pub query_height: u64,
}
/// QueryIncentivizedPacketsResponse defines the response type for the IncentivizedPackets rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPacketsResponse {
    /// list of identified fees for incentivized packets
    #[prost(message, repeated, tag="1")]
    pub incentivized_packets: ::prost::alloc::vec::Vec<IdentifiedPacketFees>,
}
/// QueryIncentivizedPacketRequest defines the request type for the IncentivizedPacket rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPacketRequest {
    /// unique packet identifier comprised of channel ID, port ID and sequence
    #[prost(message, optional, tag="1")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
    /// block height at which to query
    #[prost(uint64, tag="2")]
    pub query_height: u64,
}
/// QueryIncentivizedPacketsResponse defines the response type for the IncentivizedPacket rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPacketResponse {
    /// the identified fees for the incentivized packet
    #[prost(message, optional, tag="1")]
    pub incentivized_packet: ::core::option::Option<IdentifiedPacketFees>,
}
/// QueryIncentivizedPacketsForChannelRequest defines the request type for querying for all incentivized packets
/// for a specific channel
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPacketsForChannelRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(string, tag="2")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub channel_id: ::prost::alloc::string::String,
    /// Height to query at
    #[prost(uint64, tag="4")]
    pub query_height: u64,
}
/// QueryIncentivizedPacketsResponse defines the response type for the incentivized packets RPC
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIncentivizedPacketsForChannelResponse {
    /// Map of all incentivized_packets
    #[prost(message, repeated, tag="1")]
    pub incentivized_packets: ::prost::alloc::vec::Vec<IdentifiedPacketFees>,
}
/// QueryTotalRecvFeesRequest defines the request type for the TotalRecvFees rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalRecvFeesRequest {
    /// the packet identifier for the associated fees
    #[prost(message, optional, tag="1")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
}
/// QueryTotalRecvFeesResponse defines the response type for the TotalRecvFees rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalRecvFeesResponse {
    /// the total packet receive fees
    #[prost(message, repeated, tag="1")]
    pub recv_fees: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryTotalAckFeesRequest defines the request type for the TotalAckFees rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalAckFeesRequest {
    /// the packet identifier for the associated fees
    #[prost(message, optional, tag="1")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
}
/// QueryTotalAckFeesResponse defines the response type for the TotalAckFees rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalAckFeesResponse {
    /// the total packet acknowledgement fees
    #[prost(message, repeated, tag="1")]
    pub ack_fees: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryTotalTimeoutFeesRequest defines the request type for the TotalTimeoutFees rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalTimeoutFeesRequest {
    /// the packet identifier for the associated fees
    #[prost(message, optional, tag="1")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
}
/// QueryTotalTimeoutFeesResponse defines the response type for the TotalTimeoutFees rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalTimeoutFeesResponse {
    /// the total packet timeout fees
    #[prost(message, repeated, tag="1")]
    pub timeout_fees: ::prost::alloc::vec::Vec<super::super::super::super::cosmos::base::v1beta1::Coin>,
}
/// QueryCounterpartyAddressRequest defines the request type for the CounterpartyAddress rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCounterpartyAddressRequest {
    /// unique channel identifier
    #[prost(string, tag="1")]
    pub channel_id: ::prost::alloc::string::String,
    /// the relayer address to which the counterparty is registered
    #[prost(string, tag="2")]
    pub relayer_address: ::prost::alloc::string::String,
}
/// QueryCounterpartyAddressResponse defines the response type for the CounterpartyAddress rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCounterpartyAddressResponse {
    /// the counterparty address used to compensate forward relaying
    #[prost(string, tag="1")]
    pub counterparty_address: ::prost::alloc::string::String,
}
/// QueryFeeEnabledChannelsRequest defines the request type for the FeeEnabledChannels rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeEnabledChannelsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag="1")]
    pub pagination: ::core::option::Option<super::super::super::super::cosmos::base::query::v1beta1::PageRequest>,
    /// block height at which to query
    #[prost(uint64, tag="2")]
    pub query_height: u64,
}
/// QueryFeeEnabledChannelsResponse defines the response type for the FeeEnabledChannels rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeEnabledChannelsResponse {
    /// list of fee enabled channels
    #[prost(message, repeated, tag="1")]
    pub fee_enabled_channels: ::prost::alloc::vec::Vec<FeeEnabledChannel>,
}
/// QueryFeeEnabledChannelRequest defines the request type for the FeeEnabledChannel rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeEnabledChannelRequest {
    /// unique port identifier
    #[prost(string, tag="1")]
    pub port_id: ::prost::alloc::string::String,
    /// unique channel identifier
    #[prost(string, tag="2")]
    pub channel_id: ::prost::alloc::string::String,
}
/// QueryFeeEnabledChannelResponse defines the response type for the FeeEnabledChannel rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeEnabledChannelResponse {
    /// boolean flag representing the fee enabled channel status
    #[prost(bool, tag="1")]
    pub fee_enabled: bool,
}
/// Metadata defines the ICS29 channel specific metadata encoded into the channel version bytestring
/// See ICS004: <https://github.com/cosmos/ibc/tree/master/spec/core/ics-004-channel-and-packet-semantics#Versioning>
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Metadata {
    /// fee_version defines the ICS29 fee version
    #[prost(string, tag="1")]
    pub fee_version: ::prost::alloc::string::String,
    /// app_version defines the underlying application version, which may or may not be a JSON encoded bytestring
    #[prost(string, tag="2")]
    pub app_version: ::prost::alloc::string::String,
}
/// MsgRegisterCounterpartyAddress defines the request type for the RegisterCounterpartyAddress rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterCounterpartyAddress {
    /// the relayer address
    #[prost(string, tag="1")]
    pub address: ::prost::alloc::string::String,
    /// the counterparty relayer address
    #[prost(string, tag="2")]
    pub counterparty_address: ::prost::alloc::string::String,
    /// unique channel identifier
    #[prost(string, tag="3")]
    pub channel_id: ::prost::alloc::string::String,
}
/// MsgRegisterCounterpartyAddressResponse defines the response type for the RegisterCounterpartyAddress rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRegisterCounterpartyAddressResponse {
}
/// MsgPayPacketFee defines the request type for the PayPacketFee rpc
/// This Msg can be used to pay for a packet at the next sequence send & should be combined with the Msg that will be
/// paid for
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPayPacketFee {
    /// fee encapsulates the recv, ack and timeout fees associated with an IBC packet
    #[prost(message, optional, tag="1")]
    pub fee: ::core::option::Option<Fee>,
    /// the source port unique identifier
    #[prost(string, tag="2")]
    pub source_port_id: ::prost::alloc::string::String,
    /// the source channel unique identifer
    #[prost(string, tag="3")]
    pub source_channel_id: ::prost::alloc::string::String,
    /// account address to refund fee if necessary
    #[prost(string, tag="4")]
    pub signer: ::prost::alloc::string::String,
    /// optional list of relayers permitted to the receive packet fees
    #[prost(string, repeated, tag="5")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgPayPacketFeeResponse defines the response type for the PayPacketFee rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPayPacketFeeResponse {
}
/// MsgPayPacketFeeAsync defines the request type for the PayPacketFeeAsync rpc
/// This Msg can be used to pay for a packet at a specified sequence (instead of the next sequence send)
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPayPacketFeeAsync {
    /// unique packet identifier comprised of the channel ID, port ID and sequence
    #[prost(message, optional, tag="1")]
    pub packet_id: ::core::option::Option<super::super::super::core::channel::v1::PacketId>,
    /// the packet fee associated with a particular IBC packet
    #[prost(message, optional, tag="2")]
    pub packet_fee: ::core::option::Option<PacketFee>,
}
/// MsgPayPacketFeeAsyncResponse defines the response type for the PayPacketFeeAsync rpc
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPayPacketFeeAsyncResponse {
}
/// IncentivizedAcknowledgement is the acknowledgement format to be used by applications wrapped in the fee middleware
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IncentivizedAcknowledgement {
    /// the underlying app acknowledgement result bytes
    #[prost(bytes="vec", tag="1")]
    pub result: ::prost::alloc::vec::Vec<u8>,
    /// the relayer address which submits the recv packet message
    #[prost(string, tag="2")]
    pub forward_relayer_address: ::prost::alloc::string::String,
    /// success flag of the base application callback
    #[prost(bool, tag="3")]
    pub underlying_app_success: bool,
}
include!("ibc.applications.fee.v1.tonic.rs");
// @@protoc_insertion_point(module)
