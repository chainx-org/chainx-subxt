use anyhow::Result;
use subxt::{
    balances::{AccountData, Balances},
    extrinsic::DefaultExtra,
    register_default_type_sizes,
    sp_runtime::{traits::BlakeTwo256, MultiAddress, OpaqueExtrinsic},
    system::{AccountStoreExt, System},
    EventTypeRegistry, Runtime,
};

use self::pallet::{
    xgateway_bitcoin_v2::{XGatewayBitcoinV2, XGatewayBitcoinV2EventTypeRegistry},
    xgateway_records::{XGatewayRecords, XGatewayRecordsEventTypeRegistry},
};

mod pallet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainXRuntime;

impl System for ChainXRuntime {
    type Index = xprimitives::Index;
    type BlockNumber = xprimitives::BlockNumber;
    type Hash = xprimitives::Hash;
    type Hashing = BlakeTwo256;
    type AccountId = xprimitives::AccountId;
    type Address = MultiAddress<Self::AccountId, u32>;
    type Header = xprimitives::Header;
    type Extrinsic = OpaqueExtrinsic;
    type AccountData = AccountData<<Self as Balances>::Balance>;
}

impl Balances for ChainXRuntime {
    type Balance = xprimitives::Balance;
}

impl XGatewayBitcoinV2 for ChainXRuntime {}

impl XGatewayRecords for ChainXRuntime {}

impl Runtime for ChainXRuntime {
    type Signature = xprimitives::Signature;
    type Extra = DefaultExtra<Self>;

    fn register_type_sizes(event_type_registry: &mut EventTypeRegistry<Self>) {
        event_type_registry.with_x_gateway_bitcoin_v2();
        event_type_registry.with_x_gateway_records();
        register_default_type_sizes(event_type_registry);
    }
}

#[tokio::test]
async fn test_build_client() -> Result<()> {
    use subxt::ClientBuilder;
    let client = ClientBuilder::<ChainXRuntime>::new()
        .set_url("http://192.168.3.69:8086")
        .build()
        .await?;
    client.block_hash(Some(1.into())).await?;
    Ok(())
}
