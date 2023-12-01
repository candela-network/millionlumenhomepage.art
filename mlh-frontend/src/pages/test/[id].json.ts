import {Contract, networks} from 'Million';
import {promises as fs} from 'node:fs';
import {verify, Keypair} from 'soroban-client';

const FakeWallet = {
  isConnected: function()  { return false },
};
const million = new Contract({
  ...networks.localnet,
  rpcUrl: 'http://localhost:8000/soroban/rpc',
  wallet: FakeWallet,
})

export async function get({params, request}) {

  let id = params.id;
  if (!id.startsWith("0x")) {
    return new Response(null, {status: 404});
  }

  let filename = `./data/data-${id}.json`;
  let data = {
    name: id,
    description: "",
    image: `${import.meta.env.SITE}${import.meta.env.BASE_URL}question.png`,
    home_page: `${import.meta.env.SITE}${import.meta.env.BASE_URL}test/${id}`,
  };
  try {
    data = JSON.parse(await fs.readFile(filename, "utf8"));
  } catch (e) {

    console.log(id)
    let xy = await million.coords({token_id: parseInt(id)})
    data.coords = xy;
    fs.writeFile(filename, JSON.stringify(data));
  }

  return {
    body: JSON.stringify(data),
  };
}

export const post: APIRoute = async ({params, request }) => {
  let id = params.id;
  console.log("recv " + id)
  console.log(request.headers.get("Content-Type"))
  if (id.startsWith("0x") && request.headers.get("Content-Type") === "application/json") {

    let token_id = parseInt(id.substring(2), 16);
    let owner = await million.ownerOf({token_id: token_id });
     

    const body = await request.json();
    let kp = Keypair.fromPublicKey(owner)
    console.log(kp)
    let verified = kp.verify(Buffer.from(body.data, "base64"), Buffer.from(body.signature, "hex"))

    if (verified) {

      let filename = `./data/data-${id}.json`;
      let data = {
        name: id,
        description: "",
        image: `${import.meta.env.SITE}${import.meta.env.BASE_URL}question.png`,
        home_page: `${import.meta.env.SITE}${import.meta.env.BASE_URL}test/${id}`,
      };
      try {
        data = JSON.parse(await fs.readFile(filename, "utf8"));
        data.image = Buffer.from(body.data, "base64").toString()
      } catch (e) {
        console.log("Error writing json")
      }
        fs.writeFile(filename, JSON.stringify(data));
      return new Response(JSON.stringify({
        verified: verified
      }), {
          status: 200
        })
    }
  }
  return new Response(null, { status: 400 });
}
