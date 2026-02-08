// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const concepts = [
	'concepts',
	'concepts/myso-for-ethereum',
	'concepts/myso-for-solana',
	{
		type: 'category',
		label: 'Architecture',
		link: {
			type: 'doc',
			id: 'concepts/architecture',
		},
		items: [
			'concepts/myso-architecture/networks',
			'concepts/myso-architecture/myso-storage',
			'concepts/myso-architecture/consensus',
			'concepts/myso-architecture/epochs',
			'concepts/myso-architecture/myso-security',
			'concepts/myso-architecture/protocol-upgrades',
		],
	},
	{
		type: 'category',
		label: 'Transactions',
		items: [
			'concepts/transactions/transaction-lifecycle',
			'concepts/transactions/inputs-and-results',
			'concepts/transactions/gas-smashing',
			'concepts/transactions/transaction-auth',
		],
	},
	{
		type: 'category',
		label: 'Tokenomics',
		link: {
			type: 'doc',
			id: 'concepts/tokenomics',
		},
		items: [
			'concepts/tokenomics/staking-unstaking',
			'concepts/tokenomics/myso-bridging',
			'concepts/tokenomics/gas-in-myso',
		],
	},
	'concepts/coin-mgt',

	{
		type: 'category',
		label: 'Move',
		link: {
			type: 'doc',
			id: 'concepts/myso-move-concepts',
		},
		items: [
			'concepts/myso-move-concepts/packages',
			'concepts/myso-move-concepts/conventions',
			'concepts/myso-move-concepts/move-2024-migration',
		],
	},
	{
		type: 'category',
		label: 'Accessing Data',
		link: {
			type: 'doc',
			id: 'concepts/data-access/data-serving',
		},
		items: [
			'concepts/data-access/grpc',
			'concepts/data-access/graphql-indexer',
			'concepts/data-access/graphql-rpc',
		],
	},
	{
		type: 'category',
		label: 'Cryptography',
		link: {
			type: 'doc',
			id: 'concepts/cryptography',
		},
		items: [
			'concepts/cryptography/zklogin',
			'concepts/cryptography/passkeys',
			'concepts/cryptography/nautilus/nautilus-design',
			'concepts/cryptography/system/checkpoint-verification',
			/*{
				type: 'category',
				label: 'System',
				link: {
					type: 'doc',
					id: 'concepts/cryptography/system',
				},
				items: [
					'concepts/cryptography/system/validator-signatures',
					'concepts/cryptography/system/intents-for-validation',
				],
			},*/
		],
	},
	'concepts/gaming',
	'concepts/research-papers',
];
export default concepts;
