import * as million from 'Million';
import {promises as fs} from 'node:fs';
import {verify, Keypair} from 'stellar-base';

const FakeWallet = {
  isConnected: function()  { return false },
};

export async function get({params, request}) {

  let id = params.id;
  if (!id.startsWith("0x")) {
    return new Response(null, {status: 404});
  }

  let filename = `data-${id}.json`;
  let data = {
    name: id,
    description: "",
    image: `${import.meta.env.SITE}${import.meta.env.BASE_URL}question.png`,
    home_page: `${import.meta.env.SITE}${import.meta.env.BASE_URL}test/${id}`,
  };
  try {
    data = JSON.parse(await fs.readFile(filename, "utf8"));
  } catch (e) {
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
    let owner = await million.ownerOf({token_id: token_id }, {wallet: FakeWallet});
     

    const body = await request.json();
    console.log(body.data)
    console.log(body.signature)
    console.log(body.other)
    console.log(owner)
    let kp = Keypair.fromPublicKey(owner)
    console.log(kp)
    let r = kp.verify(Buffer.from(body.data), Buffer.from(body.signature, "hex"))
    console.log(r)
    r = kp.verify(Buffer.from(body.data), Buffer.from(body.other, "hex"))
    console.log(r);
    
    return new Response(JSON.stringify({
      verified: r
    }), {
      status: 200
    })
  }
  return new Response(null, { status: 400 });
}
