use super::*;
use crate::mock::*;
use frame_support::{assert_err, assert_ok};
use frame_system::RawOrigin;

#[test]
fn init() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_eq!(Token::total_supply(), Some(1000));
		assert_eq!(Token::owner(), Some(source_account_id));
		assert_eq!(Token::balances(source_account_id), 1000_u32);
	})
}

#[test]
fn init_should_fail_when_called_twice() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_err!(
			Token::init(RawOrigin::Signed(source_account_id).into(), 1000),
			Error::<Test>::AlreadyInitialized
		);
	})
}

#[test]
fn transfer() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let dest_account_id = 2;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_ok!(Token::transfer(
			RawOrigin::Signed(source_account_id).into(),
			dest_account_id,
			100
		));
		assert_eq!(Token::balances(source_account_id), 900_u32);
		assert_eq!(Token::balances(dest_account_id), 100_u32);
	})
}

#[test]
fn transfer_should_fail_when_no_enough_balance() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let dest_account_id = 2;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_err!(
			Token::transfer(RawOrigin::Signed(source_account_id).into(), dest_account_id, 1100),
			Error::<Test>::BalanceNotEnough
		);
	})
}

#[test]
fn approve() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let dest_account_id = 2;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_ok!(Token::approve(
			RawOrigin::Signed(source_account_id).into(),
			dest_account_id,
			100
		));
		assert_eq!(Token::allowances(source_account_id, dest_account_id), 100);
	})
}

#[test]
fn approval_should_be_updated() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let dest_account_id = 2;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_ok!(Token::approve(
			RawOrigin::Signed(source_account_id).into(),
			dest_account_id,
			100
		));
		assert_eq!(Token::allowances(source_account_id, dest_account_id), 100);
		assert_ok!(Token::approve(
			RawOrigin::Signed(source_account_id).into(),
			dest_account_id,
			200
		));
		assert_eq!(Token::allowances(source_account_id, dest_account_id), 200);
	})
}

#[test]
fn approve_should_fail_when_no_enough_balance() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let dest_account_id = 2;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_err!(
			Token::approve(RawOrigin::Signed(source_account_id).into(), dest_account_id, 1100),
			Error::<Test>::BalanceNotEnough
		);
	})
}

#[test]
fn transfer_from() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let approved_account_id = 2;
		let dest_account_id = 3;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_ok!(Token::approve(
			RawOrigin::Signed(source_account_id).into(),
			approved_account_id,
			100
		));
		assert_ok!(Token::transfer_from(
			RawOrigin::Signed(approved_account_id).into(),
			source_account_id,
			dest_account_id,
			100
		));
		assert_eq!(Token::balances(source_account_id), 900_u32);
		assert_eq!(Token::balances(dest_account_id), 100_u32);
	})
}

#[test]
fn transfer_from_should_fail_when_no_enough_balance() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let approved_account_id = 2;
		let dest_account_id = 3;
		let external_account_id = 4;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_ok!(Token::approve(
			RawOrigin::Signed(source_account_id).into(),
			approved_account_id,
			1000
		));
		assert_ok!(Token::transfer(
			RawOrigin::Signed(source_account_id).into(),
			external_account_id,
			50
		));
		assert_err!(
			Token::transfer_from(
				RawOrigin::Signed(approved_account_id).into(),
				source_account_id,
				dest_account_id,
				1000
			),
			Error::<Test>::BalanceNotEnough
		);
	})
}

#[test]
fn transfer_from_should_fail_when_no_enough_allowance() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let approved_account_id = 2;
		let dest_account_id = 3;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_ok!(Token::approve(
			RawOrigin::Signed(source_account_id).into(),
			approved_account_id,
			100
		));
		assert_err!(
			Token::transfer_from(
				RawOrigin::Signed(approved_account_id).into(),
				source_account_id,
				dest_account_id,
				200
			),
			Error::<Test>::InsufficientAllowance
		);
	})
}

#[test]
fn mint() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let dest_account_id = 2;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_ok!(Token::mint(RawOrigin::Signed(source_account_id).into(), dest_account_id, 100));
		assert_eq!(Token::balances(dest_account_id), 100);
		assert_eq!(Token::total_supply(), Some(1100));
	})
}

#[test]
fn mint_should_fail_when_caller_is_not_owner() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let external_account_id = 2;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_err!(
			Token::mint(RawOrigin::Signed(external_account_id).into(), external_account_id, 100),
			Error::<Test>::NotAuthorized
		);
	})
}

#[test]
fn burn() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let dest_account_id = 2;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_ok!(Token::mint(RawOrigin::Signed(source_account_id).into(), dest_account_id, 200));
		assert_ok!(Token::burn(RawOrigin::Signed(source_account_id).into(), dest_account_id, 100));
		assert_eq!(Token::total_supply(), Some(1100));
		assert_eq!(Token::balances(dest_account_id), 100);
	})
}

#[test]
fn burn_should_fail_when_caller_is_not_owner() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let external_account_id = 2;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_err!(
			Token::burn(RawOrigin::Signed(external_account_id).into(), external_account_id, 100),
			Error::<Test>::NotAuthorized
		);
	})
}

#[test]
fn mint_should_fail_when_burn_amount_is_greater_than_balance() {
	new_test_ext().execute_with(|| {
		let source_account_id = 1;
		let dest_account_id = 2;
		assert_ok!(Token::init(RawOrigin::Signed(source_account_id).into(), 1000));
		assert_ok!(Token::mint(RawOrigin::Signed(source_account_id).into(), dest_account_id, 100));
		assert_err!(
			Token::burn(RawOrigin::Signed(source_account_id).into(), dest_account_id, 200),
			Error::<Test>::BalanceNotEnough
		);
	})
}
