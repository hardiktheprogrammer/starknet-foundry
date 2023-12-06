use crate::CheatnetState;
use blockifier::execution::execution_utils::felt_to_stark_felt;
use cairo_felt::Felt252;
use conversions::IntoConv;
use starknet::core::types::FieldElement;
use starknet_api::core::{calculate_contract_address, ClassHash, ContractAddress};
use starknet_api::hash::StarkFelt;
use starknet_api::transaction::Calldata;

use crate::constants as crate_constants;

impl CheatnetState {
    #[must_use]
    pub fn precalculate_address(
        &self,
        class_hash: &ClassHash,
        calldata: &[Felt252],
    ) -> ContractAddress {
        let salt = self.get_salt();

        let execute_calldata = create_execute_calldata(calldata);
        let deployer_address = FieldElement::from_hex_be(crate_constants::TEST_ADDRESS).unwrap();
        calculate_contract_address(
            salt,
            *class_hash,
            &execute_calldata,
            deployer_address.into_(),
        )
        .unwrap()
    }
}

fn create_execute_calldata(calldata: &[Felt252]) -> Calldata {
    let calldata: Vec<StarkFelt> = calldata.iter().map(felt_to_stark_felt).collect();
    Calldata(calldata.into())
}
