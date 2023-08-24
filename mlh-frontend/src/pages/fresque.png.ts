
import * as million from 'Million';
import Jimp from 'jimp';
import {promises as fs} from 'node:fs';
import {x} from '../keyStore.ts';

const FakeWallet = {
  isConnected: function()  { return false },
};
export async function get({params, request}) {

  let fresque = undefined;
  let now = Date.now();
  if (now - x.get() > 10000) {
    console.log("Updating the fresco")
    fresque = await update();
    x.set(now);
  } else {
    console.log("from cache")
    //fresque = await update();
    fresque = await Jimp.read('./data/data-fresque.png');
  }
  return {
    body: "data:image/png;base64," + (await fresque.getBufferAsync("image/png")).toString("base64"),
  };
}

async function update() {

  let max = await million.totalSupply({wallet: FakeWallet});
  let fresque = new Jimp(2048, 512);
  for (let id=0; id < max; id++) {
    let filename = `./data/data-0x${id.toString(16).padStart(3, "0")}.json`;
    try {
      let data = JSON.parse(await fs.readFile(filename, "utf8"));
      let b64 = data.image.substring(data.image.indexOf(',')+1);
      let image = b64 != data.image ? await Jimp.read(Buffer.from(b64, "base64")) : await Jimp.read(data.image);
      let xy = data.coords;
       fresque.composite(image, xy[0] * 16, xy[1] * 16);
    } catch (e) {
      //
      console.log(e)
    }
  }
   fresque.write('./data/data-fresque.png');

  return fresque;
}
