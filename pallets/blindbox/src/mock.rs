#![cfg(test)]

use crate as blindbox;
use super::*;
use frame_support::{
    construct_runtime, parameter_types, ord_parameter_types, weights::Weight,
    impl_outer_event, impl_outer_origin, impl_outer_dispatch, traits::EnsureOrigin,
};
use sp_core::H256;
use sp_runtime::{testing::Header, traits::IdentityLookup, ModuleId, Perbill};
use primitives::{CurrencyId, Amount, BlindBoxId};
use frame_system::{EnsureSignedBy, EnsureRoot};
use frame_support::pallet_prelude::{MaybeSerializeDeserialize, Hooks, GenesisBuild};
use frame_support::sp_runtime::traits::AtLeast32Bit;
use frame_support::traits::Randomness;
pub type AccountId = u128;
pub type AuctionId = u64;
pub type Balance = u64;
pub type CountryId = u64;
pub type BlockNumber = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;
pub const BLINDBOX_ID: BlindBoxId = 1;
pub const SUCCESS_BLINDBOX_ID: BlindBoxId = 123;
pub const FAIL_BLINDBOX_ID: BlindBoxId = 999;

pub const COUNTRY_ID_NOT_EXIST: CountryId = 1;



// Configure a mock runtime to test the pallet.

/// Provides an implementation of [`frame_support::traits::Randomness`] that should only be used in tests!
pub struct TestRandomness<T>(sp_std::marker::PhantomData<T>);

impl<Output: codec::Decode + Default, T> frame_support::traits::Randomness<Output>
for TestRandomness<T>
    where
        T: frame_system::Config,
{
    fn random(subject: &[u8]) -> (Output) {
        use sp_runtime::traits::TrailingZeroInput;
        (
            Output::decode(&mut TrailingZeroInput::new(subject)).unwrap_or_default()
        )
    }
}

parameter_types! {
	pub const BlockHashCount: u64 = 250;
	pub const MaximumBlockWeight: u32 = 1024;
	pub const MaximumBlockLength: u32 = 2 * 1024;
	pub const AvailableBlockRatio: Perbill = Perbill::one();
}

impl frame_system::Config for Runtime {
    type Origin = Origin;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Call = Call;
    type Hash = H256;
    type Hashing = ::sp_runtime::traits::BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type BlockWeights = ();
    type BlockLength = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type DbWeight = ();
    type BaseCallFilter = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
}

parameter_types! {
	pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Runtime {
    type Balance = Balance;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type MaxLocks = ();
    type WeightInfo = ();
}

parameter_types! {
	pub const CountryFundModuleId: ModuleId = ModuleId(*b"bit/fund");
    pub const MaxNumberOfBlindBox: u32 = 50;
    pub const MaxKSMAllowed: u32 = 200000;
    pub const MaxNUUMAllowed: u32 = 100;
    pub const MaxCollectableNFTAllowed: u32 = 5;
    pub const MaxNFTHatAllowed: u32 = 200;
    pub const MaxNFTJacketAllowed: u32 = 200;
    pub const MaxNFTPantAllowed: u32 = 200;
    pub const MaxNFTShoesAllowed: u32 = 200;
}

impl Config for Runtime {
    type Event = Event;
    type ModuleId = CountryFundModuleId;
    type Randomness = TestRandomness<Self>;
    type MaxNumberOfBlindBox = MaxNumberOfBlindBox;
    type MaxKSMAllowed = MaxKSMAllowed;
    type MaxNUUMBoxAllowed = MaxNUUMAllowed;
    type MaxCollectableNFTAllowed = MaxCollectableNFTAllowed;
    type MaxNFTHatAllowed = MaxNFTHatAllowed;
    type MaxNFTJacketAllowed = MaxNFTJacketAllowed;
    type MaxNFTPantAllowed = MaxNFTPantAllowed;
    type MaxNFTShoesAllowed = MaxNFTShoesAllowed;
    type Currency = Balances;
}

construct_runtime!(
	pub enum Runtime where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic
	{
		System: frame_system::{Module, Call, Config, Storage, Event<T>},
		Balances: pallet_balances::{Module, Call, Storage, Config<T>, Event<T>},
        CollectiveFlip: pallet_randomness_collective_flip::{Module, Call, Storage},
        Blindbox: blindbox::{Module, Call ,Storage, Event<T>},
	}
);


pub type BlindBoxModule = Module<Runtime>;

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

pub struct ExtBuilder;

impl Default for ExtBuilder {
    fn default() -> Self {
        ExtBuilder
    }
}

impl ExtBuilder {
    pub fn build(self) -> sp_io::TestExternalities {
        let mut t = frame_system::GenesisConfig::default()
            .build_storage::<Runtime>()
            .unwrap();

        // pallet_balances::GenesisConfig::<Runtime> {
        //     balances: vec![(ALICE, 100000)],
        // }
        //     .assimilate_storage(&mut t)
        //     .unwrap();

        let mut ext = sp_io::TestExternalities::new(t);
        ext.execute_with(|| System::set_block_number(1));
        ext
    }
}

pub fn last_event() -> Event {
    frame_system::Module::<Runtime>::events()
        .pop()
        .expect("Event expected")
        .event
}