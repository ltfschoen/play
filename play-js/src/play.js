const { ApiPromise, WsProvider } = require('@polkadot/api');
const testKeyring = require('@polkadot/keyring/testing');

const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';

async function main () {
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({
    provider,
    types: {
      Kitty: {
        id: "H256",
        dna: "H256",
        price: "Balance",
        gen: "u64"
      }
    }
  });
  const keyring = testKeyring.default();
  const nonce = await api.query.system.accountNonce(ALICE);
  const alicePair = keyring.getPair(ALICE);

  console.log('Creating Kitty with', alicePair.address, 'with nonce', nonce.toString());

  api.tx.kitty
    .createKitty()
    .signAndSend(alicePair, { nonce }, async ({ events = [], status }) => {
      console.log('Transaction status:', status.type);
      if (status.isFinalized) {
        console.log('Completed at block hash', status.asFinalized.toHex());
        await api.query.kitty.allKittiesArray(0, async (kittyHash) => {
          if (kittyHash.isNone){
            console.log(`Kittie hash is <None>`);
          } else {
            console.log(`Kittie hash is ${kittyHash}`);
            await api.query.kitty.kitties(kittyHash, function (kitty) {
              console.log('Kitty: ', kitty.toJSON());
            });
          }
        })
        .catch(console.error);
      }
    });
}

main().catch(console.error);