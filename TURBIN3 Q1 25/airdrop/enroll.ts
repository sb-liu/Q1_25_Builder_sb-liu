import { AnchorProvider, Program, Wallet } from "@coral-xyz/anchor";
import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { IDL, Turbin3Prereq } from "./programs/Turbin3_prereq";

import wallet from "./Turbin3-wallet.json";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

// Create a devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Github account
const github = Buffer.from("sb-liu", "utf8");

// Create our anchor provider
const provider = new AnchorProvider(connection, new Wallet(keypair), { commitment: "confirmed" });

// Create our program
const program : Program<Turbin3Prereq> = new Program(IDL, provider);

// Create the PDA for our enrollment account
const enrollment_seeds = [Buffer.from("prereq"), keypair.publicKey.toBuffer()];
const [enrollment_key, _bump] = PublicKey.findProgramAddressSync(enrollment_seeds, program.programId);

// Execute our enrollment transaction
(async () => {
    try {
        const txhash = await program.methods
            .complete(github)
            .accounts({
                signer: keypair.publicKey,
            })
            .signers([keypair])
            .rpc();
        console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

// Success TX
// https://explorer.solana.com/tx/35bm6zrTy4DTVZoCCNCG244CqmM4iFiRwXqJuPQ9CP7veMMwg1kUa5p2JuKhH6ZccqSykbSAeZ2QsZ8ppwe3as3X?cluster=devnet