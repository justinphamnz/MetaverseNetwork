// This file is part of Bit.Country.

// Copyright (C) 2020-2021 Bit.Country.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![cfg_attr(not(feature = "std"), no_std)]

use blindbox_manager::{BlindBoxType, BlindBoxRewardItem};
use codec::{Decode, Encode};
use frame_support::{ensure, decl_storage};
use frame_support::{traits::{Currency, ExistenceRequirement, ReservableCurrency, LockableCurrency}};
use frame_system::{ensure_root, ensure_signed};
use primitives::{Balance, CountryId, CurrencyId, BlindBoxId};
use sp_runtime::{traits::{AccountIdConversion, One}, DispatchError, ModuleId, RuntimeDebug, DispatchResult};
use sp_runtime::SaturatedConversion;
use bc_country::*;
use sp_std::vec::Vec;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub use pallet::*;
use sp_core::H256;
use frame_support::traits::Randomness;
use sp_core::sp_std::convert::TryInto;

#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum BoxType {
    NormalBox,
    SpecialBox,
}

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::traits::Randomness;
    use sp_core::H256;
    use primitives::BlindBoxId;

    #[pallet::pallet]
    #[pallet::generate_store(trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    pub(super) type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    pub const DOLLARS: Balance = 1_000_000_000_000_000_000;

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
        type ModuleId: Get<ModuleId>;

        /// Something that provides randomness in the runtime.
        type Randomness: Randomness<Self::Hash>;

        // Maximum number of blindboxes allowed
        type MaxNumberOfBlindBox: Get<u32>;

        // Maximum number of KSM allowed
        type MaxKSMAllowed: Get<u32>;

        // Maximum number of NUUM boxes allowed
        type MaxNUUMBoxAllowed: Get<u32>;

        // Maximum number of collectable NFT allowed
        type MaxCollectableNFTAllowed: Get<u32>;

        // Maximum number of NFT hat allowed
        type MaxNFTHatAllowed: Get<u32>;

        // Maximum number of NFT jacket allowed
        type MaxNFTJacketAllowed: Get<u32>;

        // Maximum number of NFT pant allowed
        type MaxNFTPantAllowed: Get<u32>;

        // Maximum number of NFT shoes allowed
        type MaxNFTShoesAllowed: Get<u32>;

        type Currency: ReservableCurrency<Self::AccountId>
        + LockableCurrency<Self::AccountId, Moment=Self::BlockNumber>;

        #[pallet::constant]
        type TreasuryModuleId: Get<ModuleId>;
    }

    #[pallet::storage]
    #[pallet::getter(fn get_blindbox_rewards)]
    pub type BlindBoxRewards<T: Config> =
    StorageDoubleMap<_, Twox64Concat, BlindBoxId, Twox64Concat, T::AccountId, BlindBoxRewardItem<T::AccountId, BlindBoxId>, OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_blindboxes)]
    pub type BlindBoxes<T: Config> = StorageMap<_, Twox64Concat, BlindBoxId, (), OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_special_blindboxes)]
    pub type SpecialBlindBoxes<T: Config> = StorageMap<_, Twox64Concat, BlindBoxId, (), OptionQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_blindboxescreator)]
    pub type BlindBoxesCreator<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn all_blindboxes_count)]
    pub(super) type AvailableBlindBoxesCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn all_special_blindboxes_count)]
    pub(super) type SpecialAvailableBlindBoxesCount<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_available_ksm)]
    pub(super) type AvailableKSM<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_available_nuum)]
    pub(super) type AvailableNUUM<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_available_collectablenft)]
    pub(super) type AvailableCollectableNFT<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_available_nft_hat)]
    pub(super) type AvailableMainnetNFTHat<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_available_nft_jacket)]
    pub(super) type AvailableMainnetNFTJacket<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_available_nft_pant)]
    pub(super) type AvailableMainnetNFTPant<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_available_nft_shoes)]
    pub(super) type AvailableMainnetNFTShoes<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn is_init)]
    pub(super) type Init<T: Config> = StorageValue<_, bool, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn nonce)]
    pub(super) type Nonce<T: Config> = StorageValue<_, u32, ValueQuery>;

    #[pallet::storage]
    #[pallet::getter(fn get_blacklist)]
    pub type ReportedBlacklist<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, (), OptionQuery>;

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId")]
    pub enum Event<T: Config> {
        BlindBoxIdGenerated(Vec<u32>),
        BlindBoxOpened(T::AccountId, BlindBoxId, BlindBoxType, u32),
        BlindBoxGoodLuckNextTime(T::AccountId, BlindBoxId),
        BlacklistAdded(T::AccountId),
        BlacklistRemoved(T::AccountId),
    }

    #[pallet::error]
    pub enum Error<T> {
        //No permission
        NoPermission,
        // BlindBox does not exist
        BlindBoxDoesNotExist,
        // BlindBoxes still available, only allow to create once all the blindboxes have been used
        BlindBoxesStillAvailable,
        // Exceeds the maximum amount of KSM allowed
        ExceedsMaxKSMAllowed,
        // Exceeds the maximum amount of NUUM allowed
        ExceedsMaxNUUMAllowed,
        // Exceeds the maximum amount of collectable NFTs allowed
        ExceedsMaxCollectableNFTAllowed,
        // Exceeds the maximum amount of NFT hat allowed
        ExceedsMaxNFTHatAllowed,
        // Exceeds the maximum amount of NFT jacket allowed
        ExceedsMaxNFTJacketAllowed,
        // Exceeds the maximum amount of NFT pant allowed
        ExceedsMaxNFTPantAllowed,
        // Exceeds the maximum amount of NFT shoes allowed
        ExceedsMaxNFTShoesAllowed,
        // Blacklist entries already exist
        BlacklistAlreadyExist,
        // Blacklist entries is not exist
        BlacklistIsNotExist,
        // Account is on black list
        BlacklistReported,
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(100_000)]
        pub(super) fn set_available_ksm(origin: OriginFor<T>, available_ksm: u32) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            // Ensure the authorized caller can call this func
            ensure!(
                available_ksm <= T::MaxKSMAllowed::get(),
                Error::<T>::ExceedsMaxKSMAllowed
            );

            AvailableKSM::<T>::put(available_ksm); // 200000 = 20KSM

            Ok(().into())
        }

        #[pallet::weight(100_000)]
        pub(super) fn set_available_nuum(origin: OriginFor<T>, proposed_amount: u32) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            // Ensure the authorized caller can call this func
            ensure!(
                proposed_amount <= T::MaxNUUMBoxAllowed::get(),
                Error::<T>::ExceedsMaxNUUMAllowed
            );

            // Update AvailableNUUM with proposed_amount
            AvailableNUUM::<T>::put(proposed_amount);

            Ok(().into())
        }

        #[pallet::weight(100_000)]
        pub(super) fn set_available_collectable_nft(origin: OriginFor<T>, proposed_amount: u32) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            // Ensure the authorized caller can call this func
            ensure!(
                proposed_amount <= T::MaxCollectableNFTAllowed::get(),
                Error::<T>::ExceedsMaxCollectableNFTAllowed
            );

            // Update AvailableCollectableNFT with proposed_amount
            AvailableCollectableNFT::<T>::put(proposed_amount);

            Ok(().into())
        }

        #[pallet::weight(100_000)]
        pub(super) fn set_available_nft_hat(origin: OriginFor<T>, proposed_amount: u32) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            // Ensure the authorized caller can call this func
            ensure!(
                proposed_amount <= T::MaxNFTHatAllowed::get(),
                Error::<T>::ExceedsMaxNFTHatAllowed
            );

            // Update AvailableMainnetNFTHat with proposed_amount
            AvailableMainnetNFTHat::<T>::put(proposed_amount);

            Ok(().into())
        }

        #[pallet::weight(100_000)]
        pub(super) fn set_available_nft_jacket(origin: OriginFor<T>, proposed_amount: u32) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;
            // Ensure the authorized caller can call this func
            ensure!(
                proposed_amount <= T::MaxNFTJacketAllowed::get(),
                Error::<T>::ExceedsMaxNFTJacketAllowed
            );

            // Update AvailableMainnetNFTJacket with proposed_amount
            AvailableMainnetNFTJacket::<T>::put(proposed_amount);

            Ok(().into())
        }

        #[pallet::weight(100_000)]
        pub(super) fn set_available_nft_pant(origin: OriginFor<T>, proposed_amount: u32) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            // Ensure the authorized caller can call this func
            ensure!(
                proposed_amount <= T::MaxNFTPantAllowed::get(),
                Error::<T>::ExceedsMaxNFTPantAllowed
            );

            // Update AvailableMainnetNFTPant with proposed_amount
            AvailableMainnetNFTPant::<T>::put(proposed_amount);

            Ok(().into())
        }

        #[pallet::weight(100_000)]
        pub(super) fn set_available_nft_shoes(origin: OriginFor<T>, proposed_amount: u32) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            // Ensure the authorized caller can call this func
            ensure!(
                proposed_amount <= T::MaxNFTShoesAllowed::get(),
                Error::<T>::ExceedsMaxNFTShoesAllowed
            );

            // Update AvailableMainnetNFTShoes with proposed_amount
            AvailableMainnetNFTShoes::<T>::put(proposed_amount);

            Ok(().into())
        }

        #[pallet::weight(100_000)]
        pub(super) fn set_blindbox_caller(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            BlindBoxesCreator::<T>::put(account_id);

            Ok(().into())
        }

        #[pallet::weight(100_000_000)]
        pub(super) fn generate_blindbox_ids(origin: OriginFor<T>, number_blindboxes: u32) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;

            // Ensure the authorized caller can call this func
            ensure!(
                BlindBoxesCreator::<T>::get() == caller,
                Error::<T>::NoPermission
            );

            // Ensure caller can only generate blindboxes once all the available blindboxes have been used
            ensure!(
                AvailableBlindBoxesCount::<T>::get() == 0,
                Error::<T>::BlindBoxesStillAvailable
            );

            let mut blindbox_vec = Vec::new();

            // Generate random blindbox id and store
            let mut number_blindboxes_generated = 0;
            let mut i = 0;

            // Add safe check in case of infinite loop, running extra 10 loops to generate unique blindbox id
            while number_blindboxes_generated < number_blindboxes && i < T::MaxNumberOfBlindBox::get() {
                let mut blindbox_id = Self::generate_random_number(i);

                if !BlindBoxes::<T>::contains_key(blindbox_id) {
                    // Push to Vec and save to storage
                    blindbox_vec.push(blindbox_id);
                    BlindBoxes::<T>::insert(blindbox_id, ());

                    number_blindboxes_generated = number_blindboxes_generated.checked_add(One::one()).ok_or("Overflow")?;
                }

                i = i.checked_add(One::one()).ok_or("Overflow")?;
            }

            AvailableBlindBoxesCount::<T>::put(number_blindboxes_generated);

            Self::deposit_event(Event::BlindBoxIdGenerated(blindbox_vec));

            Ok(().into())
        }

        #[pallet::weight(100_000_000)]
        pub(super) fn generate_special_blindbox_ids(origin: OriginFor<T>, number_blindboxes: u32) -> DispatchResultWithPostInfo {
            let caller = ensure_signed(origin)?;

            // Ensure the authorized caller can call this func
            ensure!(
                BlindBoxesCreator::<T>::get() == caller,
                Error::<T>::NoPermission
            );

            let mut blindbox_vec = Vec::new();

            // Generate random blindbox id and store
            let mut number_blindboxes_generated = 0;
            let mut i = 0;

            // Add safe check in case of infinite loop, running extra 10 loops to generate unique blindbox id
            while number_blindboxes_generated < number_blindboxes {
                let mut blindbox_id = Self::generate_random_number(i);

                if !SpecialBlindBoxes::<T>::contains_key(blindbox_id) {
                    // Push to Vec and save to storage
                    blindbox_vec.push(blindbox_id);
                    SpecialBlindBoxes::<T>::insert(blindbox_id, ());

                    number_blindboxes_generated = number_blindboxes_generated.checked_add(One::one()).ok_or("Overflow")?;
                }

                i = i.checked_add(One::one()).ok_or("Overflow")?;
            }

            SpecialAvailableBlindBoxesCount::<T>::put(number_blindboxes_generated);

            Self::deposit_event(Event::BlindBoxIdGenerated(blindbox_vec));

            Ok(().into())
        }

        #[pallet::weight(90_000_000_000)]
        pub(super) fn open_blind_box(origin: OriginFor<T>, blindbox_id: BlindBoxId) -> DispatchResultWithPostInfo {
            let owner = ensure_signed(origin)?;

            // Ensure the specified blindbox id exist in storage
            ensure!(
                BlindBoxes::<T>::contains_key(blindbox_id) || SpecialBlindBoxes::<T>::contains_key(blindbox_id),
                Error::<T>::BlindBoxDoesNotExist
            );

            let open_box_fee = 1 * DOLLARS;
            let balance: BalanceOf<T> = TryInto::<BalanceOf<T>>::try_into(open_box_fee).unwrap_or_default();
            let treasury_module_id = T::TreasuryModuleId::get().into_account();

            <T as Config>::Currency::transfer(&owner, &treasury_module_id, balance, ExistenceRequirement::KeepAlive)?;

            ensure!(
                !ReportedBlacklist::<T>::contains_key(owner.clone()),
                Error::<T>::BlacklistReported
            );

            if BlindBoxes::<T>::contains_key(blindbox_id) {
                // Remove from Blind Boxes
                BlindBoxes::<T>::remove(blindbox_id);

                Self::handle_open_box_logic(blindbox_id, owner, BoxType::NormalBox);

                Ok(().into())
            } else if SpecialBlindBoxes::<T>::contains_key(blindbox_id) {
                // Remove from Special Blind Boxes
                SpecialBlindBoxes::<T>::remove(blindbox_id);

                Self::handle_open_box_logic(blindbox_id, owner, BoxType::SpecialBox);

                Ok(().into())
            } else {
                Ok(().into())
            }
        }

        #[pallet::weight(100_000_000)]
        pub(super) fn add_new_blacklist(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            ensure!(
                !ReportedBlacklist::<T>::contains_key(account_id.clone()),
                Error::<T>::BlacklistAlreadyExist
            );

            ReportedBlacklist::<T>::insert(account_id.clone(), ());

            Self::deposit_event(Event::BlacklistAdded(account_id));

            Ok(().into())
        }

        #[pallet::weight(100_000_000)]
        pub(super) fn remove_blacklist(origin: OriginFor<T>, account_id: T::AccountId) -> DispatchResultWithPostInfo {
            ensure_root(origin)?;

            ensure!(
                ReportedBlacklist::<T>::contains_key(account_id.clone()),
                Error::<T>::BlacklistIsNotExist
            );

            ReportedBlacklist::<T>::remove(account_id.clone());

            Self::deposit_event(Event::BlacklistRemoved(account_id));

            Ok(().into())
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<T::BlockNumber> for Pallet<T> {}
}

impl<T: Config> Pallet<T> {
    fn generate_random_number(seed: u32) -> u32 {
        let random_seed = T::Randomness::random(&("pallet-blindbox", seed).encode());

        let random_number = <u32>::decode(&mut random_seed.as_ref())
            .expect("secure hashes should always be bigger than u32; qed");
        random_number
    }

    fn save_blindbox_reward(owner: &T::AccountId, blindbox_id: BlindBoxId, blindbox_reward_item: BlindBoxRewardItem<T::AccountId, BlindBoxId>) -> Result<BlindBoxId, DispatchError> {
        // Add to BlindBoxRewards
        BlindBoxRewards::<T>::insert(blindbox_id, owner, blindbox_reward_item);

        Ok(blindbox_id)
    }

    fn check_and_deduct_rewards_availability(blindbox_type: BlindBoxType, distributed_amount: u32) -> (bool) {
        match blindbox_type {
            BlindBoxType::KSM => {
                // Deduct distribute amount from available KSM and update
                let available_amount = Self::get_available_ksm();
                if available_amount >= distributed_amount {
                    let new_available_amount = available_amount - distributed_amount;
                    AvailableKSM::<T>::put(new_available_amount);
                    return true;
                }
                return false;
            }
            BlindBoxType::NUUM => {
                let available_amount = Self::get_available_nuum();
                if available_amount >= distributed_amount {
                    let new_available_amount = available_amount - distributed_amount;
                    AvailableNUUM::<T>::put(new_available_amount);
                    return true;
                }
                return false;
            }
            BlindBoxType::CollectableNFT => {
                let available_amount = Self::get_available_collectablenft();
                if available_amount >= distributed_amount {
                    let new_available_amount = available_amount - distributed_amount;
                    AvailableCollectableNFT::<T>::put(new_available_amount);
                    return true;
                }
                return false;
            }
            BlindBoxType::MainnetNFTHat1 | BlindBoxType::MainnetNFTHat2 => {
                let available_amount = Self::get_available_nft_hat();
                if available_amount >= distributed_amount {
                    let new_available_amount = available_amount - distributed_amount;
                    AvailableMainnetNFTHat::<T>::put(new_available_amount);
                    return true;
                }
                return false;
            }
            BlindBoxType::MainnetNFTJacket1 | BlindBoxType::MainnetNFTJacket2 => {
                let available_amount = Self::get_available_nft_jacket();
                if available_amount >= distributed_amount {
                    let new_available_amount = available_amount - distributed_amount;
                    AvailableMainnetNFTJacket::<T>::put(new_available_amount);
                    return true;
                }
                return false;
            }
            BlindBoxType::MainnetNFTPants1 | BlindBoxType::MainnetNFTPants2 => {
                let available_amount = Self::get_available_nft_pant();
                if available_amount >= distributed_amount {
                    let new_available_amount = available_amount - distributed_amount;
                    AvailableMainnetNFTPant::<T>::put(new_available_amount);
                    return true;
                }
                return false;
            }
            BlindBoxType::MainnetNFTShoes1 | BlindBoxType::MainnetNFTShoes2 => {
                let available_amount = Self::get_available_nft_shoes();
                if available_amount >= distributed_amount {
                    let new_available_amount = available_amount - distributed_amount;
                    AvailableMainnetNFTShoes::<T>::put(new_available_amount);
                    return true;
                }
                return false;
            }
        }
    }

    fn check_winner(owner: &T::AccountId, blindbox_id: BlindBoxId, max_number: u32, random_number: u32) -> (bool, BlindBoxRewardItem<T::AccountId, BlindBoxId>) {
        let mut blindbox_reward_item = BlindBoxRewardItem {
            recipient: owner.clone(),
            amount: 0,
            blindbox_type: BlindBoxType::NUUM,
            blindBoxId: blindbox_id,
        };

        let mut is_winning = false;
        let max_nuum_amount: u32 = 20;
        let distribute_ksm_amount: u32 = 500; // 0.05 KSM

        if random_number % max_number == 0 {
            // 1/10000 chance of winning collectable NFT
            let available = Self::check_and_deduct_rewards_availability(BlindBoxType::CollectableNFT, 1);
            if available {
                blindbox_reward_item.blindbox_type = BlindBoxType::CollectableNFT;
                blindbox_reward_item.amount = 1;
                is_winning = true;
            }
        } else if random_number % 20 == 0 {
            // 5% chance of winning mainnet nft shoes
            let available = Self::check_and_deduct_rewards_availability(BlindBoxType::MainnetNFTShoes1, 1);
            if available {
                let rand = Self::generate_random_number(random_number);
                if rand % 2 == 0 {
                    blindbox_reward_item.blindbox_type = BlindBoxType::MainnetNFTShoes1;
                } else {
                    blindbox_reward_item.blindbox_type = BlindBoxType::MainnetNFTShoes2;
                }
                blindbox_reward_item.amount = 1;
                is_winning = true;
            }
        } else if random_number % 10 == 0 {
            // 10% chance of winning
            let rand = Self::generate_random_number(random_number);

            let reminder = rand % 3;
            if reminder == 0 {
                // 10% chance of winning KSM
                // If available KSM is less than the distribute amount, then stop
                let available = Self::check_and_deduct_rewards_availability(BlindBoxType::KSM, distribute_ksm_amount);
                if available {
                    blindbox_reward_item.amount = distribute_ksm_amount; // 500 = 0.05 KSM
                    blindbox_reward_item.blindbox_type = BlindBoxType::KSM;
                    is_winning = true;
                }
            } else if reminder == 1 {
                // 10% chance of winning wearable NFTs Jacket
                let available = Self::check_and_deduct_rewards_availability(BlindBoxType::MainnetNFTJacket1, 1);
                if available {
                    let new_rand = Self::generate_random_number(reminder);
                    if new_rand % 2 == 0 {
                        blindbox_reward_item.blindbox_type = BlindBoxType::MainnetNFTJacket1;
                    } else {
                        blindbox_reward_item.blindbox_type = BlindBoxType::MainnetNFTJacket2;
                    }
                    blindbox_reward_item.amount = 1;
                    is_winning = true;
                }
            } else if reminder == 2 {
                // 10% chance of winning wearable NFTs shoes
                let available = Self::check_and_deduct_rewards_availability(BlindBoxType::MainnetNFTPants1, 1);
                if available {
                    let new_rand = Self::generate_random_number(reminder);
                    if new_rand % 2 == 0 {
                        blindbox_reward_item.blindbox_type = BlindBoxType::MainnetNFTPants1;
                    } else {
                        blindbox_reward_item.blindbox_type = BlindBoxType::MainnetNFTPants2;
                    }
                    blindbox_reward_item.amount = 1;
                    is_winning = true;
                }
            }
        } else if random_number % 5 == 0 {
            // 20% chance of winning wearable NFTs hat
            let available = Self::check_and_deduct_rewards_availability(BlindBoxType::MainnetNFTHat1, 1);
            if available {
                let new_rand = Self::generate_random_number(random_number) % 2;
                if new_rand % 2 == 0 {
                    blindbox_reward_item.blindbox_type = BlindBoxType::MainnetNFTHat1;
                } else {
                    blindbox_reward_item.blindbox_type = BlindBoxType::MainnetNFTHat2;
                }
                blindbox_reward_item.amount = 1;
                is_winning = true;
            }
        } else if random_number % 4 == 0 {
            // 25% testnet nuum
            let max_nuum_amount_added_one = max_nuum_amount.saturating_add(One::one());
            let nuum_amount = Self::generate_random_number(random_number) % max_nuum_amount_added_one;
            let distributed_amount = nuum_amount.saturating_mul(10000);
            let available = Self::check_and_deduct_rewards_availability(BlindBoxType::NUUM, 1);
            if available {
                blindbox_reward_item.amount = distributed_amount; // 10000 = 1 NUUM
                blindbox_reward_item.blindbox_type = BlindBoxType::NUUM;
                is_winning = true;

                Self::transfer_nuum(&owner, nuum_amount);
            }
        }

        (is_winning, blindbox_reward_item)
    }

    fn transfer_nuum(owner: &T::AccountId, nuum_amount: u32) {
        let caller = BlindBoxesCreator::<T>::get();
        let nuum_amount_in_128: u128 = nuum_amount.saturated_into();
        let amount_in_balance: u128 = (nuum_amount_in_128 * DOLLARS);

        let balance: BalanceOf<T> = TryInto::<BalanceOf<T>>::try_into(amount_in_balance).unwrap_or_default();

        //Transfer balance from buy it now user to asset owner
        <T as Config>::Currency::transfer(&caller, &owner, balance, ExistenceRequirement::KeepAlive);
    }

    fn handle_open_box_logic(blindbox_id: BlindBoxId, owner: T::AccountId, box_type: BoxType) -> DispatchResult {
        match box_type {
            BoxType::NormalBox => {
                let available_blindbox_count = Self::all_blindboxes_count();

                let new_available_blindbox_count = available_blindbox_count.checked_sub(One::one()).ok_or("Overflow subtracting new count to available blindboxes")?;
                AvailableBlindBoxesCount::<T>::put(new_available_blindbox_count);

                let max_range: u32 = 10000;
                // Generate a random number between 1 and 100000
                let mut random_number = Self::generate_random_number(blindbox_id) % max_range.checked_add(One::one()).ok_or("Overflow")?;

                if random_number % 5 == 0 {
                    // 20% chance has no winning
                    Self::deposit_event(Event::<T>::BlindBoxGoodLuckNextTime(owner, blindbox_id.clone()));
                } else {
                    // 80% chance has winning, generate a new random number
                    random_number = Self::generate_random_number(random_number) % max_range.checked_add(One::one()).ok_or("Overflow")?;

                    let (is_winning, blindbox_reward_item) = Self::check_winner(&owner, blindbox_id, max_range, random_number);

                    if is_winning {
                        Self::save_blindbox_reward(&owner, blindbox_id, blindbox_reward_item.clone());
                        Self::deposit_event(Event::<T>::BlindBoxOpened(owner, blindbox_id.clone(), blindbox_reward_item.blindbox_type, blindbox_reward_item.amount));
                    } else {
                        Self::deposit_event(Event::<T>::BlindBoxGoodLuckNextTime(owner, blindbox_id.clone()));
                    }
                }

                Ok(())
            }
            BoxType::SpecialBox => {
                let available_blindbox_count = Self::all_special_blindboxes_count();

                let new_available_blindbox_count = available_blindbox_count.checked_sub(One::one()).ok_or("Overflow subtracting new count to available blindboxes")?;
                SpecialAvailableBlindBoxesCount::<T>::put(new_available_blindbox_count);

                let max_range: u32 = 10000;
                // Generate a random number between 1 and 100000
                let mut random_number = Self::generate_random_number(blindbox_id) % max_range.checked_add(One::one()).ok_or("Overflow")?;

                if random_number % 5 == 0 {
                    // 20% chance has no winning
                    Self::deposit_event(Event::<T>::BlindBoxGoodLuckNextTime(owner, blindbox_id.clone()));
                } else {
                    // 80% chance has winning, generate a new random number
                    random_number = Self::generate_random_number(random_number) % max_range.checked_add(One::one()).ok_or("Overflow")?;

                    let (is_winning, blindbox_reward_item) = Self::check_winner(&owner, blindbox_id, max_range, random_number);

                    if is_winning {
                        Self::save_blindbox_reward(&owner, blindbox_id, blindbox_reward_item.clone());
                        Self::deposit_event(Event::<T>::BlindBoxOpened(owner, blindbox_id.clone(), blindbox_reward_item.blindbox_type, blindbox_reward_item.amount));
                    } else {
                        Self::deposit_event(Event::<T>::BlindBoxGoodLuckNextTime(owner, blindbox_id.clone()));
                    }
                }

                Ok(())
            }
        }
    }
}