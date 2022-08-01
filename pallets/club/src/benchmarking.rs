#![cfg(feature = "runtime-benchmarks")]

use crate as ClubModule;
use crate::*;
use frame_benchmarking::{account, benchmarks, vec};
use frame_system::RawOrigin;

benchmarks! {

	add_new_club {
		let c in 1 .. 10000;
		let l in 1 .. 50;
	}: _(RawOrigin::Root, c, vec![0u8;l as usize])
	verify {
		assert!(ClubModule::Pallet::<T>::clubs(c).is_some());
	}

	add_member {
		let c in 1 .. 10000;
		let l in 1 .. 50;
		let member: T::AccountId = account("member", 0, 0);
		ClubModule::Pallet::<T>::add_new_club(RawOrigin::Root.into(), c, vec![0u8;l as usize])?;
	}: _(RawOrigin::Root, c, member.clone())
	verify {
		assert!(ClubModule::Pallet::<T>::membership(c,member));
	}

	remove_club {
		let c in 1 .. 10000;
		let l in 1 .. 50;
		ClubModule::Pallet::<T>::add_new_club(RawOrigin::Root.into(), c, vec![0u8;l as usize])?;
	}: _(RawOrigin::Root, c)
	verify {
		assert!(ClubModule::Pallet::<T>::clubs(c).is_none());
	}

	remove_member {
		let c in 1 .. 10000;
		let l in 1 .. 50;
		let member: T::AccountId = account("member", 0, 0);
		ClubModule::Pallet::<T>::add_new_club(RawOrigin::Root.into(), c, vec![0u8;l as usize])?;
		ClubModule::Pallet::<T>::add_member(RawOrigin::Root.into(), c, member.clone())?;
	}: _(RawOrigin::Root, c, member.clone())
	verify {
		assert!(!ClubModule::Pallet::<T>::membership(c,member));
	}

	impl_benchmark_test_suite!(Pallet, crate::mock::new_test_ext(), crate::mock::Test)
}
