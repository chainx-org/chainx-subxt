use codec::{Decode, Encode};
use std::marker::PhantomData;
use subxt::{balances::Balances, module, system::System, Store};
use xgateway_bitcoin_v2::types::TradingPrice;
use xprimitives::{AccountId, Balance, BlockNumber};
use sp_arithmetic::Percent;

#[module]
#[rustfmt::skip]
pub trait XGatewayBitcoinV2: System + Balances {
    #![event_alias(RequestId=u128)]
    #![event_type(TradingPrice)]
    #![event_type(AccountId)]
    #![event_type(BlockNumber)]
    #![event_type(Balance)]
    #![event_type(Percent)]
}

#[derive(Debug, Clone, Copy, Decode, Encode, Default, Store)]
pub struct TotalCallateralStore<T: XGatewayBitcoinV2> {
    #[store(returns = T::Balance)]
    pub _runtime: PhantomData<T>,
}
