fn main() {
    println!("Hello, world!");
    let evmAddress = convertToEvmAddress("5CAFyhxaQEzLuLjEYac9q1g9UuD1dy2dECn9DR5fsuDaUzHj");
    println!("evmAddress: {}", evmAddress);

    let substrateAddress = convertToSubstrateAddress("0xE22E1E9860a931fDBfcA58fdD345806d079750B6");
    prinln!("substrateAddress: {}", substrateAddress);
}

fn convertToEvmAddress(address: H256) -> H160 {
    let mut evmAddress: [u8; 20] = Default::default();
      evmAddress[0..20].copy_from_slice(&address[0..20]);
      evmAddress
}

fn convertToSubstrateAddress(address: H160) -> AccountId32 {
	HashedAddressMapping<BlakeTwo256>::into_account_id(address)
}