import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { VibeVault } from "../target/types/vibe_vault";
import { TOKEN_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import { publicKey } from "@coral-xyz/anchor/dist/cjs/utils";
import {
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import {
  createAssociatedTokenAccountIdempotentInstruction,
  createInitializeMint2Instruction,
  createMintToInstruction,
  getAssociatedTokenAddressSync,
  getMinimumBalanceForRentExemptMint,
  MINT_SIZE,
} from "@solana/spl-token";
import { BN } from "@coral-xyz/anchor";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";


function msleep(ms: number): Promise<void> {
  return new Promise((resolve) => {
    setTimeout(resolve, ms);
  });
}
describe("vibe-vault", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.VibeVault as Program<VibeVault>;
  const provider = anchor.getProvider() as anchor.AnchorProvider;
  const wallet = provider.wallet as NodeWallet;
  const connection = provider.connection;

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block,
    });
    return signature;
  };

  const DECIMALS = 8;

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  };

  const mint = Keypair.generate();
  const userAta = getAssociatedTokenAddressSync(
    mint.publicKey,
    provider.publicKey,
    false,
    TOKEN_PROGRAM_ID
  );

  const accounts = {};
  it("mint", async () => {
    let tx = new Transaction();
    let lamports = await getMinimumBalanceForRentExemptMint(connection);
    tx.instructions = [
      SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: mint.publicKey,
        lamports,
        space: MINT_SIZE,
        programId: TOKEN_PROGRAM_ID,
      }),
      createInitializeMint2Instruction(
        mint.publicKey,
        DECIMALS,
        provider.publicKey,
        null,
        TOKEN_PROGRAM_ID
      ),
      createAssociatedTokenAccountIdempotentInstruction(
        provider.publicKey,
        userAta,
        provider.publicKey,
        mint.publicKey,
        TOKEN_PROGRAM_ID
      ),
      createMintToInstruction(
        mint.publicKey,
        userAta,
        provider.publicKey,
        1e8,
        undefined,
        TOKEN_PROGRAM_ID
      ),
    ];
    await provider.sendAndConfirm(tx, [wallet.payer, mint]).then(log);
  });
  it("Initialize vault", async () => {
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        mint: mint.publicKey,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();
    console.log("Your transaction signature", tx);
  });
  it("Stake", async () => {
    try {
      await program.methods
        .stake(new BN(1))
        .accounts({ mint: mint.publicKey, tokenProgram: TOKEN_PROGRAM_ID })
        .signers([])
        .rpc()
        .then(confirm)
        .then(log);
    } catch (e) {
      console.log(e);
      throw e;
    }
  });
  it("Unstake", async () => {
    try {
      await msleep(3000);
      await program.methods
        .unstake(new BN(1))
        .accounts({ mint: mint.publicKey, tokenProgram: TOKEN_PROGRAM_ID })
        .signers([])
        .rpc()
        .then(confirm)
        .then(log);
    } catch (e) {
      console.log(e);
      throw e;
    }
  });
});
