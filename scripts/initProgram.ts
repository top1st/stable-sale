import * as anchor from "@coral-xyz/anchor"
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor"
import {Stablesale, IDL} from "../target/types/stablesale"
import {Connection, clusterApiUrl, Keypair, PublicKey, SystemProgram} from "@solana/web3.js"
import "dotenv/config"
import {mkdirSync, existsSync, writeFileSync} from 'fs'
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createMint, getAssociatedTokenAddressSync } from "@solana/spl-token"
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes"

const connection = new Connection(clusterApiUrl("mainnet-beta"))

const keypair = Keypair.fromSecretKey(bs58.decode(process.env.ADMIN_KEY))
const wallet = new Wallet(keypair)

const provider = new AnchorProvider(connection, wallet, { commitment: "processed" })

const program = new Program<Stablesale>(IDL, "CZCbBLPGeMx4GzGHjDhfthv1KwbujGeZYWfhtAtKkHdi", provider);

let mint: PublicKey = new PublicKey("22WSY62BTqvoDeahjs1RU9Ltfi8UsgeVpD2GRxD1dp92");
// let usdt: PublicKey;
let mint_account: PublicKey;
// let usdt_account: PublicKey;

const [app_state] = PublicKey.findProgramAddressSync([], program.programId);
const [vault_authority] = PublicKey.findProgramAddressSync([anchor.utils.bytes.utf8.encode("vault_authority")], program.programId);


async function main() {
    if(!existsSync(__dirname + "/../app/config")) {
        mkdirSync(__dirname + "/../app/config", {recursive: true})
    }

    {
        // mint = await createMint(connection, keypair, keypair.publicKey, undefined, 6);
        // usdt = await createMint(connection, keypair, keypair.publicKey, undefined, 6);
        mint_account = getAssociatedTokenAddressSync(mint, vault_authority, true);
        // usdt_account = getAssociatedTokenAddressSync(usdt, vault_authority, true);

        writeFileSync(__dirname + "/../app/config/config.json", JSON.stringify({
            owner: keypair.publicKey.toString(),
            mint: mint.toString(),
            // usdt: usdt.toString(),
            mint_account: mint_account.toString(),
            // usdt_account: usdt_account.toString(),
            app_state: app_state.toString(),
            vault_authority: vault_authority.toString()
        }))
    } 

    await program.methods.initialize().accounts({
        payer: keypair.publicKey,
        mint,
        vaultAuthority: vault_authority,
        mintAccount: mint_account,
        appState: app_state,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId
    }).signers([keypair]).rpc()
}

main()
