#!/usr/bin/env node
// Post-install script to copy the confrisk-npm binary

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const BINARY_NAME = 'confrisk-npm';
const BINARY_PATH = path.join(__dirname, BINARY_NAME);

console.log('Installing confrisk-npm...');

// Check if binary exists
if (!fs.existsSync(BINARY_PATH)) {
    console.error('Error: confrisk-npm binary not found!');
    console.error('Expected location:', BINARY_PATH);
    console.error('');
    console.error('Please build the binary first:');
    console.error('  cd /path/to/confrisk');
    console.error('  cargo build --release --bin confrisk-npm');
    console.error('  npm run package');
    process.exit(1);
}

// Make binary executable
try {
    fs.chmodSync(BINARY_PATH, 0o755);
    console.log('confrisk-npm installed successfully!');
    console.log('');
    console.log('Try it:');
    console.log('  confrisk-npm --help');
    console.log('  confrisk-npm --path /path/to/project');
    console.log('');
} catch (err) {
    console.error('Error making binary executable:', err.message);
    process.exit(1);
}
