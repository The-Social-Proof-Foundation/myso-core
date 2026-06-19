// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

const standards = [
	'standards',
	'standards/coin',
	'standards/currency',
	{
		type: 'category',
		label: 'Closed-Loop Token',
		link: {
			type: 'doc',
			id: 'standards/closed-loop-token',
		},
		items: [
			'standards/closed-loop-token/action-request',
			'standards/closed-loop-token/token-policy',
			'standards/closed-loop-token/spending',
			'standards/closed-loop-token/rules',
			'standards/closed-loop-token/coin-token-comparison',
		],
	},
	'standards/kiosk',
	'standards/kiosk-apps',
	{
		type: 'category',
		label: 'OrderbookV3',
		link: {
			type: 'doc',
			id: 'standards/orderbook',
		},
		items: [
			'standards/orderbookv3/design',
			{
				type: 'category',
				label: 'Contract Information',
				link: {
					type: 'doc',
					id: 'standards/orderbookv3/contract-information',
				},
				items: [
					'standards/orderbookv3/contract-information/balance-manager',
					'standards/orderbookv3/contract-information/permissionless-pool',
					'standards/orderbookv3/contract-information/query-the-pool',
					'standards/orderbookv3/contract-information/orders',
					'standards/orderbookv3/contract-information/swaps',
					'standards/orderbookv3/contract-information/flash-loans',
					'standards/orderbookv3/contract-information/staking-governance',
					'standards/orderbookv3/contract-information/referral',
					'standards/orderbookv3/contract-information/ewma',
				],
			},
			'standards/orderbookv3-indexer',
			{
				type: 'category',
				label: 'SDK',
				link: {
					type: 'doc',
					id: 'standards/orderbookv3-sdk',
				},
				items: [
					'standards/orderbookv3-sdk/balance-manager',
					'standards/orderbookv3-sdk/pools',
					'standards/orderbookv3-sdk/orders',
					'standards/orderbookv3-sdk/swaps',
					'standards/orderbookv3-sdk/flash-loans',
					'standards/orderbookv3-sdk/staking-governance',
				],
			},
		],
	},
	{
		type: 'category',
		label: 'Orderbook Margin',
		link: {
			type: 'doc',
			id: 'standards/orderbook-margin',
		},
		items: [
			'standards/orderbook-margin/design',
			'standards/orderbook-margin/margin-risks',
			{
				type: 'category',
				label: 'Contract Information',
				link: {
					type: 'doc',
					id: 'standards/orderbook-margin/contract-information',
				},
				items: [
					'standards/orderbook-margin/contract-information/risk-ratio',
					'standards/orderbook-margin/contract-information/margin-manager',
					'standards/orderbook-margin/contract-information/margin-pool',
					'standards/orderbook-margin/contract-information/interest-rates',
					'standards/orderbook-margin/contract-information/orders',
					'standards/orderbook-margin/contract-information/tpsl',
					'standards/orderbook-margin/contract-information/supply-referral',
					'standards/orderbook-margin/contract-information/maintainer',
				],
			},
			'standards/orderbook-margin-indexer',
			{
				type: 'category',
				label: 'SDK',
				link: {
					type: 'doc',
					id: 'standards/orderbook-margin-sdk',
				},
				items: [
					'standards/orderbook-margin-sdk/margin-manager',
					'standards/orderbook-margin-sdk/margin-pool',
					'standards/orderbook-margin-sdk/orders',
					'standards/orderbook-margin-sdk/tpsl',
					'standards/orderbook-margin-sdk/maintainer',
				],
			},
		],
	},
	'standards/display',
	'standards/payment-kit',
	'standards/sagat',
	'standards/wallet-standard',
];
export default standards;
