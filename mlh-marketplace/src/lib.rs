#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, token, Address, Env, Map, Symbol,
};

mod stub {
    use soroban_sdk::{contract, contractimpl, Address, Env};

    #[contract]
    pub struct Erc721;

    #[contractimpl]
    #[allow(dead_code)]
    impl Erc721 {
        pub fn transfer_from(
            _env: Env,
            _spender: Address,
            _from: Address,
            _to: Address,
            _token_id: u32,
        ) {
        }
        pub fn approve(
            _env: Env,
            _caller: Address,
            _operator: Option<Address>,
            _token_id: u32,
            _expiration_ledger: u32,
        ) {
        }

        pub fn balance_of(_env: Env, _owner: Address) -> u32 {
            0
        }
    }
}
#[contracttype]
#[derive(Clone)]
pub struct Nft {
    contract_id: Address,
    token_id: u32,
}

#[contracttype]
pub struct Price {
    amount: i128,
    asset: Address,
}
#[contracttype]
pub struct OrderBook {
    nfts: Map<Nft, Address>,
}

pub const CONTRACT: Symbol = symbol_short!("CONTRACT");
pub const ORDER_BOOK: Symbol = symbol_short!("BOOK");

#[contract]
pub struct MarketPlace;

///
/// An ERC721 marketplace, sellers must own an NFT from the CONTRACT
///
#[contractimpl]
impl MarketPlace {
    pub fn initialize(env: Env, contract: Address) {
        if env.storage().instance().has(&CONTRACT) {
            panic!("Already initialized");
        }
        env.storage().instance().set(&CONTRACT, &contract);
        env.storage().instance().bump(6000000, 6000000);
    }
    pub fn sell(
        env: Env,
        nft: Nft,
        seller: Address,
        owner: Address,
        price: Price,
        expiration_ledger: u32,
    ) {
        seller.require_auth();

        if price.amount <= 0 {
            panic!("Price must be positive");
        }

        let super_contract_id = env.storage().instance().get(&CONTRACT).unwrap();
        let super_contract = stub::Erc721Client::new(&env, &super_contract_id);
        if super_contract.balance_of(&owner) == 0 {
            panic!("Not authorized, only owners of CONTRACT can sell");
        }

        let erc721 = stub::Erc721Client::new(&env, &nft.contract_id);
        // Force the marketplace to be approved on the token
        // This also confirm that the token is owned and exists
        erc721.approve(
            &seller,
            &Some(env.current_contract_address()),
            &nft.token_id,
            &expiration_ledger,
        );

        // Update order book, replace previously set offer if exists
        let bump = expiration_ledger - env.ledger().sequence();
        let mut order_book = env
            .storage()
            .temporary()
            .get::<Symbol, Map<Nft, Address>>(&ORDER_BOOK)
            .unwrap_or_else(|| Map::new(&env));
        order_book.set(nft.clone(), owner);
        env.storage().temporary().set(&ORDER_BOOK, &order_book);
        env.storage().temporary().bump(&ORDER_BOOK, bump, bump);

        env.storage().temporary().set(&nft, &price);
        env.storage().temporary().bump(&nft, bump, bump);
    }
    pub fn buy(env: Env, buyer: Address, nft: Nft) {
        buyer.require_auth();

        if let Some(Price { amount, asset }) = env.storage().temporary().get(&nft) {
            let mut order_book = env
                .storage()
                .temporary()
                .get::<Symbol, Map<Nft, Address>>(&ORDER_BOOK)
                .unwrap_or_else(|| Map::new(&env));
            let seller = order_book.get(nft.clone()).expect("No seller");

            // Payment
            let token_client = token::Client::new(&env, &asset);
            token_client.transfer(&buyer, &seller, &amount);

            // Transfer NFT
            let erc721 = stub::Erc721Client::new(&env, &nft.contract_id);
            erc721.transfer_from(
                &env.current_contract_address(),
                &seller,
                &buyer,
                &nft.token_id,
            );

            // Remove NFT entry
            env.storage().temporary().remove(&nft);

            // Update order book
            order_book.remove(nft.clone());
            env.storage().temporary().set(&ORDER_BOOK, &order_book);
        } else {
            panic!("Cannot buy");
        }
    }
}

#[cfg(test)]
mod tests {
    use soroban_sdk::{xdr::Asset, Address, BytesN, Env};
    use stellar_strkey::Strkey;

    fn test() {
        //
        let env = Env::default();
    }
}
