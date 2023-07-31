import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Stablesale } from "../target/types/stablesale";

import { Keypair, LAMPORTS_PER_SOL, PublicKey, SystemProgram, Transaction } from "@solana/web3.js"
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createAccount, createMint, getAccount, getAssociatedTokenAddressSync, mintTo } from "@solana/spl-token"
import { expect } from "chai";
import { BN } from "bn.js";

describe("stablesale", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  const connection = provider.connection;
  anchor.setProvider(provider);

  const program = anchor.workspace.Stablesale as Program<Stablesale>;

  const payer = Keypair.generate();

  let mint: PublicKey;
  let usdt: PublicKey;
  let vault_authority: PublicKey;
  let mint_account: PublicKey;
  let usdt_account: PublicKey;
  let app_state: PublicKey;




  before("Init Balances", async () => {
    await connection.confirmTransaction(await connection.requestAirdrop(payer.publicKey, 100 * LAMPORTS_PER_SOL))
    mint = await createMint(connection, payer, payer.publicKey, undefined, 6);
    usdt = await createMint(connection, payer, payer.publicKey, undefined, 6);
    [vault_authority] = PublicKey.findProgramAddressSync([anchor.utils.bytes.utf8.encode("vault_authority")], program.programId);
    mint_account = getAssociatedTokenAddressSync(mint, vault_authority, true);
    usdt_account = getAssociatedTokenAddressSync(usdt, vault_authority, true);
    [app_state] = PublicKey.findProgramAddressSync([], program.programId);
    console.log(await connection.getBalance(payer.publicKey))
  })

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize().accounts({
      payer: payer.publicKey,
      mint,
      usdt,
      vaultAuthority: vault_authority,
      mintAccount: mint_account,
      usdtAccount: usdt_account,
      appState: app_state,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      systemProgram: SystemProgram.programId
    }).signers([payer])
      .rpc();
    console.log("Your transaction signature", tx);

    
    expect((await program.account.appState.fetch(app_state)).owner.equals((payer.publicKey)))
  });

  it("Test Purchase", async () => {
    const buyer = Keypair.generate();
    const receive_mint_account = await createAccount(connection, payer, mint, buyer.publicKey)
    const from_usdt_account = await createAccount(connection, payer, usdt, buyer.publicKey)

    await provider.sendAndConfirm(new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: payer.publicKey,
        toPubkey: buyer.publicKey,
        lamports: LAMPORTS_PER_SOL * 10
      }),
    ), [payer])

    await mintTo(connection, payer, usdt, from_usdt_account, payer, 1000000);
    await mintTo(connection, payer, mint, mint_account, payer, 1000000);

    await program.methods.purchase(new BN(1000000)).accounts({
      fromUsdtAccount: from_usdt_account,
      mint: mint,
      mintAccount: mint_account,
      payer: buyer.publicKey,
      receiveMintAccount: receive_mint_account,
      usdt: usdt,
      usdtAccount: usdt_account,
      vaultAuthority: vault_authority,
    }).signers([buyer]).rpc();

    console.log((await getAccount(connection, usdt_account)).amount)
  }) 

  it("Withdraw", async () => {
    const to_usdt_account = await createAccount(connection, payer, usdt, payer.publicKey)
    await program.methods.withdraw().accounts({
      appState: app_state,
      payer: payer.publicKey,
      toUsdtAccount: to_usdt_account,
      vaultAuthority: vault_authority,
      usdt,
      usdtAccount: usdt_account,
    }).signers([payer]).rpc()

    // expect((await getAccount(connection, to_usdt_account)).amount).eq(new BN(1000000))
  })
});
