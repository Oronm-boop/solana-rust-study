
import { FC, useState } from 'react';
import { useConnection, useAnchorWallet } from '@solana/wallet-adapter-react';
import * as web3 from '@solana/web3.js';
import { getProgram } from '../../utils/anchor';
import {
    getAssociatedTokenAddressSync,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";

export const TakeForm: FC = () => {
    const { connection } = useConnection();
    const wallet = useAnchorWallet();
    const [escrowAddress, setEscrowAddress] = useState('');

    const handleTake = async () => {
        if (!wallet || !escrowAddress) return;

        try {
            const program = getProgram(connection, wallet);
            const escrowPubkey = new web3.PublicKey(escrowAddress);

            // Fetch Escrow Account Data to get mints and maker
            const escrowAccount = await program.account.escrow.fetch(escrowPubkey);

            const mintA = escrowAccount.mintA;
            const mintB = escrowAccount.mintB;
            const maker = escrowAccount.maker;

            const vault = getAssociatedTokenAddressSync(
                mintA,
                escrowPubkey,
                true
            );

            const takerAtaA = getAssociatedTokenAddressSync(
                mintA,
                wallet.publicKey
            );

            const takerAtaB = getAssociatedTokenAddressSync(
                mintB,
                wallet.publicKey
            );

            const makerAtaB = getAssociatedTokenAddressSync(
                mintB,
                maker
            );

            const tx = await program.methods
                .take()
                .accounts({
                    taker: wallet.publicKey,
                    maker: maker,
                    escrow: escrowPubkey,
                    mintA: mintA,
                    mintB: mintB,
                    vault: vault,
                    takerAtaA: takerAtaA,
                    takerAtaB: takerAtaB,
                    makerAtaB: makerAtaB,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: web3.SystemProgram.programId,
                })
                .rpc();

            console.log('Transaction signature', tx);
            alert(`订单完成! 交易哈希: ${tx}`);

        } catch (error) {
            console.error(error);
            alert('交易失败: ' + error);
        }
    };

    return (
        <div className="form-container">
            <div className="input-group">
                <input
                    type="text"
                    placeholder="Escrow 账户地址 (Public Key)"
                    value={escrowAddress}
                    onChange={(e) => setEscrowAddress(e.target.value)}
                />
            </div>
            <button onClick={handleTake} disabled={!wallet}>
                完成订单 (Take)
            </button>
        </div>
    );
};
