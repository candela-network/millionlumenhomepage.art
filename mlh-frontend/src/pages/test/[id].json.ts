import * as million from 'Million';
import {promises as fs} from 'node:fs';

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
    image: `${import.meta.env.SITE}${import.meta.env.BASE_URL}test/default.png`,
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
  if (id.startsWith("0x") && request.headers.get("Content-Type") === "application/json") {

    let token_id = parseInt(id.substring(2), 16);
    let owner = await million.ownerOf({token_id: token_id }, {wallet: wallet});
     

    const body = await request.json();
    const name = body.name;
    return new Response(JSON.stringify({
      message: "Your name was: " + name
    }), {
      status: 200
    })
  }
  return new Response(null, { status: 400 });
}
