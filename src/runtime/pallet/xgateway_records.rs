use subxt::{module, system::System};
use xgateway_records::{
    WithdrawalRecord as _WithdrawalRecord, WithdrawalRecordId, WithdrawalState,
};
use xprimitives::{AccountId, Balance, BlockNumber};

type WithdrawalRecord = _WithdrawalRecord<AccountId, Balance, BlockNumber>;

#[module]
#[rustfmt::skip]
pub trait XGatewayRecords: System {
    #![event_type(WithdrawalRecordId)]
    #![event_type(WithdrawalState)]
    #![event_type(WithdrawalRecord)]
}
