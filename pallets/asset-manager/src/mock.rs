//
// The pallet-tx-pause pallet is forked from Acala's transaction-pause module https://github.com/AcalaNetwork/Acala/tree/master/modules/transaction-pause
// The original license is the following - SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

//! Mock runtime for asset-manager

use crate as pallet_asset_manager;
use crate::mock::sp_api_hidden_includes_construct_runtime::hidden_include::traits::GenesisBuild;
use common_primitives::{
    assets::{
        AssetConfig, AssetIdType, AssetLocation, AssetRegistry, AssetRegistryMetadata,
        AssetStorageMetadata, BalanceType, LocationType, NativeAndNonNative,
    },
    constants::{ASSET_MANAGER_PALLET_ID, ASSET_STRING_LIMIT},
    types::{AccountId, Balance, BlockNumber, CommonAssetId, Header},
};
use frame_support::{
    construct_runtime,
    pallet_prelude::DispatchResult,
    parameter_types,
    traits::{AsEnsureOriginWithArg, ConstU32},
    PalletId,
};
use frame_system as system;
use frame_system::{EnsureRoot, EnsureSigned};
use sp_core::H256;
use sp_runtime::traits::{BlakeTwo256, IdentityLookup};
use xcm::{
    prelude::{Parachain, X1},
    v1::MultiLocation,
    VersionedMultiLocation,
};
parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub const SS58Prefix: u8 = common_primitives::constants::WISP_SS58PREFIX;
}

impl system::Config for Runtime {
    type BaseCallFilter = frame_support::traits::Everything;
    type BlockWeights = ();
    type BlockLength = ();
    type DbWeight = ();
    type RuntimeOrigin = RuntimeOrigin;
    type RuntimeCall = RuntimeCall;
    type Index = u64;
    type BlockNumber = BlockNumber;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = AccountId;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type RuntimeEvent = RuntimeEvent;
    type BlockHashCount = ConstU32<250>;
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<Balance>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = SS58Prefix;
    type OnSetCode = ();
    type MaxConsumers = ConstU32<16>;
}

parameter_types! {
    pub const AssetDeposit: Balance = 0; // Does not really matter as this will be only called by root
    pub const AssetAccountDeposit: Balance = 0;
    pub const ApprovalDeposit: Balance = 0;
    pub const AssetsStringLimit: u32 = ASSET_STRING_LIMIT;
    pub const MetadataDepositBase: Balance = 0;
    pub const MetadataDepositPerByte: Balance = 0;
}

impl pallet_assets::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type Balance = Balance;
    type AssetId = CommonAssetId;
    type Currency = Balances;
    type ForceOrigin = EnsureRoot<AccountId>;
    type AssetDeposit = AssetDeposit;
    type AssetAccountDeposit = AssetAccountDeposit;
    type MetadataDepositBase = MetadataDepositBase;
    type MetadataDepositPerByte = MetadataDepositPerByte;
    type ApprovalDeposit = ApprovalDeposit;
    type StringLimit = AssetsStringLimit;
    type Freezer = ();
    type Extra = ();
    type WeightInfo = ();
    type RemoveItemsLimit = ConstU32<1000>;
    type AssetIdParameter = CommonAssetId;
    type CreateOrigin = AsEnsureOriginWithArg<EnsureSigned<AccountId>>;
    type CallbackHandle = ();
}

parameter_types! {
    pub ExistentialDeposit: Balance = 1;
    pub const MaxLocks: u32 = 50;
    pub const MaxReserves: u32 = 50;
}

impl pallet_balances::Config for Runtime {
    type MaxLocks = MaxLocks;
    type Balance = Balance;
    type RuntimeEvent = RuntimeEvent;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
    type MaxReserves = MaxReserves;
    type ReserveIdentifier = [u8; 8];
}

pub struct CommonAssetRegistry;
impl BalanceType for CommonAssetRegistry {
    type Balance = Balance;
}
impl AssetIdType for CommonAssetRegistry {
    type AssetId = CommonAssetId;
}
impl AssetRegistry for CommonAssetRegistry {
    type Metadata = AssetStorageMetadata;
    type Error = sp_runtime::DispatchError;

    fn create_asset(
        asset_id: CommonAssetId,
        metadata: AssetStorageMetadata,
        min_balance: Balance,
        is_sufficient: bool,
    ) -> DispatchResult {
        Assets::force_create(
            RuntimeOrigin::root(),
            asset_id,
            AssetManager::account_id(),
            is_sufficient,
            min_balance,
        )?;

        Assets::force_set_metadata(
            RuntimeOrigin::root(),
            asset_id,
            metadata.name,
            metadata.symbol,
            metadata.decimals,
            metadata.is_frozen,
        )
    }

    fn update_asset_metadata(
        asset_id: &CommonAssetId,
        metadata: AssetStorageMetadata,
    ) -> DispatchResult {
        Assets::force_set_metadata(
            RuntimeOrigin::root(),
            *asset_id,
            metadata.name,
            metadata.symbol,
            metadata.decimals,
            metadata.is_frozen,
        )
    }
}

parameter_types! {
    pub const StartNonNativeAssetId: CommonAssetId = 8;
    pub const NativeAssetId: CommonAssetId = 1;
    pub NativeAssetLocation: AssetLocation = AssetLocation(
        VersionedMultiLocation::V1(MultiLocation::new(1, X1(Parachain(1024)))));
    pub NativeAssetMetadata: AssetRegistryMetadata<Balance> = AssetRegistryMetadata {
        metadata: AssetStorageMetadata {
            name: b"wisp".to_vec(),
            symbol: b"WSP".to_vec(),
            decimals: 18,
            is_frozen: false,
        },
        min_balance: 1u128,
        is_sufficient: true,
    };
    pub const AssetManagerPalletId: PalletId = ASSET_MANAGER_PALLET_ID;
}

/// AssetConfig implementations for this runtime
#[derive(Clone, Eq, PartialEq)]
pub struct CommonAssetConfig;
impl LocationType for CommonAssetConfig {
    type Location = AssetLocation;
}
impl AssetIdType for CommonAssetConfig {
    type AssetId = CommonAssetId;
}
impl BalanceType for CommonAssetConfig {
    type Balance = Balance;
}
impl AssetConfig<Runtime> for CommonAssetConfig {
    type StartNonNativeAssetId = StartNonNativeAssetId;
    type NativeAssetId = NativeAssetId;
    type AssetRegistryMetadata = AssetRegistryMetadata<Balance>;
    type NativeAssetLocation = NativeAssetLocation;
    type NativeAssetMetadata = NativeAssetMetadata;
    type StorageMetadata = AssetStorageMetadata;
    type AssetRegistry = CommonAssetRegistry;
    type FungibleLedger = NativeAndNonNative<Runtime, CommonAssetConfig, Balances, Assets>;
}

impl pallet_asset_manager::Config for Runtime {
    type RuntimeEvent = RuntimeEvent;
    type AssetId = CommonAssetId;
    type Balance = Balance;
    type Location = AssetLocation;
    type AssetConfig = CommonAssetConfig;
    type ModifierOrigin = EnsureRoot<AccountId>;
    type PalletId = AssetManagerPalletId;
    type WeightInfo = ();
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Runtime>;
type Block = frame_system::mocking::MockBlock<Runtime>;

construct_runtime!(
    pub enum Runtime where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Storage, Config, Event<T>} = 0,
        Assets: pallet_assets::{Pallet, Storage, Event<T>} = 1,
        AssetManager: pallet_asset_manager::{Pallet, Call, Storage, Event<T>} = 2,
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>} = 3,
    }
);

pub const PALLET_BALANCES_INDEX: u8 = 3;

pub fn new_test_ext() -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Runtime>()
        .unwrap();
    pallet_asset_manager::GenesisConfig::<Runtime> {
        start_id: <CommonAssetConfig as AssetConfig<Runtime>>::StartNonNativeAssetId::get(),
    }
    .assimilate_storage(&mut t)
    .unwrap();
    sp_io::TestExternalities::new(t)
}

pub(crate) fn create_asset_metadata(
    name: &str,
    symbol: &str,
    decimals: u8,
    min_balance: u128,
    is_frozen: bool,
    is_sufficient: bool,
) -> AssetRegistryMetadata<Balance> {
    AssetRegistryMetadata {
        metadata: AssetStorageMetadata {
            name: name.as_bytes().to_vec(),
            symbol: symbol.as_bytes().to_vec(),
            decimals,
            is_frozen,
        },
        min_balance,
        is_sufficient,
    }
}
