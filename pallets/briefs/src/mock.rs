use crate as pallet_briefs;
use frame_support::{
    parameter_types,
    traits::{ConstU32, Nothing},
    weights::{ConstantMultiplier, IdentityFee},
    PalletId,
};

use frame_system::EnsureRoot;
use sp_core::{sr25519::Signature, H256};

use crate::mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GenesisBuild;
use crate::pallet::BriefHash;

use common_types::CurrencyId;

use frame_support::once_cell::sync::Lazy;

use sp_core::sr25519;
use sp_runtime::{
    testing::Header,
    traits::{AccountIdConversion, BlakeTwo256, IdentifyAccount, IdentityLookup, Verify},
    BuildStorage,
};

use sp_std::{
    convert::{TryFrom, TryInto},
    str,
    vec::Vec,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

pub type BlockNumber = u64;
pub type Amount = i128;
pub type Balance = u64;
pub type Moment = u64;
//type AccountId = sp_core::sr25519::Public;

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
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Tokens: orml_tokens::{Pallet, Storage, Event<T>},
        Currencies: orml_currencies::{Pallet, Call, Storage},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        TransactionPayment: pallet_transaction_payment::{Pallet, Storage, Event<T>},
        BriefsMod: pallet_briefs::{Pallet, Call, Storage, Event<T>},
        Proposals: pallet_proposals::{Pallet, Call, Storage, Event<T>},
        Identity: pallet_identity::{Pallet, Call, Storage, Event<T>},
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
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
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

pub type AccountId = <<Signature as Verify>::Signer as IdentifyAccount>::AccountId;

impl frame_system::offchain::SigningTypes for Test {
    type Public = <Signature as Verify>::Signer;
    type Signature = Signature;
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
    pub MaximumApplicants: u32 = 10_000u32;
    pub ApplicationSubmissionTime: BlockNumber = 1000u32.into();
    pub MaxBriefOwners: u32 = 100;
}

impl pallet_briefs::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type RMultiCurrency = Tokens;
    type BriefHasher = BlakeTwo256;
    type AuthorityOrigin = EnsureRoot<AccountId>;
    type BriefEvolver = pallet_proposals::Pallet<Test>;
    type MaxBriefOwners = MaxBriefOwners;
}

parameter_types! {
    pub const TwoWeekBlockUnit: u32 = 100800u32;
    pub const ProposalsPalletId: PalletId = PalletId(*b"imbgrant");
    pub NoConfidenceTimeLimit: BlockNumber = 100800u32.into();
    pub PercentRequiredForVoteToPass: u8 = 75u8;
    pub MaximumContributorsPerProject: u32 = 5000;
    pub RefundsPerBlock: u8 = 2;
    pub IsIdentityRequired: bool = false;
}

// Requires binding howerver they may be a more succinct way of doing this.
//impl<T: proposals::Config> BriefEvolver<AccountId, Balance, BlockNumber, Moment> for MockEvolver<T>
//where
//    Project<AccountId, Balance, BlockNumber, Moment>: EncodeLike<
//        Project<
//            <T as frame_system::Config>::AccountId,
//            <<T as proposals::Config>::MultiCurrency as MultiCurrency<
//                <T as frame_system::Config>::AccountId,
//            >>::Balance,
//            <T as frame_system::Config>::BlockNumber,
//            <T as pallet_timestamp::Config>::Moment,
//        >,
//    >,
//{
//    fn convert_to_proposal(
//        currency_id: CurrencyId,
//        contributions: BTreeMap<AccountId, Contribution<Balance, Moment>>,
//        brief_hash: BriefHash,
//        applicant: AccountId,
//        milestones: BTreeMap<MilestoneKey, ProposedMilestone>,
//    ) -> Result<(), ()> {
//        let project_key = proposals::ProjectCount::<Test>::get()
//            .checked_add(1)
//            .ok_or(())?;
//        proposals::ProjectCount::<Test>::put(project_key);
//
//        let sum_of_contributions = contributions
//            .values()
//            .fold(Default::default(), |acc: Balance, x| {
//                acc.saturating_add(x.value)
//            });
//        let mut project_milestones: BTreeMap<MilestoneKey, Milestone> = BTreeMap::new();
//
//        let _ = milestones
//            .into_iter()
//            .map(|i: (MilestoneKey, ProposedMilestone)| {
//                project_milestones.insert(
//                    i.0,
//                    Milestone {
//                        project_key,
//                        milestone_key: i.0,
//                        percentage_to_unlock: i.1.percentage_to_unlock,
//                        is_approved: false,
//                    },
//                )
//            })
//            .collect::<Vec<_>>();
//
//        let project: Project<AccountId, Balance, BlockNumber, Moment> = Project {
//            milestones: project_milestones,
//            contributions: contributions,
//            currency_id,
//            required_funds: sum_of_contributions,
//            withdrawn_funds: 0u32.into(),
//            raised_funds: sum_of_contributions,
//            initiator: applicant,
//            create_block_number: System::block_number(),
//            approved_for_funding: true,
//            funding_threshold_met: true,
//            cancelled: false,
//            agreement_hash: brief_hash,
//            // Maybe we dont need this new field because we have create_block_number
//        };
//
//        Projects::<T>::insert(project_key, project);
//
//        Ok(())
//    }
//}

impl pallet_proposals::Config for Test {
    type RuntimeEvent = RuntimeEvent;
    type PalletId = ProposalsPalletId;
    type AuthorityOrigin = EnsureRoot<AccountId>;
    type MultiCurrency = Tokens;
    type WeightInfo = ();
    type MaxProjectsPerRound = ConstU32<4>;
    // Adding 2 weeks as th expiration time
    type MaxWithdrawalExpiration = TwoWeekBlockUnit;
    type NoConfidenceTimeLimit = NoConfidenceTimeLimit;
    type PercentRequiredForVoteToPass = PercentRequiredForVoteToPass;
    type MaximumContributorsPerProject = MaximumContributorsPerProject;
    type RefundsPerBlock = RefundsPerBlock;
    type IsIdentityRequired = IsIdentityRequired;
    type MilestoneVotingWindow = TwoWeekBlockUnit;

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
pub static ALICE: Lazy<sr25519::Public> = Lazy::new(|| sr25519::Public::from_raw([125u8; 32]));
pub static BOB: Lazy<sr25519::Public> = Lazy::new(|| sr25519::Public::from_raw([126u8; 32]));
pub static CHARLIE: Lazy<sr25519::Public> = Lazy::new(|| sr25519::Public::from_raw([127u8; 32]));

pub fn gen_hash(seed: u8) -> BriefHash {
    H256::from([seed; 32])
}

pub(crate) fn build_test_externality() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();

    GenesisConfig::default().assimilate_storage(&mut t).unwrap();
    orml_tokens::GenesisConfig::<Test> {
        balances: {
            vec![*ALICE, *BOB, *CHARLIE]
                .into_iter()
                .map(|id| (id, CurrencyId::Native, 1000000))
                .collect::<Vec<_>>()
        },
    }
    .assimilate_storage(&mut t)
    .unwrap();

    let mut ext = sp_io::TestExternalities::new(t);
    ext.execute_with(|| {
        System::set_block_number(1);
    });
    ext
}