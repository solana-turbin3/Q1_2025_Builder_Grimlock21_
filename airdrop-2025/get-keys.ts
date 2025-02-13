import { Keypair } from "@solana/web3.js";

const keypair = Keypair.fromSecretKey(
    Uint8Array.from(
        [174,146,232,144,87,46,214,158,211,195,218,234,255,177,52,195,237,113,110,239,245,235,215,242,135,82,239,149,118,232,212,204,196,186,50,39,63,161,190,166,252,16,48,84,140,63,172,19,12,248,46,37,182,166,234,89,93,105,104,116,117,154,214,116]
    ),
);

console.log(keypair.publicKey);
console.log(keypair.secretKey);