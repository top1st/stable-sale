import * as anchor from "@coral-xyz/anchor"
import { Program, AnchorProvider, Wallet } from "@coral-xyz/anchor"
import {Stablesale} from "../target/types/stablesale"
import IDL from "../target/idl/stablesale.json"
import {Connection, clusterApiUrl, Keypair, PublicKey, SystemProgram} from "@solana/web3.js"
import "dotenv/config"
import {mkdirSync, existsSync, writeFileSync} from 'fs'
import { ASSOCIATED_TOKEN_PROGRAM_ID, TOKEN_PROGRAM_ID, createAssociatedTokenAccount, createMint, getAssociatedTokenAddressSync, getOrCreateAssociatedTokenAccount, mintTo } from "@solana/spl-token"
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes"
import config from "../app/config/config.json"
import { BN } from "bn.js"

const connection = new Connection(clusterApiUrl("testnet"))

const keypair = Keypair.fromSecretKey(bs58.decode(process.env.ADMIN_KEY))


async function main() {
    const admin_token = await getOrCreateAssociatedTokenAccount(connection, keypair, new PublicKey(config.mint), keypair.publicKey)
    const admin_usdt = await getOrCreateAssociatedTokenAccount(connection, keypair, new PublicKey(config.usdt), keypair.publicKey)
    
    await mintTo(connection, keypair, new PublicKey(config.usdt), admin_usdt.address, keypair, 1000_000_000)
    await mintTo(connection, keypair, new PublicKey(config.mint), admin_token.address, keypair, 1000_000_000)
    
    
}

main()
