use crate as pallet_proposals;
use frame_support::{
    parameter_types,
    traits::{ConstU32, Nothing},
    weights::{ConstantMultiplier, IdentityFee},
    PalletId,
};

use crate::*;
use common_types::CurrencyId;
use frame_system::EnsureRoot;
use sp_core::H256;

use orml_traits::MultiCurrency;
use sp_arithmetic::per_things::Percent;
use sp_runtime::{
    traits::{AccountIdConversion, BlakeTwo256, IdentityLookup},
    BuildStorage,
};

use sp_std::{
    convert::{TryFrom, TryInto},
    str,
};

type Block = frame_system::mocking::MockBlock<Test>;

pub type BlockNumber = u64;
pub type Amount = i128;
pub type Balance = u64;
pub type Moment = u64;
pub type AccountId = u128;

parameter_types! {
    pub const GetNativeCurrencyId: CurrencyId = CurrencyId::Native;
}

pub type AdaptedBasicCurrency =
    orml_currencies::BasicCurrencyAdapter<Test, Balances, Amount, BlockNumber>;

impl orml_currencies::Config for Test {
    type GetNativeCurrencyId = GetNativeCurrencyId;
    type MultiCurrency = Tokens;
    type NativeCurrency = AdaptedBasicCurrency;
    type WeightInfo = ();
}

frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Tokens: orml_tokens::{Pallet, Storage, Event<T>},
        Currencies: orml_currencies::{Pallet, Call, Storage},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>},
        Proposals: pallet_proposals::{Pallet, Call, Storage, Event<T>},
        IdentityPallet: pallet_identity::{Pallet, Call, Storage, Event<T>},
    }
);

orml_traits::parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
        1
    };
}

parameter_types! {
    pub DustAccount: AccountId = PalletId(*b"orml/dst").into_account_truncating();
    pub MaxLocks: u32 = 2;
}

impl orml_tokens::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type Amount = i128;
    type CurrencyId = common_types::CurrencyId;
    type CurrencyHooks = ();
    type WeightInfo = ();
    type ExistentialDeposits = ExistentialDeposits;
    type MaxLocks = MaxLocks;
    type DustRemovalWhitelist = Nothing;
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

parameter_types! {
    pub const TransactionByteFee: u64 = 1;
    pub const OperationalFeeMultiplier: u8 = 5;
}
impl pallet_transaction_payment::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type OnChargeTransaction = pallet_transaction_payment::CurrencyAdapter<Balances, ()>;
    type WeightToFee = IdentityFee<u64>;
    type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
    type FeeMultiplierUpdate = ();
    type OperationalFeeMultiplier = OperationalFeeMultiplier;
}

parameter_types! {
    pub const BlockHashCount: u64 = 250;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Block = Block;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type BlockHashCount = BlockHashCount;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type RuntimeEvent = RuntimeEvent;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

parameter_types! {
    pub const GracePeriod: u64 = 5;
    pub const UnsignedInterval: u64 = 128;
    pub const UnsignedPriority: u64 = 1 << 20;
}

parameter_types! {
    pub const ExistentialDeposit: u64 = 5;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type AccountStore = System;
    type Balance = u64;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type MaxLocks = ();
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type WeightInfo = ();
    type FreezeIdentifier = ();
    type MaxHolds = ConstU32<0>;
    type MaxFreezes = ConstU32<0>;
    type RuntimeHoldReason = ();
}

parameter_types! {
    pub const MinimumPeriod: u64 = 1;
}
impl pallet_timestamp::Config for Test {
    type Moment = Moment;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}

parameter_types! {
    pub const TwoWeekBlockUnit: u32 = 100800u32;
    pub const ProposalsPalletId: PalletId = PalletId(*b"imbgrant");
    pub NoConfidenceTimeLimit: BlockNumber = 100800u32.into();
    pub PercentRequiredForVoteToPass: Percent = Percent::from_percent(75u8);
    pub MaximumContributorsPerProject: u32 = 50;
    pub RefundsPerBlock: u8 = 2;
    pub IsIdentityRequired: bool = false;
    pub MilestoneVotingWindow: BlockNumber  =  100800u64;
    pub MaxMilestonesPerProject: u32 = 10;
    pub ImbueFee: Percent = Percent::from_percent(5u8);
    pub ExpiringProjectRoundsPerBlock: u32 = 10;
    pub ProjectStorageItem: StorageItems = StorageItems::Project;
    pub MaxProjectsPerAccount: u16 = 50;
    pub PercentRequiredForVoteNoConfidenceToPass: Percent = Percent::from_percent(75u8);
}

impl pallet_proposals::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type PalletId = ProposalsPalletId;
    type AuthorityOrigin = EnsureRoot<AccountId>;
    type MultiCurrency = Tokens;
    type WeightInfo = crate::WeightInfo<Self>;
    // Adding 2 weeks as th expiration time
    type MaxWithdrawalExpiration = TwoWeekBlockUnit;
    type NoConfidenceTimeLimit = NoConfidenceTimeLimit;
    type PercentRequiredForVoteToPass = PercentRequiredForVoteToPass;
    type MaximumContributorsPerProject = MaximumContributorsPerProject;
    type MilestoneVotingWindow = MilestoneVotingWindow;
    type RefundHandler = pallet_proposals::traits::MockRefundHandler<Test>;
    type MaxMilestonesPerProject = MaxMilestonesPerProject;
    type ImbueFee = ImbueFee;
    type ExpiringProjectRoundsPerBlock = ExpiringProjectRoundsPerBlock;
    type ProjectStorageItem = ProjectStorageItem;
    type DepositHandler = MockDepositHandler<Test>;
    type MaxProjectsPerAccount = MaxProjectsPerAccount;
    type PercentRequiredForVoteNoConfidenceToPass = PercentRequiredForVoteNoConfidenceToPass;
}

parameter_types! {
    pub const BasicDeposit: u64 = 10;
    pub const FieldDeposit: u64 = 10;
    pub const SubAccountDeposit: u64 = 10;
    pub const MaxSubAccounts: u32 = 2;
    pub const MaxAdditionalFields: u32 = 2;
    pub const MaxRegistrars: u32 = 20;
}

impl pallet_identity::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type Currency = Balances;
    type Slashed = ();
    type BasicDeposit = BasicDeposit;
    type FieldDeposit = FieldDeposit;
    type SubAccountDeposit = SubAccountDeposit;
    type MaxSubAccounts = MaxSubAccounts;
    type MaxAdditionalFields = MaxAdditionalFields;
    type MaxRegistrars = MaxRegistrars;
    type RegistrarOrigin = EnsureRoot<AccountId>;
    type ForceOrigin = EnsureRoot<AccountId>;
    type WeightInfo = ();
}

parameter_types! {
    pub const UnitWeightCost: u64 = 10;
    pub const MaxInstructions: u32 = 100;
}
pub static ALICE: AccountId = 125;
pub static BOB: AccountId = 126;
pub static CHARLIE: AccountId = 127;
pub static DAVE: AccountId = 128;
pub static STEVE: AccountId = 254;
pub static JOHN: AccountId = 255;

pub(crate) fn build_test_externality() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        let initial_balance = 100_000_000u64;
        System::set_block_number(1);
        let _ = Tokens::deposit(CurrencyId::Native, &ALICE, initial_balance);
        let _ = Tokens::deposit(CurrencyId::Native, &BOB, initial_balance);
        let _ = Tokens::deposit(CurrencyId::Native, &CHARLIE, initial_balance);
        let _ = Tokens::deposit(CurrencyId::Native, &DAVE, initial_balance);
        let _ = Tokens::deposit(CurrencyId::Native, &JOHN, initial_balance);
        let _ = Tokens::deposit(CurrencyId::Native, &STEVE, initial_balance);
    });
    ext
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug, MaxEncodedLen, TypeInfo, Copy)]
pub enum StorageItems {
    Project,
}

pub struct MockDepositHandler<T>(T);
impl<T: crate::Config> DepositHandler<crate::BalanceOf<T>, crate::AccountIdOf<T>>
    for MockDepositHandler<T>
{
    type DepositId = u64;
    type StorageItem = StorageItems;
    fn take_deposit(
        _who: crate::AccountIdOf<T>,
        _storage_item: Self::StorageItem,
        _currency_id: CurrencyId,
    ) -> Result<Self::DepositId, DispatchError> {
        Ok(0u64)
    }
    fn return_deposit(_deposit_id: Self::DepositId) -> DispatchResult {
        Ok(())
    }
    fn slash_reserve_deposit(_deposit_id: Self::DepositId) -> DispatchResult {
        Ok(())
    }
}
