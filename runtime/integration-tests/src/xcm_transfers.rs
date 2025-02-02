// Copyright 2022 Imbue Network (imbue.network).
// This file is part of Imbue chain project.

// Imbue is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version (see http://www.gnu.org/licenses).

// Imbue is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
use frame_support::assert_ok;
use frame_support::dispatch::RawOrigin;

use xcm_emulator::{bx, Chain, TestExt};

use xcm::latest::{Junction, Junction::*, Junctions::*, MultiLocation, NetworkId, WeightLimit};

use common_runtime::{common_xcm::general_key, parachains};

use crate::constants::SAFE_XCM_VERSION;
use crate::kusama_test_net::{
    Development, DevelopmentReceiver, DevelopmentSender, Kusama, KusamaReceiver, KusamaSender,
    Sibling, SiblingReceiver,
};
use crate::setup::{ksm_amount, mgx_amount, native_amount, PARA_ID_DEVELOPMENT, PARA_ID_SIBLING};
use common_runtime::Balance;
use common_types::{CurrencyId, FundingType, TreasuryOrigin};
use imbue_kusama_runtime::{OrmlTokens, Runtime as R, RuntimeOrigin, XTokens};
use orml_traits::MultiCurrency;
use pallet_proposals::traits::RefundHandler;

#[test]
fn transfer_treasury_to_parachain_grant_escrow_address() {
    let transfer_amount: Balance = ksm_amount(1);
    let treasury_origin = TreasuryOrigin::Kusama;
    let kusama_treasury_address =
        <R as pallet_proposals::Config>::RefundHandler::get_treasury_account_id(treasury_origin)
            .unwrap();
    Development::execute_with(|| {
        assert_eq!(
            OrmlTokens::free_balance(CurrencyId::KSM, &DevelopmentReceiver::get()),
            0
        );
    });

    Kusama::execute_with(|| {
        // First we ensure the treasury has enough balance to transfer
        assert_ok!(kusama_runtime::Balances::force_set_balance(
            kusama_runtime::RuntimeOrigin::root(),
            kusama_treasury_address.clone().into(),
            transfer_amount.saturating_mul(10)
        ));

        let call = Box::new(kusama_runtime::RuntimeCall::XcmPallet(
            pallet_xcm::Call::<kusama_runtime::Runtime>::limited_reserve_transfer_assets {
                dest: Box::new(Parachain(PARA_ID_DEVELOPMENT).into()),
                beneficiary: Box::new(
                    Junction::AccountId32 {
                        network: Some(NetworkId::Kusama),
                        id: DevelopmentReceiver::get().into(),
                    }
                    .into(),
                ),
                assets: Box::new((Here, transfer_amount).into()),
                fee_asset_item: 0,
                weight_limit: WeightLimit::Unlimited,
            },
        ));

        assert_ok!(kusama_runtime::Utility::dispatch_as(
            kusama_runtime::RuntimeOrigin::root(),
            bx!(kusama_runtime::OriginCaller::system(RawOrigin::Signed(
                kusama_treasury_address
            ))),
            call,
        ));
    });

    Development::execute_with(|| {
        let para_receiver_balance_after =
            OrmlTokens::free_balance(CurrencyId::KSM, &DevelopmentReceiver::get());
        assert!(para_receiver_balance_after > 0);
    });
}

#[test]
fn transfer_ksm_to_relay_chain() {
    let transfer_amount: Balance = ksm_amount(10);
    let _kusama_receiver_balance_before = Kusama::account_data_of(KusamaReceiver::get()).free;
    Kusama::execute_with(|| {
        assert_ok!(kusama_runtime::XcmPallet::limited_reserve_transfer_assets(
            kusama_runtime::RuntimeOrigin::signed(KusamaSender::get()),
            Box::new(Parachain(PARA_ID_DEVELOPMENT).into()),
            Box::new(
                Junction::AccountId32 {
                    network: Some(NetworkId::Kusama),
                    id: DevelopmentSender::get().into(),
                }
                .into()
            ),
            Box::new((Here, transfer_amount.saturating_mul(5)).into()),
            0,
            WeightLimit::Unlimited,
        ));
    });

    Development::execute_with(|| {
        assert_ok!(XTokens::transfer(
            imbue_kusama_runtime::RuntimeOrigin::signed(DevelopmentSender::get()),
            CurrencyId::KSM,
            transfer_amount,
            Box::new(
                MultiLocation::new(
                    1,
                    X1(Junction::AccountId32 {
                        id: KusamaReceiver::get().into(),
                        network: Some(NetworkId::Kusama),
                    })
                )
                .into()
            ),
            WeightLimit::Unlimited
        ));
    });
    let _kusama_receiver_balance_after = Kusama::account_data_of(KusamaReceiver::get()).free;
    #[cfg(not(feature = "runtime-benchmarks"))]
    assert!(_kusama_receiver_balance_after > _kusama_receiver_balance_before);
}

#[test]
fn test_xcm_refund_handler_to_kusama() {
    let treasury_origin = TreasuryOrigin::Kusama;
    let kusama_treasury_address =
        <R as pallet_proposals::Config>::RefundHandler::get_treasury_account_id(treasury_origin)
            .unwrap();
    let _kusama_treasury_balance_before =
        Kusama::account_data_of(kusama_treasury_address.clone()).free;
    let transfer_amount: Balance = ksm_amount(10);
    Development::execute_with(|| {
        let ksm_balance = OrmlTokens::free_balance(CurrencyId::KSM, &DevelopmentReceiver::get());
        assert_eq!(ksm_balance, 0);
    });

    Kusama::execute_with(|| {
        assert_ok!(kusama_runtime::XcmPallet::limited_reserve_transfer_assets(
            kusama_runtime::RuntimeOrigin::signed(KusamaSender::get()),
            Box::new(Parachain(PARA_ID_DEVELOPMENT).into()),
            Box::new(
                Junction::AccountId32 {
                    network: Some(NetworkId::Kusama),
                    id: DevelopmentReceiver::get().into(),
                }
                .into()
            ),
            Box::new((Here, transfer_amount).into()),
            0,
            WeightLimit::Unlimited,
        ));
    });
    Development::execute_with(|| {
        let ksm_balance = OrmlTokens::free_balance(CurrencyId::KSM, &DevelopmentReceiver::get());
        assert!(ksm_balance > 0);
        assert_ok!(
            <R as pallet_proposals::Config>::RefundHandler::send_refund_message_to_treasury(
                DevelopmentReceiver::get(),
                ksm_balance,
                CurrencyId::KSM,
                FundingType::Grant(TreasuryOrigin::Kusama)
            )
        );
    });

    let _kusama_treasury_balance_after = Kusama::account_data_of(kusama_treasury_address).free;
    #[cfg(not(feature = "runtime-benchmarks"))]
    assert!(_kusama_treasury_balance_after > _kusama_treasury_balance_before)
}

#[test]
fn transfer_ksm_from_sibling() {
    let transfer_amount = native_amount(1);
    Development::execute_with(|| {
        let ksm_balance = OrmlTokens::free_balance(CurrencyId::KSM, &SiblingReceiver::get());
        assert_eq!(ksm_balance, 0);
    });
    Sibling::execute_with(|| {
        assert_ok!(OrmlTokens::deposit(
            CurrencyId::KSM,
            &DevelopmentSender::get(),
            transfer_amount.saturating_mul(2)
        ));

        assert_ok!(XTokens::transfer(
            RuntimeOrigin::signed(DevelopmentSender::get()),
            CurrencyId::KSM,
            transfer_amount,
            Box::new(
                MultiLocation::new(
                    1,
                    X2(
                        Parachain(PARA_ID_DEVELOPMENT),
                        Junction::AccountId32 {
                            network: Some(NetworkId::Kusama),
                            id: SiblingReceiver::get().into(),
                        }
                    )
                )
                .into()
            ),
            WeightLimit::Unlimited
        ));
    });

    // Necessary to make sure messages are processed. Feels like a bug in the emulator. Might
    // be fixed in next version.
    Kusama::execute_with(|| {});

    #[cfg(not(feature = "runtime-benchmarks"))]
    Development::execute_with(|| {
        let ksm_balance = OrmlTokens::free_balance(CurrencyId::KSM, &SiblingReceiver::get());
        assert!(ksm_balance > 0);
    });
}

#[test]
fn transfer_from_relay_chain() {
    let transfer_amount: Balance = ksm_amount(1);
    Development::execute_with(|| {
        assert_eq!(
            OrmlTokens::free_balance(CurrencyId::KSM, &DevelopmentReceiver::get()),
            0
        );
    });

    Kusama::execute_with(|| {
        assert_ok!(kusama_runtime::XcmPallet::limited_reserve_transfer_assets(
            kusama_runtime::RuntimeOrigin::signed(KusamaSender::get()),
            Box::new(Parachain(PARA_ID_DEVELOPMENT).into()),
            Box::new(
                Junction::AccountId32 {
                    network: Some(NetworkId::Kusama),
                    id: DevelopmentReceiver::get().into(),
                }
                .into()
            ),
            Box::new((Here, transfer_amount).into()),
            0,
            WeightLimit::Unlimited,
        ));
    });

    Development::execute_with(|| {
        let para_receiver_balance_after =
            OrmlTokens::free_balance(CurrencyId::KSM, &DevelopmentReceiver::get());
        assert!(para_receiver_balance_after > 0);
    });
}

#[test]
fn transfer_native_to_sibling() {
    Development::execute_with(|| {
        assert_ok!(imbue_kusama_runtime::PolkadotXcm::force_xcm_version(
            RuntimeOrigin::root(),
            Box::new(MultiLocation::new(1, X1(Parachain(PARA_ID_SIBLING)),)),
            SAFE_XCM_VERSION
        ));
    });
    let transfer_amount: Balance = native_amount(10);
    let sibling_balance_before: Balance = Sibling::account_data_of(SiblingReceiver::get()).free;
    Development::execute_with(|| {
        assert_ok!(XTokens::transfer(
            imbue_kusama_runtime::RuntimeOrigin::signed(DevelopmentSender::get()),
            CurrencyId::Native,
            transfer_amount,
            Box::new(
                MultiLocation::new(
                    1,
                    X2(
                        Parachain(PARA_ID_SIBLING),
                        Junction::AccountId32 {
                            network: Some(NetworkId::Kusama),
                            id: SiblingReceiver::get().into(),
                        }
                    )
                )
                .into()
            ),
            WeightLimit::Limited(4_000_000_000.into())
        ));
    });

    let sibling_balance_after = Sibling::account_data_of(SiblingReceiver::get()).free;
    assert!(sibling_balance_after > sibling_balance_before);
}

#[test]
fn transfer_mgx_from_sibling() {
    // TestNet::reset();
    let transfer_amount = mgx_amount(1_000_000_000);
    Sibling::execute_with(|| {
        let mgx_balance = OrmlTokens::free_balance(CurrencyId::MGX, &SiblingReceiver::get());
        assert_eq!(mgx_balance, 0);
    });
    Sibling::execute_with(|| {
        assert_ok!(OrmlTokens::deposit(
            CurrencyId::MGX,
            &DevelopmentSender::get(),
            transfer_amount.saturating_mul(10)
        ));
        assert_ok!(XTokens::transfer(
            RuntimeOrigin::signed(DevelopmentSender::get()),
            CurrencyId::MGX,
            transfer_amount,
            Box::new(
                MultiLocation::new(
                    1,
                    X2(
                        Parachain(PARA_ID_DEVELOPMENT),
                        Junction::AccountId32 {
                            network: Some(NetworkId::Kusama),
                            id: SiblingReceiver::get().into(),
                        }
                    )
                )
                .into()
            ),
            WeightLimit::Unlimited
        ));
    });
    Development::execute_with(|| {
        let mgx_balance = OrmlTokens::free_balance(CurrencyId::MGX, &SiblingReceiver::get());
        assert!(mgx_balance > 0);
    });
}

#[test]
fn currency_id_convert_imbu() {
    use imbue_kusama_runtime::CurrencyIdConvert;
    use sp_runtime::traits::Convert as C2;

    let imbu_location: MultiLocation = MultiLocation::new(
        1,
        X2(
            Parachain(parachains::kusama::imbue::ID),
            general_key(parachains::kusama::imbue::IMBU_KEY),
        ),
    );

    assert_eq!(
        CurrencyIdConvert::convert(imbu_location),
        Some(CurrencyId::Native),
    );

    let imbu_location_2: MultiLocation =
        MultiLocation::new(0, X1(general_key(parachains::kusama::imbue::IMBU_KEY)));

    assert_eq!(
        CurrencyIdConvert::convert(imbu_location_2),
        Some(CurrencyId::Native),
    );
}

// The fee associated with transferring Native tokens
// fn native_fee() -> Balance {
//     let (_asset, fee, _) = CanonicalImbuePerSecond::get();
//     // NOTE: it is possible that in different machines this value may differ. We shall see.
//     fee.div_euclid(10_000) * 8
// }

// The fee associated with transferring AUSD tokens
// fn ausd_fee() -> Balance {
//     let (_asset, fee, _) = AUsdPerSecond::get();
//     // NOTE: it is possible that in different machines this value may differ. We shall see.
//     fee.div_euclid(10_000) * 8
// }
//
// // The fee associated with transferring AUSD tokens
// fn kar_fee() -> Balance {
//     let (_asset, fee, _) = KarPerSecond::get();
//     // NOTE: it is possible that in different machines this value may differ. We shall see.
//     fee.div_euclid(10_000) * 8
// }
//
//
