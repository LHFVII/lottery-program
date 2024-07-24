import { startAnchor } from 'solana-bankrun';
import { BankrunProvider } from 'anchor-bankrun';
import { PublicKey } from "@solana/web3.js";
import { Program } from "@coral-xyz/anchor";
import { Lottery } from "../target/types/lottery";
import * as anchor from "@coral-xyz/anchor";
import { expect } from 'chai';
import { createMint } from './utils';



const IDL = require("../target/idl/lottery.json");

describe("Create a system account", async () => {
    
    it("Initialize instruction Success", async () => {
        const programId = PublicKey.unique()
        const context = await startAnchor("",[{name:"lottery", programId: programId}],[])
        const banksClient = context.banksClient;
        const provider = new BankrunProvider(context);
        const puppetProgram = new Program<Lottery>(IDL, provider);

        const authority = anchor.web3.Keypair.generate();
        const mint = await createMint(banksClient, provider.wallet.payer, authority.publicKey, authority.publicKey, 9);

        await puppetProgram.methods.initialize(
            new anchor.BN(0),
            new anchor.BN(1821707382),
            new anchor.BN(1000000000),
        )
        .accounts({tokenProgram: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA', mint: mint})
        .rpc()
        
        const [lotteryAddress] = PublicKey.findProgramAddressSync([Buffer.from("token_lottery_config")], puppetProgram.programId);
        
        const lotteryConfig = await puppetProgram.account.tokenLottery.fetch(lotteryAddress);
        
        console.log(lotteryConfig)
    });
});

