// use std::path::PathBuf;
// // Outlining aspects of the Casper test support crate to include.
// use casper_engine_test_support::{
//     ExecuteRequestBuilder,
//     InMemoryWasmTestBuilder,
//     DEFAULT_ACCOUNT_ADDR,
//     DEFAULT_RUN_GENESIS_REQUEST,
// };
// // Custom Casper types that will be used within this test.
// use casper_types::{
//     runtime_args,
//     ContractHash,
//     RuntimeArgs,
//     PublicKey,
//     SecretKey,
//     URef,
//     CLValue,
//     CLType::Any,
//     bytesrepr::ToBytes,
//     U512,
//     account::AccountHash,
//     Key,
//     ApiError
// };
// use common_lib::{
//     enums::price_oracle_contract::PriceType,
//     constants::{
//         PRICE_ORACLE_CONTRACT_NAME_WASM,
//         PRICE_ORACLE_CONTRACT_NAME,
//         KEY_PO_CONTRACT_HASH, ENDPOINT_PO_SET_PRICE, KEY_PO_PRICE, ENDPOINT_PO_ADD_AUTHORITY, ENDPOINT_PO_REMOVE_AUTHORITY,
//     }
// };
// use common_lib::constants::{KEY_PO_CHARS_COUNT_MID, KEY_PO_PRICE_MID, KEY_PO_PRICE_MORE};
//
// use crate::utils::{
//     fund_account,
//     deploy,
//     DeploySource,
//     query
// };
//
// struct PriceOracleContractContext {
//     builder: InMemoryWasmTestBuilder,
//     contract_hash: ContractHash,
//     alice_account: AccountHash,
//     bob_account: AccountHash,
//     path_buf: PathBuf
// }
//
// impl PriceOracleContractContext {
//
//     pub fn deploy() -> Self {
//         let alice_public_key: PublicKey =
//             PublicKey::from(&SecretKey::ed25519_from_bytes([1u8; 32]).unwrap());
//         let bob_public_key: PublicKey =
//             PublicKey::from(&SecretKey::ed25519_from_bytes([2u8; 32]).unwrap());
//
//         let alice_account = AccountHash::from(&alice_public_key);
//         let bob_account = AccountHash::from(&bob_public_key);
//
//         let mut builder = InMemoryWasmTestBuilder::default();
//         builder.run_genesis(&DEFAULT_RUN_GENESIS_REQUEST).commit();
//         builder
//             .exec(fund_account(&alice_account))
//             .expect_success()
//             .commit();
//         builder
//             .exec(fund_account(&bob_account))
//             .expect_success()
//             .commit();
//
//         let code = PathBuf::from(PRICE_ORACLE_CONTRACT_NAME_WASM);
//
//         deploy(
//             &mut builder,
//             &alice_account,
//             &DeploySource::Code(code.clone()),
//             runtime_args! {},
//             true,
//             None,
//         );
//
//         let contract_hash = query(
//             &builder,
//             Key::Account(alice_account),
//             &[KEY_PO_CONTRACT_HASH.to_string()],
//         );
//
//         Self {
//             builder,
//             contract_hash,
//             alice_account,
//             bob_account,
//             path_buf: code.clone()
//         }
//     }
//
//     pub fn set_fixed_price(
//         &mut self,
//         amount: U512,
//         is_success: bool
//     ) {
//         let args = runtime_args! {
//             "arg_price_type" => PriceType::Fixed,
//             "arg_price" => amount,
//             "arg_price_mid" => Vec::<u64>::new(),
//             "arg_chars_count_mid" => Vec::<u64>::new(),
//             "arg_price_more" => 0u64
//         };
//
//         deploy(
//             &mut self.builder,
//             &self.alice_account,
//             &DeploySource::ByContractHash {
//                 hash: self.contract_hash,
//                 entry_point: ENDPOINT_PO_SET_PRICE.to_string(),
//             },
//             args,
//             is_success,
//             None
//         );
//
//     }
//
//     pub fn set_dynamic_price(
//         &mut self,
//         price: U512,
//         price_mid: Vec<U512>,
//         chars_count_mid: Vec<u64>,
//         price_more: U512,
//         is_success: bool
//     ) {
//         let args = runtime_args! {
//             "arg_price_type" => PriceType::Dynamic,
//             "arg_price" => price,
//             "arg_price_mid" => price_mid,
//             "arg_chars_count_mid" => chars_count_mid,
//             "arg_price_more" => price_more
//         };
//
//         deploy(
//             &mut self.builder,
//             &self.alice_account,
//             &DeploySource::ByContractHash {
//                 hash: self.contract_hash,
//                 entry_point: ENDPOINT_PO_SET_PRICE.to_string(),
//             },
//             args,
//             is_success,
//             None
//         );
//     }
//
//     pub fn set_price_correct_data(&mut self) -> bool {
//
//         let args = runtime_args! {
//             "arg_price_type" => PriceType::Fixed,
//             "arg_price" => U512::from(100),
//             "arg_price_mid" => Vec::<u64>::new(),
//             "arg_chars_count_mid" => Vec::<u64>::new(),
//             "arg_price_more" => 0u64
//         };
//
//         deploy(
//             &mut self.builder,
//             &self.alice_account,
//             &DeploySource::ByContractHash {
//                 hash: self.contract_hash,
//                 entry_point: ENDPOINT_PO_SET_PRICE.to_string(),
//             },
//             args,
//             true,
//             None
//         );
//
//         true
//     }
//
//     pub fn set_price_incorrect_data(&mut self) -> bool {
//         let args = runtime_args! {
//             "arg_price_type" => PriceType::Fixed,
//             "arg_price" => U512::from(100),
//             "arg_price_mid" => Vec::<u64>::new(),
//             "arg_chars_count_mid" => Vec::<u64>::new(),
//             "arg_price_more" => 0u64
//         };
//
//         deploy(
//             &mut self.builder,
//             &self.bob_account,
//             &DeploySource::ByContractHash {
//                 hash: self.contract_hash,
//                 entry_point: ENDPOINT_PO_SET_PRICE.to_string(),
//             },
//             args,
//             false,
//             None
//         );
//
//         false
//     }
//
//     pub fn get_price(&mut self) -> U512 {
//         let price: U512 = query(
//             &mut self.builder,
//             self.contract_hash.into(),
//             &[KEY_PO_PRICE.to_string()]
//         );
//         price
//     }
//
//     pub fn get_price_mid(&mut self) -> Vec<U512> {
//         query(
//             &mut self.builder,
//             self.contract_hash.into(),
//             &[KEY_PO_PRICE_MID.to_string()]
//         )
//     }
//
//     pub fn get_chars_count(&mut self) -> Vec<u64> {
//         query(
//             &mut self.builder,
//             self.contract_hash.into(),
//             &[KEY_PO_CHARS_COUNT_MID.to_string()]
//         )
//     }
//
//     pub fn get_price_more(&mut self) -> U512 {
//         query(
//             &mut self.builder,
//             self.contract_hash.into(),
//             &[KEY_PO_PRICE_MORE.to_string()]
//         )
//     }
//
// }
//
// #[test]
// fn should_test_set_fixed_price() {
//     let mut context = PriceOracleContractContext::deploy();
//     context.set_fixed_price(U512::from(121_000), true);
//     let price = context.get_price();
//     assert_eq!(price, U512::from(121_000))
// }
//
// #[test]
// fn should_test_set_dynamic_price_correct_data() {
//     let mut context = PriceOracleContractContext::deploy();
//     context.set_dynamic_price(
//         U512::from(100_000),
//         vec![
//             U512::from(90_000),
//             U512::from(80_000),
//             U512::from(70_000),
//         ],
//         vec![3, 4, 5],
//         U512::from(60_000),
//         true
//     );
//     let price = context.get_price();
//     let price_mid = context.get_price_mid();
//     let chars_count = context.get_chars_count();
//     let price_more = context.get_price_more();
//
//     assert_eq!(price, U512::from(100_000));
//     assert_eq!(price_mid, vec![
//         U512::from(90_000),
//         U512::from(80_000),
//         U512::from(70_000),
//     ]);
//     assert_eq!(chars_count, vec![3, 4, 5]);
//     assert_eq!(price_more, U512::from(60_000));
// }
//
// #[test]
// fn should_test_set_dynamic_price_incorrect_data() {
//     let mut context = PriceOracleContractContext::deploy();
//     context.set_dynamic_price(
//         U512::from(100_000),
//         vec![
//             U512::from(90_000),
//             U512::from(80_000),
//         ],
//         vec![3, 4, 5],
//         U512::from(60_000),
//         false
//     );
// }
