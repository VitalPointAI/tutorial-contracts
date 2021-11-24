## Prerequisites
Ensure `near-cli` is installed by running:

```
near --version
```

If needed, install `near-cli`:

```
npm install near-cli -g
```

Ensure `Rust` is installed by running:

```
rustc --version
```

If needed, install `Rust`:

```
curl https://sh.rustup.rs -sSf | sh
```

Install dependencies

```
npm install
```

## Building this contract
To make the build process compatible with multiple operating systems, the build process exists as a script in `package.json`.
There are a number of special flags used to compile the smart contract into the wasm file.
Run this command to build and place the wasm file in the `res` directory:
```bash
npm run build
```

**Note**: Instead of `npm`, users of [yarn](https://yarnpkg.com) may run:
```bash
yarn build
```

### Important
If you encounter an error similar to:
>note: the `wasm32-unknown-unknown` target may not be installed

Then run:

```bash
rustup target add wasm32-unknown-unknown
```

## Using this contract

### Quickest deploy
Build and deploy this smart contract to an development account. This development account will be created automatically and is not intended to be permanent. Please see the "Standard deploy" section for creating a more personalized account to deploy to.

```bash
near dev-deploy --wasmFile res/counter.wasm --helperUrl https://near-contract-helper.onrender.com
```

Behind the scenes, this is creating an account and deploying a contract to it. On the console, notice a message like:

>Done deploying to dev-1234567890123

In this instance, the account is `dev-1234567890123`. A file has been created containing the key to the account, located at `neardev/dev-account`. 

### Standard deploy
In this option, the smart contract will get deployed to a specific account created with the NEAR Wallet.

If you do not have a NEAR account, please create one with [NEAR Wallet](https://wallet.testnet.near.org).

Make sure you have credentials saved locally for the account you want to deploy the contract to. To perform this run the following `near-cli` command:

```
near login
```

Deploy the contract:

```bash
near deploy --wasmFile res/counter.wasm --accountId YOUR_ACCOUNT_NAME
```

