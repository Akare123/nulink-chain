use super::*;
use crate::{Error, mock::*};
use frame_support::{assert_ok, assert_noop};

#[test]
fn it_works_for_default_value() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		// assert_ok!(NuLinkProxy::do_something(Origin::signed(1), 42));
		// Read pallet storage and assert an expected result.
		// assert_eq!(NuLinkProxy::something(), Some(42));
	});
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		// Ensure the expected error is thrown when no value is present.
		// assert_noop!(
		// 	NuLinkProxy::cause_error(Origin::signed(1)),
		// 	Error::<Test>::NoneValue
		// );
	});
}

#[test]
fn it_works_for_set_watcher() {
	new_test_ext().execute_with(|| {
		assert_ok!(NuLinkProxy::set_watcher(Origin::signed(1)));
		assert_ok!(NuLinkProxy::set_watcher(Origin::signed(2)));
		assert_ok!(NuLinkProxy::set_watcher(Origin::signed(3)));
		assert_ok!(NuLinkProxy::set_watcher(Origin::signed(4)));
		assert_noop!(NuLinkProxy::set_watcher(Origin::signed(2)),Error::<Test>::AlreadyExist);
		assert_noop!(NuLinkProxy::set_watcher(Origin::signed(4)),Error::<Test>::AlreadyExist);

		assert_eq!(NuLinkProxy::exist_watcher(1),true);
		assert_eq!(NuLinkProxy::exist_watcher(3),true);
		assert_eq!(NuLinkProxy::exist_watcher(5),false);
		assert_eq!(NuLinkProxy::exist_watcher(6),false);
	});
}

#[test]
fn it_works_for_coinbase_to_keys() {
	new_test_ext().execute_with(|| {
		// keep the stakers
		let staker1 = make_stake_infos(1,100,1);
		let staker2 = make_stake_infos(2,200,2);
		let staker3 = make_stake_infos(3,300,3);
		let stakers1 = vec![staker1.clone(),staker2.clone(),staker3.clone()];
		assert_ok!(NuLinkProxy::update_stakers(stakers1));
		let accounts = vec![1 as u64,2,3];
		let h = NuLinkProxy::coinbase_to_staker_key(accounts);
		assert_eq!(h.len(),3);
		println!("{:?}", h);
		assert_eq!(h[2],NuLinkProxy::calc_staker_hash(staker1.clone()));
		assert_eq!(h[1],NuLinkProxy::calc_staker_hash(staker2.clone()));
		assert_eq!(h[0],NuLinkProxy::calc_staker_hash(staker3.clone()));
	});
}

#[test]
fn it_works_for_calc_staker_hash() {
	new_test_ext().execute_with(|| {
		let staker0 = make_stake_infos(1,100,1);
		let staker1 = make_stake_infos(1,100,1);
		let staker2 = make_stake_infos(1,200,1);
		let staker3 = make_stake_infos(1,200,2);
		let staker4 = make_stake_infos(1,200,2);
		assert_eq!(NuLinkProxy::calc_staker_hash(staker0.clone()),NuLinkProxy::calc_staker_hash(staker1.clone()));
		// lock_balance has no hash field
		assert_eq!(NuLinkProxy::calc_staker_hash(staker0.clone()),NuLinkProxy::calc_staker_hash(staker2.clone()));
		assert_ne!(NuLinkProxy::calc_staker_hash(staker0.clone()),NuLinkProxy::calc_staker_hash(staker4.clone()));
		assert_ne!(NuLinkProxy::calc_staker_hash(staker0.clone()),NuLinkProxy::calc_staker_hash(staker3.clone()));
	});
}

#[test]
fn it_works_for_update_staker() {
	new_test_ext().execute_with(|| {
		let staker1 = make_stake_infos(1,100,1);
		let staker2 = make_stake_infos(2,200,2);
		let staker3 = make_stake_infos(3,300,3);
		let stakers1 = vec![staker1.clone(),staker2.clone(),staker3.clone()];
		assert_ok!(NuLinkProxy::update_stakers(stakers1));
		assert_eq!(NuLinkProxy::get_staker_count(),3);
		assert_eq!(NuLinkProxy::get_total_staking(),600);
		assert_eq!(Stakers::<Test>::get(NuLinkProxy::calc_staker_hash(staker1.clone())).iswork,true);
		// update the stakers
		let staker4 = make_stake_infos(1,100,1);
		let staker5 = make_stake_infos(5,500,1);
		let staker6 = make_stake_infos(6,600,1);
		let stakers2 = vec![staker4.clone(),staker5.clone(),staker6.clone()];
		assert_ok!(NuLinkProxy::update_stakers(stakers2));
		// the staker1 same as staker4,they equal hash.
		assert_eq!(NuLinkProxy::get_staker_count(),5);
		assert_eq!(NuLinkProxy::get_total_staking(),1200);
		// staker1 still work in the next epoch
		assert_eq!(Stakers::<Test>::get(NuLinkProxy::calc_staker_hash(staker1.clone())).iswork,true);
		assert_eq!(Stakers::<Test>::get(NuLinkProxy::calc_staker_hash(staker2.clone())).iswork,false);
		assert_eq!(Stakers::<Test>::get(NuLinkProxy::calc_staker_hash(staker3.clone())).iswork,false);
	});
}

#[test]
fn it_works_for_mint_in_epoch() {
	new_test_ext().execute_with(|| {
		let staker1 = make_stake_infos(1,100,1);
		let staker2 = make_stake_infos(2,200,2);
		let staker3 = make_stake_infos(3,300,3);
		let stakers1 = vec![staker1.clone(),staker2.clone(),staker3.clone()];
		assert_ok!(NuLinkProxy::update_stakers(stakers1));
		assert_eq!(NuLinkProxy::get_staker_reward_by_coinbase(staker1.coinbase.clone()),0);
		assert_eq!(NuLinkProxy::get_staker_reward_by_coinbase(staker2.coinbase.clone()),0);
		assert_eq!(NuLinkProxy::get_staker_reward_by_coinbase(staker3.coinbase.clone()),0);
		assert_ok!(NuLinkProxy::mint_by_staker(100));
		let allStaking = NuLinkProxy::get_total_staking();
		let v3 =  staker3.lockedBalance * 100 / allStaking;
		let v2 = staker2.lockedBalance * 100 / allStaking;
		let v1 = 100 - v2 -v3;
		assert_eq!(v1,NuLinkProxy::get_staker_reward_by_coinbase(staker1.coinbase));
		assert_eq!(v2,NuLinkProxy::get_staker_reward_by_coinbase(staker2.coinbase));
		assert_eq!(v3,NuLinkProxy::get_staker_reward_by_coinbase(staker3.coinbase));
		// mint again
		assert_ok!(NuLinkProxy::mint_by_staker(200));
		let vv3 = staker3.lockedBalance * 200 / allStaking;
		let vv2 = staker2.lockedBalance * 200 / allStaking;
		let vv1 = 200 - vv2 - vv3;
		assert_eq!(v1+vv1,NuLinkProxy::get_staker_reward_by_coinbase(staker1.coinbase));
		assert_eq!(v2+vv2,NuLinkProxy::get_staker_reward_by_coinbase(staker2.coinbase));
		assert_eq!(v3+vv3,NuLinkProxy::get_staker_reward_by_coinbase(staker3.coinbase));
	});
}
#[test]
fn it_works_for_assigned_by_policy_reward() {
	new_test_ext().execute_with(|| {
		let allAmount :u64 = 100;
		let ids = vec![1 as u64,2,3];
		assert_ok!(NuLinkProxy::assigned_by_policy_reward(ids.clone(),allAmount));
		let unit = allAmount / ids.len() as u64;
		assert_eq!(unit,NuLinkProxy::get_staker_reward_by_coinbase(1));
		assert_eq!(unit,NuLinkProxy::get_staker_reward_by_coinbase(2));
		assert_eq!(unit,NuLinkProxy::get_staker_reward_by_coinbase(3));
	});
}

#[test]
fn it_works_for_reward_by_user_policy() {
	new_test_ext().execute_with(|| {
		let staker1 = make_stake_infos(1,100,1);
		let staker2 = make_stake_infos(2,200,2);
		let staker3 = make_stake_infos(3,300,3);
		let stakers1 = vec![staker1.clone(),staker2.clone(),staker3.clone()];
		assert_ok!(NuLinkProxy::update_stakers(stakers1));
		frame_system::Pallet::<Test>::set_block_number(10);
		// create the policy by owner
		let value = 100;
		let policyid = 1111;
		let stakers0 = vec![1,2];
		// check the owner asset
		assert_eq!(Balances::free_balance(OWNER),1000);
		create_policy(OWNER.clone(),value,50,policyid,stakers0.clone());
		assert_eq!(PolicyReserve::<Test>::contains_key(policyid),true);
		assert_eq!(Balances::free_balance(OWNER),1000-value);
		// set the epoch
		let epoch = 20;
		frame_system::Pallet::<Test>::set_block_number(epoch);
		let num = frame_system::Pallet::<Test>::block_number();
		assert_ok!(NuLinkProxy::reward_in_epoch(num));
		// check the result

	});
}

#[test]
fn it_works_for_set_get_from_vault() {
	new_test_ext().execute_with(|| {

	});
}
#[test]
fn it_works_for_claim_reward() {
	new_test_ext().execute_with(|| {

	});
}