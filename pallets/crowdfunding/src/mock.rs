use crate as pallet_crowdfunding;
use common_types::CurrencyId;
use frame_support::traits::{ConstU16, ConstU64, Nothing};
use frame_support::{pallet_prelude::*, parameter_types, PalletId};
use frame_system::EnsureRoot;
use orml_traits::MultiCurrency;
use pallet_deposits::traits::DepositHandler;
use sp_arithmetic::per_things::Percent;
use sp_core::H256;
use sp_runtime::{
    traits::{BlakeTwo256, IdentityLookup},
    BuildStorage,
};

type Block = frame_system::mocking::MockBlock<Test>;
pub type BlockNumber = u64;
pub type Balance = u64;
pub type AccountId = u128;
pub type Moment = u64;
// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
    pub enum Test
    {
        System: frame_system,
        CrowdFunding: pallet_crowdfunding,
        Tokens: orml_tokens,
        Identity: pallet_identity,
        Balances: pallet_balances,
        Proposals: pallet_proposals,
        Timestamp: pallet_timestamp,
    }
);

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Nonce = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Block = Block;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU64<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ConstU16<42>;
    type OnSetCode = ();
    type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
    pub RoundExpiry: BlockNumber = 100;
    pub MaxKeysPerRound: u32 = 50;
    pub MaxContributionsPerCrowdFund: u32 = 1000;
    pub MaxMilestonesPerCrowdFund: u32 = 50;
    pub MaxWhitelistPerCrowdFund: u32 = 50;
    pub MinimumRequiredFunds: Balance = 2000;
    pub MinimumContribution: Balance = 5;
}

impl pallet_crowdfunding::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type MultiCurrency = Tokens;
    type RoundExpiry = RoundExpiry;
    type MaxContributionsPerCrowdFund = MaxContributionsPerCrowdFund;
    type MaxKeysPerRound = MaxKeysPerRound;
    type MaxMilestonesPerCrowdFund = MaxMilestonesPerCrowdFund;
    type MaxWhitelistPerCrowdFund = MaxWhitelistPerCrowdFund;
    type IsIdentityRequired = IsIdentityRequired;
    type AuthorityOrigin = EnsureRoot<AccountId>;
    type IntoProposals = pallet_proposals::Pallet<Test>;
    type WeightInfo = ();
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

orml_traits::parameter_type_with_key! {
    pub ExistentialDeposits: |_currency_id: CurrencyId| -> Balance {
        100
    };
}
parameter_types! {
    pub const MaxReserves: u32 = 50;
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
    pub const ExistentialDeposit: u64 = 5;
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
    pub const TwoWeekBlockUnit: u32 = 100800u32;
    pub const ProposalsPalletId: PalletId = PalletId(*b"imbgrant");
    pub NoConfidenceTimeLimit: BlockNumber = 100800u32.into();
    pub PercentRequiredForVoteToPass: Percent = Percent::from_percent(75u8);
    pub MaximumContributorsPerProject: u32 = 50;
    pub RefundsPerBlock: u8 = 2;
    pub IsIdentityRequired: bool = false;
    pub MilestoneVotingWindow: BlockNumber  =  100800u64;
    pub MaxMilestonesPerProject: u32 = 10;
    pub ProjectStorageDeposit: Balance = 100;
    pub ImbueFee: Percent = Percent::from_percent(5u8);
    pub ExpiringProjectRoundsPerBlock: u32 = 10;
    pub ProjectStorageItem: StorageItem = StorageItem::Project;
    pub MaxProjectsPerAccount: u16 = 100;
    pub PercentRequiredForVoteNoConfidenceToPass: Percent = Percent::from_percent(75u8);
}

impl pallet_proposals::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type PalletId = ProposalsPalletId;
    type AuthorityOrigin = EnsureRoot<AccountId>;
    type MultiCurrency = Tokens;
    type WeightInfo = pallet_proposals::WeightInfo<Self>;
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
    type DepositHandler = MockDepositHandler;
    type MaxProjectsPerAccount = MaxProjectsPerAccount;
    type PercentRequiredForVoteNoConfidenceToPass = PercentRequiredForVoteNoConfidenceToPass;
}

#[derive(Encode, Decode, PartialEq, Eq, Clone, Debug, MaxEncodedLen, TypeInfo, Copy)]
pub enum StorageItem {
    CrowdFund,
    Brief,
    Grant,
    Project,
}

pub struct MockDepositHandler;
impl DepositHandler<Balance, AccountId> for MockDepositHandler {
    type DepositId = u64;
    type StorageItem = StorageItem;
    fn take_deposit(
        _who: AccountId,
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

pub static ALICE: AccountId = 125;
pub static BOB: AccountId = 126;
pub static CHARLIE: AccountId = 127;

pub(crate) fn new_test_ext() -> sp_io::TestExternalities {
    let t = frame_system::GenesisConfig::<Test>::default()
        .build_storage()
        .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        let initial_balance = 10_000_000u64;
        System::set_block_number(1);
        let _ = Tokens::deposit(CurrencyId::Native, &ALICE, initial_balance);
        let _ = Tokens::deposit(CurrencyId::Native, &BOB, initial_balance);
        let _ = Tokens::deposit(CurrencyId::Native, &CHARLIE, initial_balance);
    });
    ext
}
