import bs58 from 'bs58';

// Converting base58ToWallet test function
async function base58ToWallet() {
    try {
        // Decode the base58 string to a buffer (equivalent to Vec<u8> in Rust)
        const wallet = bs58.decode("")
        console.log(wallet)
    } catch (error) {
        console.error("Error decoding base58:", error)
    }
}

// Converting walletToBase58 test function
function walletToBase58() {
    // Create the same byte array as in the Rust code
    const wallet = new Uint8Array([])

    // Encode the wallet bytes to base58
    const base58 = bs58.encode(wallet)
    console.log(base58)
}

base58ToWallet()