import { Connection, Keypair, PublicKey, sendAndConfirmTransaction, SystemProgram, Transaction } from "@solana/web3.js";
import wallet from "./dev-wallet.json";

// Import our dev wallet keypair from the wallet file
const from = Keypair.fromSecretKey(new Uint8Array(wallet));

// Define our Turbin3 public key
const to = new PublicKey("8NJ59grUmY3xHC9e3wuXUoW6qBbVRcVSXe88u66dLzRF");

//Create a Solana devnet connection
const connection = new Connection("https://api.devnet.solana.com");

// Move some SOL to the dev wallet
// (async () => {
//     try {
//         const transaction = new Transaction().add(
//             SystemProgram.transfer({
//             fromPubkey: from.publicKey,
//             toPubkey: to,
//             lamports: LAMPORTS_PER_SOL/10,
//             })
//         );
//         transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
//         transaction.feePayer = from.publicKey;
//         // Sign transaction, broadcast, and confirm
//         const signature = await sendAndConfirmTransaction(
//             connection,
//             transaction,
//             [from]
//         );
//         console.log(`Success! Check out your TX here:
//         https://explorer.solana.com/tx/${signature}?cluster=devnet`);
//     } catch(e) {
//         console.error(`Oops, something went wrong: ${e}`)
//     }
// })();

// Success TX 0.1 SOL
// https://explorer.solana.com/tx/V4Fs6TtsPow5pkVQqzefeLDkx4BUi6qXLGCaS6nGwjpvJt6eAs1CL22AX5GnVpjnN4pbY8JKgz8sem2h7sR6Ab1?cluster=devnet


// Move all SOL out of the dev wallet
(async () => {
    try {
    // Get balance of dev wallet
    const balance = await connection.getBalance(from.publicKey)
    // Create a test transaction to calculate fees
    const transaction = new Transaction().add(
        SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance,
        })
    );
    transaction.recentBlockhash = (await connection.getLatestBlockhash('confirmed')).blockhash;
    transaction.feePayer = from.publicKey;

    // Calculate exact fee rate to transfer entire SOL amount out of account minus fees
    const fee = (await connection.getFeeForMessage(transaction.compileMessage(),'confirmed')).value || 0;

    // Remove our transfer instruction to replace it
    transaction.instructions.pop();

    // Now add the instruction back with correct amount of lamports
    transaction.add(
        SystemProgram.transfer({
            fromPubkey: from.publicKey,
            toPubkey: to,
            lamports: balance - fee,
        })
    );
    // Sign transaction, broadcast, and confirm
    const signature = await sendAndConfirmTransaction(connection, transaction, [from]);

    console.log(`Success! Check out your TX here: https://explorer.solana.com/tx/${signature}?cluster=devnet`)
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

// Success TX Rest of SOL
// https://explorer.solana.com/tx/4uX4apNyNk1ojXDLb8iRwVmJVLVQ5jMtX6gJTHHRCxadiT1RM67qVsJdTwLbpYJuR6hMqFg9mChFHy57bz3acXu1?cluster=devnet