// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
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

use crate::{Config, Pallet};
use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_runtime::{
	impl_tx_ext_default,
	traits::{transaction_extension::TransactionExtensionBase, TransactionExtension},
	transaction_validity::TransactionValidityError,
};

/// Ensure the transaction version registered in the transaction is the same as at present.
///
/// # Transaction Validity
///
/// The transaction with incorrect `transaction_version` are considered invalid. The validity
/// is not affected in any other way.
#[derive(Encode, Decode, Clone, Eq, PartialEq, TypeInfo)]
#[scale_info(skip_type_params(T))]
pub struct CheckTxVersion<T: Config + Send + Sync>(sp_std::marker::PhantomData<T>);

impl<T: Config + Send + Sync> sp_std::fmt::Debug for CheckTxVersion<T> {
	#[cfg(feature = "std")]
	fn fmt(&self, f: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		write!(f, "CheckTxVersion")
	}

	#[cfg(not(feature = "std"))]
	fn fmt(&self, _: &mut sp_std::fmt::Formatter) -> sp_std::fmt::Result {
		Ok(())
	}
}

impl<T: Config + Send + Sync> CheckTxVersion<T> {
	/// Create new `TransactionExtension` to check transaction version.
	pub fn new() -> Self {
		Self(sp_std::marker::PhantomData)
	}
}

impl<T: Config + Send + Sync> TransactionExtensionBase for CheckTxVersion<T> {
	const IDENTIFIER: &'static str = "CheckTxVersion";
	type Implicit = u32;
	fn implicit(&self) -> Result<Self::Implicit, TransactionValidityError> {
		Ok(<Pallet<T>>::runtime_version().transaction_version)
	}
	fn weight(&self) -> sp_weights::Weight {
		<T::ExtensionsWeightInfo as super::WeightInfo>::check_tx_version()
	}
}
impl<T: Config + Send + Sync, Context> TransactionExtension<<T as Config>::RuntimeCall, Context>
	for CheckTxVersion<T>
{
	type Val = ();
	type Pre = ();
	impl_tx_ext_default!(<T as Config>::RuntimeCall; Context; validate prepare);
}
