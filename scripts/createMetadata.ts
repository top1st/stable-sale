import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes"
import { bundlrStorage, keypairIdentity, Metaplex } from "@metaplex-foundation/js"
import { createCreateMetadataAccountV2Instruction, createCreateMetadataAccountV3Instruction, DataV2, PROGRAM_ADDRESS as TOKEN_METADATA_PROGRAM_ID } from "@metaplex-foundation/mpl-token-metadata"
import { Connection, clusterApiUrl, Keypair, PublicKey, SystemProgram, Transaction, VersionedTransaction, sendAndConfirmTransaction } from "@solana/web3.js"
import "dotenv/config"

const connection = new Connection(clusterApiUrl("testnet"))

const payer = Keypair.fromSecretKey(bs58.decode(process.env.ADMIN_KEY))
const metaplex = Metaplex.make(connection).use(keypairIdentity(payer)).use(bundlrStorage(
    { address: "https://devnet.bundlr.network", providerUrl: "https://api.devnet.solana.com", timeout: 60000, }
))



const createMetadataAccount = async (metadataPDA: PublicKey, mint: PublicKey, payer: PublicKey, data: DataV2) => {
    const metadataPDA1 = metaplex.nfts().pdas().metadata({ mint });
    console.log(metadataPDA.equals(metadataPDA1))
    const tx = new Transaction().add(
        createCreateMetadataAccountV3Instruction({
            metadata: metadataPDA,
            mint: mint,
            mintAuthority: payer,
            payer: payer,
            updateAuthority: payer,
        }, 
        {
            createMetadataAccountArgsV3: {
                data,
                isMutable: true,
                collectionDetails: null
            },
        }
        ))
    return tx;
}

const getMetadata = (
    mint: PublicKey,
): PublicKey => {
    return (
        PublicKey.findProgramAddressSync(
            [
                Buffer.from('metadata'),
                new PublicKey(TOKEN_METADATA_PROGRAM_ID).toBuffer(),
                mint.toBuffer(),
            ],
            new PublicKey(TOKEN_METADATA_PROGRAM_ID),
        )
    )[0];
};

const addMetadata = async (mintAddress: PublicKey, connection: Connection) => {
    console.log(await connection.getBalance(payer.publicKey))
    const upload = await metaplex.nfts().uploadMetadata({
        name: "Test Token",
        symbol: "Test",
    })

    const metadataData: DataV2 = {
        name: "SCFG-Test",
        symbol: "SCFG-T",
        uri: upload.uri, // Arweave URI link which uses metaplex standard
        sellerFeeBasisPoints: 0,
        creators: null,
        collection: null,
        uses: null
    };
    // const payer = Keypair.fromSecretKey(new Uint8Array(JSON.parse(fs.readFileSync(keypair, { encoding: "utf8" }))));
    
    const metadataAccount = getMetadata(mintAddress);
    const tx = await createMetadataAccount(metadataAccount, mintAddress, payer.publicKey, metadataData);
    
    tx.recentBlockhash = ((await connection.getLatestBlockhash()).blockhash);
    tx.feePayer = payer.publicKey;
    const transactionId = await sendAndConfirmTransaction(
        connection,
        tx, [payer,payer], {skipPreflight: true});

    console.log(transactionId)
}

async function main() {
    await addMetadata(new PublicKey('GPkwnhyWxcAPmzvBPxmiWntTHyYMJmvuJ5RuxbA4YBcE'), connection)
}

main()
