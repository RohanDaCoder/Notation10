#!/usr/bin/env node
// Sync the packaged version to the release tag before building.
// Usage: TAG=v0.2.0 node scripts/sync_version.js
//
// Patches:
//   - crates/notation-tauri/tauri.conf.json  -> bundle version (artifact name)
//   - Cargo.toml [workspace.package]         -> crate version (updater compare)

const fs = require('fs');

const tag = process.env.TAG;
if (!tag) {
  console.error('TAG environment variable is required (e.g. v0.2.0)');
  process.exit(1);
}
const version = tag.replace(/^v/, '');

const confPath = 'crates/notation-tauri/tauri.conf.json';
const conf = JSON.parse(fs.readFileSync(confPath, 'utf8'));
conf.version = version;
fs.writeFileSync(confPath, JSON.stringify(conf, null, 2) + '\n');

const cargoPath = 'Cargo.toml';
let cargo = fs.readFileSync(cargoPath, 'utf8');
if (!/\[workspace\.package\]\r?\nversion = "[^"]+"/.test(cargo)) {
  console.error('failed to locate [workspace.package] version in Cargo.toml');
  process.exit(1);
}
cargo = cargo.replace(
  /(\[workspace\.package\]\r?\nversion = ")[^"]+(")/,
  `$1${version}$2`,
);
fs.writeFileSync(cargoPath, cargo);

console.log(`versions synced to ${version}`);
