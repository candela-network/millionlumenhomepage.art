#[cfg(test)]
extern crate std;

use super::*;
use crate::MillionError;
use soroban_sdk::{testutils::Address as _, Address, Env};
#[test]
fn init() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Million);
    let client = MillionClient::new(&env, &contract_id);

    let asset_admin = Address::random(&env);
    let native_addr = env.register_stellar_asset_contract(asset_admin);

    let admin = Address::random(&env);
    client.initialize(&admin, &native_addr, &100);

    assert_eq!(client.name(), String::from_slice(&env, "Pixel"));
    assert_eq!(client.symbol(), String::from_slice(&env, "PIX"));
}

#[test]
fn mint() {
    let env = Env::default();
    let contract_id = env.register_contract(None, Million);
    let client = MillionClient::new(&env, &contract_id);

    let asset_admin = Address::random(&env);
    let native_addr = env.register_stellar_asset_contract(asset_admin.clone());
    let asset_client_admin = token::AdminClient::new(&env, &native_addr);

    let admin = Address::random(&env);
    client.initialize(&admin, &native_addr, &2_560_000_000);

    assert_eq!(client.name(), String::from_slice(&env, "Pixel"));
    assert_eq!(client.symbol(), String::from_slice(&env, "PIX"));

    let user1 = Address::random(&env);
    asset_client_admin
        .mock_all_auths()
        .mint(&user1, &2_560_000_000);
    client.mock_all_auths().mint(&user1);
    let auths = env.auths();
    for a in auths.into_iter() {
        std::println!("{:?}", a.1);
    }

    //std::println!("{:?}", r1);
    let user2 = Address::random(&env);
    asset_client_admin
        .mock_all_auths()
        .mint(&user2, &2_560_000_000);
    let _ = client.mock_all_auths().try_mint(&user2);

    assert_eq!(client.balance_of(&user1), 1);
    assert_eq!(client.balance_of(&user2), 1);

    let mut uri = [0u8; 62];
    client.token_uri(&0).copy_into_slice(&mut uri);
    assert_eq!(
        uri,
        "https://0x000.millionlumenhomepage.art/.well-known/erc721.json".as_bytes()
    );
    client.token_uri(&1).copy_into_slice(&mut uri);
    assert_eq!(
        uri,
        "https://0x001.millionlumenhomepage.art/.well-known/erc721.json".as_bytes()
    );
}
#[test]
fn mint_all() {
    let max = crate::MAX_SUPPLY + 1;
    let env = Env::default();
    let contract_id = env.register_contract(None, Million);
    let client = MillionClient::new(&env, &contract_id);

    let asset_admin = Address::random(&env);
    let native_addr = env.register_stellar_asset_contract(asset_admin.clone());
    let asset_client_admin = token::AdminClient::new(&env, &native_addr);

    let admin = Address::random(&env);
    client.initialize(&admin, &native_addr, &2_560_000_000);

    env.budget().reset_unlimited();
    for _ in 0..max {
        let user1 = Address::random(&env);
        asset_client_admin
            .mock_all_auths()
            .mint(&user1, &2_560_000_000);
        let _ = client.mock_all_auths().try_mint(&user1);
    }

    assert_eq!(client.total_supply(), max);
    let user1 = Address::random(&env);
    asset_client_admin
        .mock_all_auths()
        .mint(&user1, &2_560_000_000);
    let result = client.mock_all_auths().try_mint(&user1);

    assert_eq!(result, Err(Ok(MillionError::Exhausted)));
}

/*
*
=======================================================
Cpu limit: 40000000; used: 732594
Mem limit: 52428800; used: 25854
=======================================================
CostType                 cpu_insns      mem_bytes
WasmInsnExec             0              0
WasmMemAlloc             0              0
HostMemAlloc             204450         24186
HostMemCpy               2737           0
HostMemCmp               129083         0
InvokeHostFunction       0              0
VisitObject              1102           0
ValXdrConv               9514           0
ValSer                   16724          1116
ValDeser                 0              0
ComputeSha256Hash        12954          80
ComputeEd25519PubKey     0              0
MapEntry                 48760          0
VecEntry                 60             0
GuardFrame               4050           472
VerifyEd25519Sig         0              0
VmMemRead                0              0
VmMemWrite               0              0
VmInstantiation          0              0
VmCachedInstantiation    0              0
InvokeVmFunction         0              0
ChargeBudget             303160         0
ComputeKeccak256Hash     0              0
ComputeEcdsaSecp256k1Key 0              0
ComputeEcdsaSecp256k1Sig 0              0
RecoverEcdsaSecp256k1Key 0              0
Int256AddSub             0              0
Int256Mul                0              0
Int256Div                0              0
Int256Pow                0              0
Int256Shift              0              0
=======================================================
=======================================================
Cpu limit: 40000000; used: 7257086
Mem limit: 52428800; used: 457054
=======================================================
CostType                 cpu_insns      mem_bytes
WasmInsnExec             0              0
WasmMemAlloc             0              0
HostMemAlloc             204450         455386
HostMemCpy               2737           0
HostMemCmp               2183624        0
InvokeHostFunction       0              0
VisitObject              1102           0
ValXdrConv               9514           0
ValSer                   16724          1116
ValDeser                 0              0
ComputeSha256Hash        12954          80
ComputeEd25519PubKey     0              0
MapEntry                 733891         0
VecEntry                 60             0
GuardFrame               4050           472
VerifyEd25519Sig         0              0
VmMemRead                0              0
VmMemWrite               0              0
VmInstantiation          0              0
VmCachedInstantiation    0              0
InvokeVmFunction         0              0
ChargeBudget             4087980        0
ComputeKeccak256Hash     0              0
ComputeEcdsaSecp256k1Key 0              0
ComputeEcdsaSecp256k1Sig 0              0
RecoverEcdsaSecp256k1Key 0              0
Int256AddSub             0              0
Int256Mul                0              0
Int256Div                0              0
Int256Pow                0              0
Int256Shift              0              0
=======================================================





=======================================================
Cpu limit: 40000000; used: 8578760
Mem limit: 52428800; used: 746482
=======================================================
CostType                 cpu_insns      mem_bytes
WasmInsnExec             90475          0
WasmMemAlloc             0              0
HostMemAlloc             211500         39777
HostMemCpy               3059           0
HostMemCmp               132454         0
InvokeHostFunction       18560          0
VisitObject              1102           0
ValXdrConv               9514           0
ValSer                   16724          1116
ValDeser                 0              0
ComputeSha256Hash        12954          80
ComputeEd25519PubKey     0              0
MapEntry                 51357          0
VecEntry                 60             0
GuardFrame               4050           472
VerifyEd25519Sig         0              0
VmMemRead                0              0
VmMemWrite               0              0
VmInstantiation          7699275        704551
VmCachedInstantiation    0              0
InvokeVmFunction         5926           486
ChargeBudget             321750         0
ComputeKeccak256Hash     0              0
ComputeEcdsaSecp256k1Key 0              0
ComputeEcdsaSecp256k1Sig 0              0
RecoverEcdsaSecp256k1Key 0              0
Int256AddSub             0              0
Int256Mul                0              0
Int256Div                0              0
Int256Pow                0              0
Int256Shift              0              0
=======================================================


=======================================================
Cpu limit: 40000000; used: 15091999
Mem limit: 52428800; used: 1177682
=======================================================
CostType                 cpu_insns      mem_bytes
WasmInsnExec             90475          0
WasmMemAlloc             0              0
HostMemAlloc             211500         470977
HostMemCpy               3059           0
HostMemCmp               2182443        0
InvokeHostFunction       18560          0
VisitObject              1102           0
ValXdrConv               9514           0
ValSer                   16724          1116
ValDeser                 0              0
ComputeSha256Hash        12954          80
ComputeEd25519PubKey     0              0
MapEntry                 738237         0
VecEntry                 60             0
GuardFrame               4050           472
VerifyEd25519Sig         0              0
VmMemRead                0              0
VmMemWrite               0              0
VmInstantiation          7699275        704551
VmCachedInstantiation    0              0
InvokeVmFunction         5926           486
ChargeBudget             4098120        0
ComputeKeccak256Hash     0              0
ComputeEcdsaSecp256k1Key 0              0
ComputeEcdsaSecp256k1Sig 0              0
RecoverEcdsaSecp256k1Key 0              0
Int256AddSub             0              0
Int256Mul                0              0
Int256Div                0              0
Int256Pow                0              0
Int256Shift              0              0
=======================================================
*/
