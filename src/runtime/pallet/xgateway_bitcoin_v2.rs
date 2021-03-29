use codec::{Decode, Encode};
use std::marker::PhantomData;
use subxt::{balances::Balances, system::System, Store};

#[subxt::module]
pub trait XGatewayBitcoinV2: System + Balances {}

#[derive(Debug, Clone, Copy, Encode, Store)]
pub struct TotalCallateralStore<T: XGatewayBitcoinV2> {
    #[store(returns = T::Balance)]
    /// Marker
    pub _runtime: PhantomData<T>,
}
