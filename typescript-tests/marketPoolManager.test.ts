import {
    Connection,
    Keypair,
    PublicKey,
    SystemProgram,
    Transaction,
    sendAndConfirmTransaction,
    TransactionInstruction,
} from '@solana/web3.js';
import { assert } from 'chai';
import { PROGRAM_ID, MARKET_POOL_PROGRAM_ID } from './constants'; // Assume constants are defined elsewhere

describe('Market Pool Manager Tests', () => {
    const connection = new Connection('http://localhost:8899', 'confirmed');
    let payer: Keypair;
    let marketId: PublicKey;

    before(async () => {
        payer = Keypair.generate();

        const airdropSignature = await connection.requestAirdrop(
            payer.publicKey,
            1e9 // 1 SOL
        );
        await connection.confirmTransaction(airdropSignature);

        marketId = Keypair.generate().publicKey;
    });

    it('should create a market pool', async () => {
        const programId = new PublicKey(PROGRAM_ID); // Your deployed program's ID
        const marketPoolProgramId = new PublicKey(MARKET_POOL_PROGRAM_ID); // ID for the market pool program

        const transaction = new Transaction().add(
            new TransactionInstruction({
                keys: [
                    { pubkey: payer.publicKey, isSigner: true, isWritable: true },
                    { pubkey: marketId, isSigner: false, isWritable: true },
                    // Add other necessary accounts here
                ],
                programId,
                data: Buffer.from(Uint8Array.of(0, ...new PublicKey(marketId).toBuffer())), // Assuming '0' is the instruction index for 'initialize_market_pool'
            })
        );

        const signature = await sendAndConfirmTransaction(connection, transaction, [payer]);
        console.log('Market pool created with transaction:', signature);

        assert.isNotNull(signature, 'Transaction signature should not be null');
    });
});