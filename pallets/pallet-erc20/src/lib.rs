#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
use scale_info::prelude::vec::Vec;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use crate::weights::WeightInfo;

	use super::*;
	use frame_support::pallet_prelude::{DispatchResult, *};
	use frame_system::pallet_prelude::*;
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		/// Name of the token
		#[pallet::constant]
		type Name: Get<Vec<u8>>;

		/// Symbol of the token
		#[pallet::constant]
		type Symbol: Get<Vec<u8>>;

		/// Decimals
		#[pallet::constant]
		type Decimals: Get<u8>;
	}

	#[pallet::storage]
	#[pallet::getter(fn balances)]
	pub(super) type Balances<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AccountId, u32, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn allowances)]
	pub type Allowances<T: Config> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		Blake2_128Concat,
		T::AccountId,
		u32,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn total_supply)]
	pub type TotalSupply<T> = StorageValue<_, u32>;

	#[pallet::storage]
	#[pallet::getter(fn owner)]
	pub type Owner<T: Config> = StorageValue<_, T::AccountId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		TokenCreated { total_supply: u32, who: T::AccountId },
		TokenTransferred { from: T::AccountId, to: T::AccountId, num_of_tokens: u32 },
		TokenApproved { owner: T::AccountId, delegate: T::AccountId, num_of_tokens: u32 },
		TokenMinted { to: T::AccountId, num_of_tokens: u32 },
		TokenBurnt { from: T::AccountId, num_of_tokens: u32 },
	}

	#[pallet::error]
	pub enum Error<T> {
		BalanceNotEnough,
		InsufficientAllowance,
		NotAuthorized,
		AlreadyInitialized,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::init())]
		pub fn init(origin: OriginFor<T>, total_supply: u32) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(!<Owner<T>>::exists(), Error::<T>::AlreadyInitialized);
			<TotalSupply<T>>::put(total_supply);
			<Owner<T>>::put(&who);
			<Balances<T>>::insert(&who, total_supply);
			Self::deposit_event(Event::TokenCreated { total_supply, who });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			to: T::AccountId,
			num_of_tokens: u32,
		) -> DispatchResult {
			let from = ensure_signed(origin)?;
			let from_balance = <Balances<T>>::get(&from);
			ensure!(from_balance >= num_of_tokens, Error::<T>::BalanceNotEnough);
			let to_balance = <Balances<T>>::get(&to);

			<Balances<T>>::set(&from, from_balance - num_of_tokens);
			<Balances<T>>::set(&to, to_balance + num_of_tokens);
			Self::deposit_event(Event::TokenTransferred { from, to, num_of_tokens });
			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::approve())]
		pub fn approve(
			origin: OriginFor<T>,
			delegate: T::AccountId,
			num_of_tokens: u32,
		) -> DispatchResult {
			let owner = ensure_signed(origin)?;
			ensure!(<Balances<T>>::get(&owner) >= num_of_tokens, Error::<T>::BalanceNotEnough);
			<Allowances<T>>::insert(&owner, &delegate, num_of_tokens);

			Self::deposit_event(Event::TokenApproved { owner, delegate, num_of_tokens });
			Ok(())
		}

		#[pallet::call_index(3)]
		#[pallet::weight(T::WeightInfo::transfer_from())]
		pub fn transfer_from(
			origin: OriginFor<T>,
			from: T::AccountId,
			to: T::AccountId,
			num_of_tokens: u32,
		) -> DispatchResult {
			let spender = ensure_signed(origin)?;
			ensure!(<Balances<T>>::get(&from) >= num_of_tokens, Error::<T>::BalanceNotEnough);
			Allowances::<T>::try_mutate(&from, &spender, |allowance| -> Result<(), Error<T>> {
				ensure!(*allowance >= num_of_tokens, Error::<T>::InsufficientAllowance);
				*allowance = *allowance - num_of_tokens;
				Ok(())
			})?;

			Balances::<T>::try_mutate(&from, |balance| -> Result<(), Error<T>> {
				ensure!(*balance >= num_of_tokens, Error::<T>::BalanceNotEnough);
				*balance = *balance - num_of_tokens;

				Balances::<T>::mutate(&to, |balance| {
					*balance = *balance + num_of_tokens;
				});

				Ok(())
			})?;

			Self::deposit_event(Event::TokenApproved {
				owner: from.clone(),
				delegate: spender.clone(),
				num_of_tokens,
			});
			Self::deposit_event(Event::TokenTransferred { from, to, num_of_tokens });

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(T::WeightInfo::mint())]
		pub fn mint(origin: OriginFor<T>, to: T::AccountId, num_of_tokens: u32) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			ensure!(<Owner<T>>::get() == Some(caller), Error::<T>::NotAuthorized);
			TotalSupply::<T>::try_mutate(|total_supply| -> Result<(), Error<T>> {
				let cur_total_supply = total_supply.unwrap();
				*total_supply = Some(cur_total_supply + num_of_tokens);
				Ok(())
			})?;

			Balances::<T>::mutate(&to, |balance| {
				*balance = *balance + num_of_tokens;
			});
			Self::deposit_event(Event::TokenMinted { to, num_of_tokens });
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(T::WeightInfo::burn())]
		pub fn burn(
			origin: OriginFor<T>,
			from: T::AccountId,
			num_of_tokens: u32,
		) -> DispatchResult {
			let caller = ensure_signed(origin)?;
			ensure!(<Owner<T>>::get() == Some(caller), Error::<T>::NotAuthorized);
			Balances::<T>::try_mutate(&from, |balance| -> Result<(), Error<T>> {
				ensure!(*balance >= num_of_tokens, Error::<T>::BalanceNotEnough);
				*balance = *balance - num_of_tokens;
				Ok(())
			})?;

			TotalSupply::<T>::try_mutate(|total_supply| -> Result<(), Error<T>> {
				let cur_total_supply = total_supply.unwrap();
				*total_supply = Some(cur_total_supply - num_of_tokens);
				Ok(())
			})?;

			Self::deposit_event(Event::TokenBurnt { from, num_of_tokens });
			Ok(())
		}
	}
}
