
import { FC, useState } from 'react';
import { useConnection, useAnchorWallet } from '@solana/wallet-adapter-react';
import * as web3 from '@solana/web3.js';
import { getProgram } from '../../utils/anchor';
import {
    getAssociatedTokenAddressSync,
    TOKEN_PROGRAM_ID,
    ASSOCIATED_TOKEN_PROGRAM_ID
} from "@solana/spl-token";

export const RefundForm: FC = () => {
    const { connection } = useConnection();
    const wallet = useAnchorWallet();
    const [escrowAddress, setEscrowAddress] = useState('');

    const handleRefund = async () => {
        if (!wallet || !escrowAddress) return;

        try {
            const program = getProgram(connection, wallet);
            const escrowPubkey = new web3.PublicKey(escrowAddress);

            // Fetch Escrow Account Data
            const escrowAccount = await program.account.escrow.fetch(escrowPubkey);

            const mintA = escrowAccount.mintA;

            // Re-derive Vault PDA
            const vault = getAssociatedTokenAddressSync(
                mintA,
                escrowPubkey,
                true
            );

            // Maker ATA A (to receive refund)
            const makerAtaA = getAssociatedTokenAddressSync(
                mintA,
                wallet.publicKey
            );

            const tx = await program.methods
                .refund()
                .accounts({
                    maker: wallet.publicKey,
                    escrow: escrowPubkey,
                    mintA: mintA,
                    vault: vault,
                    makerAtaA: makerAtaA,
                    associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
                    tokenProgram: TOKEN_PROGRAM_ID,
                    systemProgram: web3.SystemProgram.programId,
                })
                .rpc();

            console.log('Transaction signature', tx);
            alert(`退款成功! 交易哈希: ${tx}`);

        } catch (error) {
            console.error(error);
            alert('退款失败: ' + error);
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
            <button onClick={handleRefund} disabled={!wallet}>
                执行退款 (Refund)
            </button>
        </div>
    );
};
