
import { FC, useState } from 'react';
import { useConnection, useAnchorWallet } from '@solana/wallet-adapter-react';
import * as web3 from '@solana/web3.js';
import { BN, utils } from '@coral-xyz/anchor';
import { getProgram } from '../../utils/anchor';
import {
    getAssociatedTokenAddressSync,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";

export const MakeForm: FC = () => {
    const { connection } = useConnection();
    const wallet = useAnchorWallet();

    const [seed, setSeed] = useState('');
    const [receiveAmount, setReceiveAmount] = useState('');
    const [depositAmount, setDepositAmount] = useState('');
    const [tokenMintA, setTokenMintA] = useState('');
    const [tokenMintB, setTokenMintB] = useState('');

    const handleMake = async () => {
        if (!wallet || !seed || !receiveAmount || !depositAmount || !tokenMintA || !tokenMintB) return;

        try {
            const program = getProgram(connection, wallet);

            const seedBN = new BN(seed);
            const receiveBN = new BN(receiveAmount);
            const depositBN = new BN(depositAmount);

            const mintA = new web3.PublicKey(tokenMintA);
            const mintB = new web3.PublicKey(tokenMintB);

            // PDA for Escrow State
            const [escrowPda] = web3.PublicKey.findProgramAddressSync(
                [
                    Buffer.from("escrow"),
                    wallet.publicKey.toBuffer(),
                    seedBN.toArrayLike(Buffer, "le", 8)
                ],
                program.programId
            );

            const vault = getAssociatedTokenAddressSync(
                mintA,
                escrowPda,
                true
            );

            const makerAtaA = getAssociatedTokenAddressSync(
                mintA,
                wallet.publicKey
            );

            const tx = await program.methods
                .make(seedBN, receiveBN, depositBN)
                .accounts({
                    maker: wallet.publicKey,
                    escrow: escrowPda,
                    mintA: mintA,
                    mintB: mintB,
                    makerAtaA: makerAtaA,
                    vault: vault,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: web3.SystemProgram.programId,
                })
                .rpc();

            console.log('Transaction signature', tx);
            alert(`担保订单创建成功! 交易哈希: ${tx}`);

        } catch (error) {
            console.error(error);
            alert('创建失败: ' + error);
        }
    };

    return (
        <div className="form-container">
            <div className="input-group">
                <input
                    type="number"
                    placeholder="种子 (Seed / Unique ID)"
                    value={seed}
                    onChange={(e) => setSeed(e.target.value)}
                />
            </div>
            <div className="input-group">
                <input
                    type="text"
                    placeholder="存入代币地址 (Mint A)"
                    value={tokenMintA}
                    onChange={(e) => setTokenMintA(e.target.value)}
                />
            </div>
            <div className="input-group">
                <input
                    type="number"
                    placeholder="存入数量 (Amount A)"
                    value={depositAmount}
                    onChange={(e) => setDepositAmount(e.target.value)}
                />
            </div>
            <div className="input-group">
                <input
                    type="text"
                    placeholder="接收代币地址 (Mint B)"
                    value={tokenMintB}
                    onChange={(e) => setTokenMintB(e.target.value)}
                />
            </div>
            <div className="input-group">
                <input
                    type="number"
                    placeholder="接收数量 (Amount B)"
                    value={receiveAmount}
                    onChange={(e) => setReceiveAmount(e.target.value)}
                />
            </div>
            <button onClick={handleMake} disabled={!wallet}>
                创建担保订单
            </button>
        </div>
    );
};
