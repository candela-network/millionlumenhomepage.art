import {Contract, networks} from 'Million';

export async function get({params, request}) {

  const FakeWallet = {
    isConnected: function()  { return false },
  }

  const million = new Contract({
    ...networks.localnet,
    rpcUrl: 'http://localhost:8000/soroban/rpc',
    wallet: FakeWallet,
  })
  console.log("Calling total_supply");
  let total = await million.totalSupply({wallet: FakeWallet});
  let uris = []
  for (let i=0; i < total; i++) {
    let data = await million.tokenUri({token_id: i}, {wallet: FakeWallet});
    uris.push(data);
  }
  console.log(total);
  return {
    body: JSON.stringify({
      supply: total,
      uris: uris,
    }),
  };
}
