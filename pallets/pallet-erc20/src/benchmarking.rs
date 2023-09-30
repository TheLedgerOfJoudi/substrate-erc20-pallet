//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as Token;
use frame_benchmarking::{account, benchmarks};
use frame_system::RawOrigin;

benchmarks! {
  init {
	  let src_account: T::AccountId = account("acc1", 0,0);
	  let total_supply: u32 = 1000;
  }:_(RawOrigin::Signed(src_account.clone()),total_supply)
  verify {
	  assert_eq!(Token::<T>::total_supply(), Some(total_supply));
	  assert_eq!(Token::<T>::balances(src_account.clone()), total_supply);
	  assert_eq!(Token::<T>::owner(), Some(src_account) );
  }

  transfer {
	  let src_account: T::AccountId = account("acc_1", 0, 0);
	  let dest_account: T::AccountId = account("acc_2", 1, 0);
	  let total_supply: u32 = 1000;
	  let _ = Token::<T>::init(RawOrigin::Signed(src_account.clone()).into(),total_supply);
	  let num_of_tokens = 100;
  }: _(RawOrigin::Signed(src_account.clone()), dest_account.clone(),  num_of_tokens)
  verify{
	  assert_eq!(Token::<T>::balances(src_account.clone()), total_supply - num_of_tokens);
	  assert_eq!(Token::<T>::balances(dest_account), num_of_tokens);
	  assert_eq!(Token::<T>::total_supply(), Some(total_supply));
  }

  approve {
	  let src_account: T::AccountId = account("acc_1", 0, 0);
	  let dest_account: T::AccountId = account("acc_2", 1, 0);
	  let total_supply: u32 = 1000;
	  let _ = Token::<T>::init(RawOrigin::Signed(src_account.clone()).into(),total_supply);
	  let num_of_tokens = 100;
  }: _(RawOrigin::Signed(src_account.clone()), dest_account.clone(),  num_of_tokens)
  verify{
	  assert_eq!(Token::<T>::allowances(src_account, dest_account), num_of_tokens);
  }

  transfer_from{
	  let src_account: T::AccountId = account("acc_1", 0, 0);
	  let approved_account: T::AccountId = account("acc_2", 1, 0);
	  let dest_account: T::AccountId = account("acc_3", 2, 0);
	  let total_supply = 1000;
	  let _ = Token::<T>::init(RawOrigin::Signed(src_account.clone()).into(),total_supply);
	  let num_of_approved_tokens = 100;
	  let num_of_spent_tokens = 50;
	  let _ = Token::<T>::approve(RawOrigin::Signed(src_account.clone()).into(), approved_account.clone(),  num_of_approved_tokens);
  }: _(RawOrigin::Signed(approved_account.clone()),src_account.clone(), dest_account.clone(), num_of_spent_tokens)
  verify{
	  assert_eq!(Token::<T>::allowances(src_account.clone(), approved_account.clone()), num_of_approved_tokens - num_of_spent_tokens);
	  assert_eq!(Token::<T>::balances(src_account.clone()), total_supply - num_of_spent_tokens);
	  assert_eq!(Token::<T>::balances(dest_account.clone()), num_of_spent_tokens);
  }

  mint {
	  let src_account: T::AccountId = account("acc_1", 0, 0);
	  let dest_account: T::AccountId = account("acc_2", 1, 0);
	  let initial_total_supply: u32 = 1000;
	  let _ = Token::<T>::init(RawOrigin::Signed(src_account.clone()).into(),initial_total_supply);
	  let num_of_tokens = 100;
  }: _(RawOrigin::Signed(src_account.clone()), dest_account.clone(),  num_of_tokens)
  verify{
	  assert_eq!(Token::<T>::balances(dest_account), num_of_tokens);
	  assert_eq!(Token::<T>::total_supply(), Some(initial_total_supply + num_of_tokens));
  }

  burn {
	  let src_account: T::AccountId = account("acc_1", 0, 0);
	  let dest_account: T::AccountId = account("acc_2", 1, 0);
	  let initial_total_supply: u32 = 1000;
	  let _ = Token::<T>::init(RawOrigin::Signed(src_account.clone()).into(),initial_total_supply);
	  let num_of_minted_tokens = 100;
	  let _ = Token::<T>::mint(RawOrigin::Signed(src_account.clone()).into(),dest_account.clone(), num_of_minted_tokens);
	  let num_of_burnt_tokens = 50;
  }: _(RawOrigin::Signed(src_account.clone()), dest_account.clone(),  num_of_burnt_tokens)
  verify{
	  assert_eq!(Token::<T>::balances(dest_account), num_of_minted_tokens - num_of_burnt_tokens);
	  assert_eq!(Token::<T>::total_supply(), Some(initial_total_supply + num_of_minted_tokens - num_of_burnt_tokens));
  }

}
