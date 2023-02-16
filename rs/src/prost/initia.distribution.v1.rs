// @generated
/// Pool is a Coins wrapper with denom.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pool {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// DecPool is a DecCoins wrapper with denom.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DecPool {
    #[prost(string, tag="1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub dec_coins: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// ValidatorHistoricalRewards represents historical rewards for a validator.
/// Height is implicit within the store key.
/// Cumulative reward ratio is the sum from the zeroeth period
/// until this period of rewards / tokens, per the spec.
/// The reference count indicates the number of objects
/// which might need to reference this historical entry at any point.
/// ReferenceCount =
///     number of outstanding delegations which ended the associated period (and
///     might need to read that record)
///   + number of slashes which ended the associated period (and might need to
///   read that record)
///   + one per validator for the zeroeth period, set on initialization
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorHistoricalRewards {
    #[prost(message, repeated, tag="1")]
    pub cumulative_reward_ratios: ::prost::alloc::vec::Vec<DecPool>,
    #[prost(uint32, tag="2")]
    pub reference_count: u32,
}
/// ValidatorCurrentRewards represents current rewards and current
/// period for a validator kept as a running counter and incremented
/// each block as long as the validator's tokens remain constant.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorCurrentRewards {
    #[prost(message, repeated, tag="1")]
    pub rewards: ::prost::alloc::vec::Vec<DecPool>,
    #[prost(uint64, tag="2")]
    pub period: u64,
}
/// ValidatorAccumulatedCommission represents accumulated commission
/// for a validator kept as a running counter, can be withdrawn at any time.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorAccumulatedCommission {
    #[prost(message, repeated, tag="1")]
    pub commissions: ::prost::alloc::vec::Vec<DecPool>,
}
/// ValidatorOutstandingRewards represents outstanding (un-withdrawn) rewards
/// for a validator inexpensive to track, allows simple sanity checks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorOutstandingRewards {
    #[prost(message, repeated, tag="1")]
    pub rewards: ::prost::alloc::vec::Vec<DecPool>,
}
/// ValidatorSlashEvent represents a validator slash event.
/// Height is implicit within the store key.
/// This is needed to calculate appropriate amount of staking tokens
/// for delegations which are withdrawn after a slash has occurred.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEvent {
    #[prost(uint64, tag="1")]
    pub validator_period: u64,
    #[prost(message, repeated, tag="2")]
    pub fractions: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
}
/// ValidatorSlashEvents is a collection of ValidatorSlashEvent messages.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEvents {
    #[prost(message, repeated, tag="1")]
    pub validator_slash_events: ::prost::alloc::vec::Vec<ValidatorSlashEvent>,
}
/// DelegatorStartingInfo represents the starting info for a delegator reward
/// period. It tracks the previous validator period, the delegation's amount of
/// staking token, and the creation height (to check later on if any slashes have
/// occurred). NOTE: Even though validators are slashed to whole staking tokens,
/// the delegators within the validator may be left with less than a full token,
/// thus sdk.Dec is used.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorStartingInfo {
    #[prost(uint64, tag="1")]
    pub previous_period: u64,
    #[prost(message, repeated, tag="2")]
    pub stakes: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::DecCoin>,
    #[prost(uint64, tag="3")]
    pub height: u64,
}
/// ValidatorOutstandingRewardsRecord is used for import/export via genesis json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorOutstandingRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    /// outstanding_rewards represents the oustanding rewards of a validator.
    #[prost(message, repeated, tag="2")]
    pub outstanding_rewards: ::prost::alloc::vec::Vec<DecPool>,
}
/// ValidatorAccumulatedCommissionRecord is used for import / export via genesis
/// json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorAccumulatedCommissionRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    /// accumulated is the accumulated commission of a validator.
    #[prost(message, optional, tag="2")]
    pub accumulated: ::core::option::Option<ValidatorAccumulatedCommission>,
}
/// ValidatorHistoricalRewardsRecord is used for import / export via genesis
/// json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorHistoricalRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    /// period defines the period the historical rewards apply to.
    #[prost(uint64, tag="2")]
    pub period: u64,
    /// rewards defines the historical rewards of a validator.
    #[prost(message, optional, tag="3")]
    pub rewards: ::core::option::Option<ValidatorHistoricalRewards>,
}
/// ValidatorCurrentRewardsRecord is used for import / export via genesis json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorCurrentRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    /// rewards defines the current rewards of a validator.
    #[prost(message, optional, tag="2")]
    pub rewards: ::core::option::Option<ValidatorCurrentRewards>,
}
/// DelegatorStartingInfoRecord used for import / export via genesis json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorStartingInfoRecord {
    /// delegator_address is the address of the delegator.
    #[prost(string, tag="1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_address is the address of the validator.
    #[prost(string, tag="2")]
    pub validator_address: ::prost::alloc::string::String,
    /// starting_info defines the starting info of a delegator.
    #[prost(message, optional, tag="3")]
    pub starting_info: ::core::option::Option<DelegatorStartingInfo>,
}
/// ValidatorSlashEventRecord is used for import / export via genesis json.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEventRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag="1")]
    pub validator_address: ::prost::alloc::string::String,
    /// height defines the block height at which the slash event occured.
    #[prost(uint64, tag="2")]
    pub height: u64,
    /// period is the period of the slash event.
    #[prost(uint64, tag="3")]
    pub period: u64,
    /// validator_slash_event describes the slash event.
    #[prost(message, optional, tag="4")]
    pub validator_slash_event: ::core::option::Option<ValidatorSlashEvent>,
}
/// GenesisState defines the distribution module's genesis state.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the paramaters of the module.
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<super::super::super::cosmos::distribution::v1beta1::Params>,
    /// fee_pool defines the fee pool at genesis.
    #[prost(message, optional, tag="2")]
    pub fee_pool: ::core::option::Option<super::super::super::cosmos::distribution::v1beta1::FeePool>,
    /// fee_pool defines the delegator withdraw infos at genesis.
    #[prost(message, repeated, tag="3")]
    pub delegator_withdraw_infos: ::prost::alloc::vec::Vec<super::super::super::cosmos::distribution::v1beta1::DelegatorWithdrawInfo>,
    /// fee_pool defines the previous proposer at genesis.
    #[prost(string, tag="4")]
    pub previous_proposer: ::prost::alloc::string::String,
    /// fee_pool defines the outstanding rewards of all validators at genesis.
    #[prost(message, repeated, tag="5")]
    pub outstanding_rewards: ::prost::alloc::vec::Vec<ValidatorOutstandingRewardsRecord>,
    /// fee_pool defines the accumulated commisions of all validators at genesis.
    #[prost(message, repeated, tag="6")]
    pub validator_accumulated_commissions: ::prost::alloc::vec::Vec<ValidatorAccumulatedCommissionRecord>,
    /// fee_pool defines the historical rewards of all validators at genesis.
    #[prost(message, repeated, tag="7")]
    pub validator_historical_rewards: ::prost::alloc::vec::Vec<ValidatorHistoricalRewardsRecord>,
    /// fee_pool defines the current rewards of all validators at genesis.
    #[prost(message, repeated, tag="8")]
    pub validator_current_rewards: ::prost::alloc::vec::Vec<ValidatorCurrentRewardsRecord>,
    /// fee_pool defines the delegator starting infos at genesis.
    #[prost(message, repeated, tag="9")]
    pub delegator_starting_infos: ::prost::alloc::vec::Vec<DelegatorStartingInfoRecord>,
    /// fee_pool defines the validator slash events at genesis.
    #[prost(message, repeated, tag="10")]
    pub validator_slash_events: ::prost::alloc::vec::Vec<ValidatorSlashEventRecord>,
}
// @@protoc_insertion_point(module)
