# Solana-Bootcamp-Final-Project

> Note: This project is developed on wsl.

If you are still on windows follow [this](https://learn.microsoft.com/en-us/windows/wsl/install) and get started!

### _Prerequisite_

- [Codigo cli](https://github.com/Codigo-io/platform/releases)
- [rust](https://www.rust-lang.org/tools/install)
- [Solana cli](https://docs.solana.com/cli/install-solana-cli-tools)

---

The **program** folder contains a Smart Contract source code to mint, transfer and burn NFT. The boilerplate code was generated using Codigo CLI. The business logic was then updated manually for each function/instruction.

- **Mint:** The mint instruction is responsible for creating NFT. This process is essential for introducing new tokens into the system. Based on the inputs (regarding color, description, and rarity), gem metadata will be created which will be used to mint the NFT.
- **Transfer:** The transfer instruction is used to send NFTs from one account to another.
- **Burn:** The burn instruction is used to destroy NFTs, removing them from circulation.

The **program_client** folder contains the client side code to interact/test the smart contract.
Currently there is only a app.ts file which can be tested locally.
Moving forward I'll improve the Smart Contract add a frontend app using Nextjs on client side.

## Steps to test it on local

[clone](https://github.com/NiKHiLkr23/Solana-Bootcamp-Final-Project.git) the repository

cd into **program** directory

```bash
cargo build-sbf
```

make sure you're on devnet

```bash
solana config set --url devnet

solana program deploy target/deploy/nft.so

```

This will return a program ID.

cd into **program_client** directory

```bash
npm i
#or
yarn install
```

run

```bash
npx ts-node app.ts <Program_Id>
```

Example Program_Id - JVLTjVnNskyMmn3SfXUz5cbow2mV6yyLDeeyHfyHPb8

#### We can see our transactions on the blockchain [here](https://explorer.solana.com/?cluster=devnet)

---

&nbsp;

A huge thanks to [Rise In](https://www.risein.com/) for organizing such an amazing bootcamp.🎉🎉🎉
