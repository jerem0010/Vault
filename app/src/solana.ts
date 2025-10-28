import { Connection } from "@solana/web3.js";

// Connecte à la blockchain (ici devnet pour test)
const connection = new Connection("https://api.devnet.solana.com");

export async function deposit(wallet: string, amount: number) {
  console.log(`💸 Depositing ${amount} for wallet ${wallet}`);
  // Ici tu pourras plus tard envoyer une vraie tx Anchor
  return "mock-tx-signature";
}

export async function getBalance(wallet: string) {
  // Simule une lecture on-chain
  return 100 + Math.floor(Math.random() * 50);
}
