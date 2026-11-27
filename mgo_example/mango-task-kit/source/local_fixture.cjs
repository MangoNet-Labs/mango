'use strict';

const assert = require('node:assert/strict');
const crypto = require('node:crypto');
const fs = require('node:fs');
const path = require('node:path');
const {createRequire} = require('node:module');

const sourceName = 'TestEntitlementSource.sol';
const eventSignature = 'TestEntitlementRequested(bytes32,bytes32)';
const expectedSourceHash = 'ece34807a86be1654e5b16a9c6b5d7a943bd216b8484afc765a5d222bb87daa6';
const expectedLockHash = '4db878a49edc6389a16918b54bc1e227fd94e9b3fc390a2fbaf751e043c6ff0e';
const chainId = 31337;
const host = '127.0.0.1';
const sha256 = bytes => crypto.createHash('sha256').update(bytes).digest('hex');

function payloadDigest(payload) {
  const digest = crypto.createHash('sha256');
  for (const field of [
    'mango-entitlement-payload-v1', payload.action, payload.campaign_id,
    payload.entitlement_code, payload.recipient_mango,
  ]) {
    const encoded = Buffer.from(field, 'utf8');
    const length = Buffer.alloc(4);
    length.writeUInt32BE(encoded.length);
    digest.update(length).update(encoded);
  }
  return '0x' + digest.digest('hex');
}

function publishReady(filename, ready) {
  const temporary = filename + '.' + process.pid + '.tmp';
  fs.writeFileSync(temporary, JSON.stringify(ready, null, 2) + '\n', {flag: 'wx', mode: 0o600});
  try {
    fs.linkSync(temporary, filename);
  } finally {
    fs.unlinkSync(temporary);
  }
}

async function main() {
  assert.equal(process.argv.length, 5, 'Usage: node local_fixture.cjs <S3-fixture-directory> <target.json> <ready.json>');
  const [fixtureDirectory, targetFile, readyFile] = process.argv.slice(2).map(value => path.resolve(value));
  const target = JSON.parse(fs.readFileSync(targetFile, 'utf8'));
  for (const key of ['target_genesis_digest', 'target_package_id', 'recipient']) {
    assert.match(target[key], /^0x[0-9a-f]{64}$/);
  }
  assert.ok(Number.isSafeInteger(target.route_revision) && target.route_revision >= 1);
  const depth = target.confirmation_depth ?? 1;
  assert.ok(Number.isInteger(depth) && depth >= 1 && depth <= 32);
  const payload = {
    action: 'issue_test_entitlement',
    campaign_id: target.campaign_id ?? 's4-local',
    entitlement_code: target.entitlement_code ?? 'preview-access',
    recipient_mango: target.recipient,
  };
  for (const key of ['campaign_id', 'entitlement_code']) {
    assert.equal(typeof payload[key], 'string');
    assert.match(payload[key], /^[!-~]{1,64}$/);
  }

  const source = fs.readFileSync(path.join(fixtureDirectory, sourceName));
  assert.equal(sha256(source), expectedSourceHash, 'S3 Solidity source changed');
  assert.equal(sha256(fs.readFileSync(path.join(fixtureDirectory, 'package-lock.json'))), expectedLockHash,
    'S3 fixture dependency lock changed');
  const fixtureRequire = createRequire(path.join(fixtureDirectory, 'package.json'));
  const ganache = fixtureRequire('ganache');
  const solc = fixtureRequire('solc');
  const ganacheVersion = fixtureRequire('ganache/package.json').version;
  assert.equal(ganacheVersion, '7.9.2');
  assert.equal(solc.version(), '0.8.28+commit.7893614a.Emscripten.clang');
  const compilation = JSON.parse(solc.compile(JSON.stringify({
    language: 'Solidity',
    sources: {[sourceName]: {content: source.toString('utf8')}},
    settings: {
      evmVersion: 'shanghai',
      outputSelection: {'*': {'*': ['evm.bytecode.object', 'evm.deployedBytecode.object', 'evm.methodIdentifiers']}},
    },
  })));
  assert.ok(!compilation.errors?.some(error => error.severity === 'error'), 'Pinned Solidity fixture did not compile');
  const {evm} = compilation.contracts[sourceName].TestEntitlementSource;

  const server = ganache.server({
    chain: {
      chainId,
      networkId: chainId,
      hardfork: 'shanghai',
      time: new Date(1900000000 * 1000),
      vmErrorsOnRPCResponse: false,
    },
    wallet: {deterministic: true, totalAccounts: 1},
    miner: {timestampIncrement: 1},
    logging: {quiet: true},
    server: {ws: false},
  });
  await server.listen(0, host);
  let closing = false;
  async function close() {
    if (!closing) {
      closing = true;
      await server.close();
    }
  }
  for (const signal of ['SIGTERM', 'SIGINT']) {
    process.once(signal, () => close().catch(() => { process.exitCode = 1; }));
  }
  try {
    const {address, port} = server.address();
    assert.equal(address, host);
    assert.ok(Number.isInteger(port) && port > 0);
    const rpc = (method, params = []) => server.provider.request({method, params});
    assert.equal(await rpc('eth_chainId'), '0x7a69');
    const [from] = await rpc('eth_accounts');
    const deployment = await rpc('eth_sendTransaction', [
      {from, data: '0x' + evm.bytecode.object, gas: '0x500000'},
    ]);
    const receipt = await rpc('eth_getTransactionReceipt', [deployment]);
    assert.equal(receipt.status, '0x1');
    const contract = receipt.contractAddress;
    const code = await rpc('eth_getCode', [contract, receipt.blockNumber]);
    assert.equal(code, '0x' + evm.deployedBytecode.object);
    const genesis = await rpc('eth_getBlockByNumber', ['0x0', false]);
    const eventTopic = await rpc('web3_sha3', ['0x' + Buffer.from(eventSignature, 'utf8').toString('hex')]);
    const selector = evm.methodIdentifiers['record(bytes32,bytes32)'];
    assert.match(selector, /^[0-9a-f]{8}$/);
    publishReady(readyFile, {
      rpc_url: `http://${host}:${port}`,
      chain_id: chainId,
      source_genesis_hash: genesis.hash,
      source_contract: contract,
      start_block: Number.parseInt(receipt.blockNumber, 16),
      confirmation_depth: depth,
      event_topic: eventTopic,
      contract_code_sha256: sha256(Buffer.from(code.slice(2), 'hex')),
      record_selector: '0x' + selector,
      payload_digest: payloadDigest(payload),
      payload,
      target_genesis_digest: target.target_genesis_digest,
      target_package_id: target.target_package_id,
      recipient: target.recipient,
      route_revision: target.route_revision,
      fixture_versions: {node: process.version, ganache: ganacheVersion, solidity: solc.version()},
    });
  } catch (error) {
    await close();
    throw error;
  }
}

main().catch(error => {
  console.error(error.message);
  process.exitCode = 1;
});
