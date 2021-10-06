birthday-quest
==============

This app was initialized with [create-near-app]


Quick Start
===========

The most automated way to run this project locally:

1. Prerequisites: Make sure you've installed [Node.js] ≥ 12
2. Install dependencies: `npm install`
3. Compile your contract `npm run build:contract`

Now you'll have a local development environment backed by the NEAR TestNet!

Go ahead and play with the app and the code. As you make code changes, the app will automatically reload.


Exploring The Code
==================

The "backend" code lives in the `/contract` folder. See the README there for more info.


Deploy
======

Every smart contract in NEAR has its [own associated account][NEAR accounts]. When you run `npm run dev:deploy:contract`, your smart contract gets deployed to the live NEAR TestNet with a throwaway account. When you're ready to make it permanent, here's how.


Step 0: Install near-cli (optional)
-----------------------------------

[near-cli] is a command line interface (CLI) for interacting with the NEAR blockchain. It was installed to the local `node_modules` folder when you ran `yarn install`, but for best ergonomics you may want to install it globally:

    npm install --global near-cli

Or, if you'd rather use the locally-installed version, you can prefix all `near` commands with `npx`

Ensure that it's installed with `near --version` (or `npx near --version`)


Step 1: Create an account for the contract
------------------------------------------

Each account on NEAR can have at most one contract deployed to it. If you've already created an account such as `your-name.testnet`, you can deploy your contract to `birthday-quest.your-name.testnet`. Assuming you've already created an account on [NEAR Wallet], here's how to create `birthday-quest.your-name.testnet`:

1. Authorize NEAR CLI, following the commands it gives you:

      near login

2. Create a subaccount (replace `YOUR-NAME` below with your actual account name):

      near create-account birthday-quest.YOUR-NAME.testnet --masterAccount YOUR-NAME.testnet


Step 2: set contract name in code
---------------------------------

Modify the line in `src/config.js` that sets the account name of the contract. Set it to the account id you used above.

    const CONTRACT_NAME = process.env.CONTRACT_NAME || 'birthday-quest.YOUR-NAME.testnet'


Step 3: deploy!
---------------

Two commands:

    npm run build:contract
    npm run deploy:contract

As you can see in `package.json`, this builds & deploys smart contract to NEAR TestNet


Troubleshooting
===============

On Windows, if you're seeing an error containing `EPERM` it may be related to spaces in your path. Please see [this issue](https://github.com/zkat/npx/issues/209) for more details.


  [create-near-app]: https://github.com/near/create-near-app
  [Node.js]: https://nodejs.org/en/download/package-manager/
  [jest]: https://jestjs.io/
  [NEAR accounts]: https://docs.near.org/docs/concepts/account
  [NEAR Wallet]: https://wallet.testnet.near.org/
  [near-cli]: https://github.com/near/near-cli
  [gh-pages]: https://github.com/tschaub/gh-pages
