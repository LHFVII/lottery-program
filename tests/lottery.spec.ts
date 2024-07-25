import { startAnchor } from 'solana-bankrun';
import { BankrunProvider } from 'anchor-bankrun';
import { PublicKey } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { Lottery } from "../target/types/lottery";
import * as anchor from "@coral-xyz/anchor";
import { expect } from 'chai';
import { createAccount, createMint } from './utils';



const IDL = require("../target/idl/lottery.json");

describe("Create a system account", async () => {

    const TOKEN_METADATA_PROGRAM_ID = new anchor.web3.PublicKey(
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"
      );
    
    it("Initialize instruction Success", async () => {
        const programId = PublicKey.unique()
        const context = await startAnchor("",[{name:"lottery", programId: programId}],[])
        const banksClient = context.banksClient;
        const provider = new BankrunProvider(context);
        const puppetProgram = new Program<Lottery>(IDL, provider);
        const payer = provider.wallet.payer;

        const authority = anchor.web3.Keypair.generate();
        const mint = await createMint(banksClient, provider.wallet.payer, authority.publicKey, authority.publicKey, 9);
        await createAccount(banksClient,payer, mint, authority.publicKey),
        

        await puppetProgram.methods.initialize(
            new anchor.BN(0),
            new anchor.BN(1821707382),
            new anchor.BN(1000000000),
        )
        .accounts({tokenProgram: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA', mint: mint})
        .rpc()
        
        const [lotteryAddress] = PublicKey.findProgramAddressSync([Buffer.from("token_lottery_config")], puppetProgram.programId);
        //const [collectionMintAddress] = PublicKey.findProgramAddressSync([Buffer.from(lotteryAddress.toString())], puppetProgram.programId);
        
        const lotteryConfig = await puppetProgram.account.tokenLottery.fetch(lotteryAddress);
        //const collectionMint = await puppetProgram.account.tokenLottery.fetch(collectionMintAddress);
        
        console.log(lotteryConfig)
    });
});

