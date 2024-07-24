import { startAnchor, BanksClient, BanksTransactionMeta, ProgramTestContext} from 'solana-bankrun';
import { SystemProgram, Signer, PublicKey, Keypair, Transaction, Commitment, ConfirmOptions, AccountInfo } from "@solana/web3.js";
import * as token from "@solana/spl-token";

export async function createAssociatedTokenAccount(
    banksClient: BanksClient,
    payer: Signer,
    mint: PublicKey,
    owner: PublicKey,
    programId = token.TOKEN_PROGRAM_ID,
    associatedTokenProgramId = token.ASSOCIATED_TOKEN_PROGRAM_ID
  ): Promise<PublicKey> {
    const associatedToken = token.getAssociatedTokenAddressSync(
      mint,
      owner,
      true,
      programId,
      associatedTokenProgramId
    );
  
    const tx = new Transaction().add(
      token.createAssociatedTokenAccountInstruction(
        payer.publicKey,
        associatedToken,
        owner,
        mint,
        programId,
        associatedTokenProgramId
      )
    );
  
    [tx.recentBlockhash] = (await banksClient.getLatestBlockhash())!;
    tx.sign(payer);
  
    await banksClient.processTransaction(tx);
  
    return associatedToken;
  }

  export async function createMint(
    banksClient: BanksClient,
    payer: Keypair,
    mintAuthority: PublicKey,
    freezeAuthority: PublicKey | null,
    decimals: number,
    keypair = Keypair.generate(),
    programId = token.TOKEN_PROGRAM_ID
  ): Promise<PublicKey> {
    let rent = await banksClient.getRent();
  
    const tx = new Transaction().add(
      SystemProgram.createAccount({
        fromPubkey: payer.publicKey,
        newAccountPubkey: keypair.publicKey,
        space: token.MINT_SIZE,
        lamports: Number(await rent.minimumBalance(BigInt(token.MINT_SIZE))),
        programId: token.TOKEN_PROGRAM_ID,
      }),
      token.createInitializeMint2Instruction(
        keypair.publicKey,
        decimals,
        mintAuthority,
        freezeAuthority,
        programId
      )
    );
    [tx.recentBlockhash] = (await banksClient.getLatestBlockhash())!;
    tx.sign(payer, keypair);
  
    await banksClient.processTransaction(tx);
  
    return keypair.publicKey;
  }