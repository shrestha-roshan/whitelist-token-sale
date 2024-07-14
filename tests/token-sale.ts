import * as anchor from "@coral-xyz/anchor";
import * as spl from "@solana/spl-token";
import { Program } from "@coral-xyz/anchor";
import { TokenSale } from "../target/types/token_sale";
import { SEEDS, USDC_DECIMALS } from "./shared";
import { assert, expect } from "chai";
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
require('dotenv').config();

describe("token-sale", () => {
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider();
  const program = anchor.workspace.TokenSale as Program<TokenSale>;

  let now = Date.now() / 1000;
  let auctionName = "Test Auction 22";
  let auction: anchor.web3.PublicKey;
  let auctionVault: anchor.web3.PublicKey;
  let auctionToken: anchor.web3.PublicKey;
  let admin: anchor.web3.PublicKey;
  let auctionVaultAta: anchor.web3.PublicKey;
  let adminTokenAccount: anchor.web3.PublicKey;
  let depositor1: anchor.web3.Keypair;
  let depositor1TokenAccount: anchor.web3.PublicKey;
  let auctionWhitelist: anchor.web3.PublicKey;
  let userPda: anchor.web3.PublicKey;

  it("Initiallize Accounts", async () => {    
    auctionToken = new anchor.web3.PublicKey("Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr"); // get from https://spl-token-faucet.com/

    admin = provider.publicKey;
    adminTokenAccount = await spl.getAssociatedTokenAddress(
      auctionToken,
      admin,
      true,
      spl.TOKEN_PROGRAM_ID
    );

    [auction] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(SEEDS.AUCTION), Buffer.from(auctionName)], program.programId);
    [auctionVault] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(SEEDS.AUCTION_VAULT), auction.toBuffer()], program.programId);
    auctionVaultAta = await spl.getAssociatedTokenAddress(
      auctionToken,
      auctionVault,
      true,
      spl.TOKEN_PROGRAM_ID
    );
    [auctionWhitelist] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(SEEDS.WHITELIST), auction.toBuffer()], program.programId);
    
    depositor1 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(
      JSON.parse(process.env.DEPOSITOR_1)
    ));
    depositor1TokenAccount = await spl.getAssociatedTokenAddress(
      auctionToken,
      depositor1.publicKey,
      true,
      spl.TOKEN_PROGRAM_ID
    );
    [userPda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(SEEDS.CONSUMER), depositor1.publicKey.toBuffer(), auction.toBuffer()], program.programId);
  });

  it("Init Auction", async () => {
    const initArgs = {
      name: auctionName,
      startTime: new anchor.BN(now),
      endTime: new anchor.BN(now + 20),
      purchaseLimit: new anchor.BN(3 * 10 ** USDC_DECIMALS),
      tokensInPool: new anchor.BN(0),
      pricePerToken: new anchor.BN(0.01 * LAMPORTS_PER_SOL)
    }
    const txSignature = await program.methods.initAuction({
      name: initArgs.name,
      startTime: initArgs.startTime,
      endTime: initArgs.endTime,
      purchaseLimit: initArgs.purchaseLimit,
      tokensInPool: initArgs.tokensInPool,
      pricePerToken: initArgs.pricePerToken
    }).accounts({
      auctionToken: auctionToken,
      admin: admin,
      auction: auction,
      auctionVault: auctionVault,
      aucitonWhitelist: auctionWhitelist,
      auctionVaultAta: auctionVaultAta,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID
    }).rpc();

    console.log(`Init Auction: https://explorer.solana.com/tx/${txSignature}?cluster=devnet`);    

    const auctionAccount = await program.account.auction.fetch(auction);
    assert.ok(auctionAccount.name == initArgs.name);
    assert.ok(auctionAccount.startTime.eq(initArgs.startTime));
    assert.ok(auctionAccount.endTime.eq(initArgs.endTime));
    assert.ok((auctionAccount.tokenDetails as any).purchaseLimit.eq(initArgs.purchaseLimit));
    assert.ok((auctionAccount.tokenDetails as any).tokensInPool.eq(initArgs.tokensInPool));
  });

  it("Should deposit X tokens", async () => {
    const amount = new anchor.BN(10 * 10 ** USDC_DECIMALS);
    const txSignature = await program.methods.depositToVault(amount).accounts({
      auctionVault: auctionVault,
      auctionToken: auctionToken,
      depositor: admin,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      auction: auction,
      auctionVaultAta: auctionVaultAta,
      depositorTokenAccount: adminTokenAccount
    }).rpc();

    console.log(`Deposit to Vault: https://explorer.solana.com/tx/${txSignature}?cluster=devnet`);    
  }); 

  it("Should whitelist Users[]", async () => {
    const txSignature = await program.methods.addWhitelist([depositor1.publicKey]).accounts({
      auction: auction,
      admin: admin,
      aucitonWhitelist: auctionWhitelist
    }).rpc();
    console.log(`Whitelist Depositor 1: https://explorer.solana.com/tx/${txSignature}?cluster=devnet`);

    const whitelistAccount = await program.account.whitelist.fetch(auctionWhitelist);
    assert.ok(whitelistAccount.whitelistAddresses.find((address) => address.equals(depositor1.publicKey)));
  });

  it("Whitelisted user 1 should be able to buy from Sale", async () => {
    const amount = new anchor.BN(2 * 10 ** USDC_DECIMALS);
    const txSignature = await program.methods.whitelistBuy(amount).accounts({
      auction: auction,
      auctionVault: auctionVault,
      auctionToken: auctionToken,
      auctionVaultAta: auctionVaultAta,
      user: depositor1.publicKey,
      userTokenAccount: depositor1TokenAccount,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      auctionWhitelist: auctionWhitelist,
      systemProgram: anchor.web3.SystemProgram.programId,
      userPda: userPda,
      associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID
    }).signers([depositor1]).rpc();

    console.log(`Whitelist Buy: https://explorer.solana.com/tx/${txSignature}?cluster=devnet`);

    const userTokenAccountBalance = await provider.connection.getTokenAccountBalance(depositor1TokenAccount);
    assert.ok(userTokenAccountBalance.value.uiAmount >= 2)
  });

  it("Whitelisted user 1 can't join the Sale because they've hit the purchase limit.", async () => {
    try {
    const amount = new anchor.BN(2 * 10 ** USDC_DECIMALS);
    const txSignature = await program.methods.whitelistBuy(amount).accounts({
        auction: auction,
        auctionVault: auctionVault,
        auctionToken: auctionToken,
        auctionVaultAta: auctionVaultAta,
        user: depositor1.publicKey,
        userTokenAccount: depositor1TokenAccount,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        auctionWhitelist: auctionWhitelist,
        systemProgram: anchor.web3.SystemProgram.programId,
        userPda: userPda,
        associatedTokenProgram: spl.ASSOCIATED_TOKEN_PROGRAM_ID
      }).signers([depositor1]).rpc();
    } catch (error) {
      expect(error.message).contains("Error Code: PurchaseLimitExceeded");
    }
  });

  it("Non-Whitelisted should not be able to buy from Sale", async () => {
    const amount = new anchor.BN(100);
    const depositor2 = anchor.web3.Keypair.fromSecretKey(new Uint8Array(
      JSON.parse(process.env.DEPOSITOR_2)
    ));
    const [user2Pda] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from(SEEDS.CONSUMER), depositor2.publicKey.toBuffer(), auction.toBuffer()], program.programId);
    const depositor2TokenAccount = await spl.getAssociatedTokenAddress(
      auctionToken,
      depositor2.publicKey,
      true,
      spl.TOKEN_PROGRAM_ID
    );
    try {
      const txSignature = await program.methods.whitelistBuy(amount).accounts({
        auction: auction,
        auctionVault: auctionVault,
        auctionToken: auctionToken,
        auctionVaultAta: auctionVaultAta,
        user: depositor2.publicKey,
        userTokenAccount: depositor2TokenAccount,
        tokenProgram: spl.TOKEN_PROGRAM_ID,
        auctionWhitelist: auctionWhitelist,
        systemProgram: anchor.web3.SystemProgram.programId,
        userPda: user2Pda
      }).signers([depositor2]).rpc();
    } catch (error) {
      expect(error.message).contains("Error Code: UserNotWhitelisted");
    }
  });

  it("After the sale ends, the admin can withdraw all funds from the Auction.", async () => {
    let auctionData = await program.account.auction.fetch(auction);
    let timeNow = new anchor.BN(Date.now()/1000);

    console.log("========== Waiting for Sale to End ==============");
    while (timeNow.lt(auctionData.endTime)) {
      await new Promise((resolve) => setTimeout(resolve, 1000));
      timeNow = new anchor.BN(Date.now()/ 1000);
    }
    await new Promise((resolve) => setTimeout(resolve, 2000));
    const txSignature = await program.methods.withdrawFromVault().accounts({
      auction: auction,
      auctionVault: auctionVault,
      auctionToken: auctionToken,
      auctionVaultAta: auctionVaultAta,
      admin: admin,
      adminTokenAccount: adminTokenAccount,
      tokenProgram: spl.TOKEN_PROGRAM_ID,
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();

    console.log(`Withdraw: https://explorer.solana.com/tx/${txSignature}?cluster=devnet`);
  });
});
