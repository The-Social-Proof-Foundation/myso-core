// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { MySoObjectRef } from '@mysten/myso/client';
import { getFaucetHost, requestMySoFromFaucetV1 } from '@mysten/myso/faucet';
import { Ed25519Keypair } from '@mysten/myso/keypairs/ed25519';
import { Transaction } from '@mysten/myso/transactions';

import { client } from './rpc';

// This simulates what a server would do to sponsor a transaction
export async function sponsorTransaction(sender: string, transactionKindBytes: Uint8Array) {
	// Rather than do gas pool management, we just spin out a new keypair to sponsor the transaction with:
	const keypair = new Ed25519Keypair();
	const address = keypair.getPublicKey().toMySoAddress();
	console.log(`Sponsor address: ${address}`);

	await requestMySoFromFaucetV1({ recipient: address, host: getFaucetHost('testnet') });

	let payment: MySoObjectRef[] = [];
	let retires = 50;
	while (retires !== 0) {
		const coins = await client.getCoins({ owner: address, limit: 1 });
		if (coins.data.length > 0) {
			payment = coins.data.map((coin) => ({
				objectId: coin.coinObjectId,
				version: coin.version,
				digest: coin.digest,
			}));
			break;
		}
		await new Promise((resolve) => setTimeout(resolve, 200)); // Sleep for 200ms
		retires -= 1;
	}

	const tx = Transaction.fromKind(transactionKindBytes);
	tx.setSender(sender);
	tx.setGasOwner(address);
	tx.setGasPayment(payment);

	return keypair.signTransaction(await tx.build({ client }));
}
