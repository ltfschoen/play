const { ApiPromise } = require('@polkadot/api');
const testKeyring = require('@polkadot/keyring/testing');
const { randomAsU8a } = require('@polkadot/util-crypto');

const ALICE = '5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY';
const ORG = 123;

async function main () {
  const api = await ApiPromise.create(
    {
      types: {
        Org: {
          id: 'u64',
          fee_paid: 'Boolean'
        },
      }
    }
  );

  const keyring = testKeyring.default();
  const nonce = await api.query.system.accountNonce(ALICE);
  const alicePair = keyring.getPair(ALICE);
  const recipient = keyring.addFromSeed(randomAsU8a(32)).address;

  console.log('Creating org', ORG, 'from', alicePair.address, 'to', recipient, 'with nonce', nonce.toString());

  api.tx.manager
    .createOrg(ORG)
    .signAndSend(alicePair, { nonce }, async ({ events = [], status }) => {
      console.log('Transaction status:', status.type);

      if (status.isFinalized) {
        console.log('Completed at block hash', status.asFinalized.toHex());

        let unsub = api.query.manager.orgs(ORG, orgInfo => {
          if (orgInfo.isNone){
            console.log(`Org info is <None>`);
          } else {
            console.log(`Org info is ${orgInfo.isSome().unwrap()}`);
          }
        }).then(res => {
          return res && unsub();
        })
        .catch(console.error);

        console.log('Events:');

        events.forEach(({ phase, event: { data, method, section } }) => {
          console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
        });

        process.exit(0);
      }
    });
}

main().catch(console.error);