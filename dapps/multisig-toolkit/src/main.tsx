// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

import '@mysten/dapp-kit/dist/index.css';
import './index.css';
import '@fontsource-variable/inter';
import '@fontsource-variable/red-hat-mono';

import { MySoClientProvider, WalletProvider } from '@mysten/dapp-kit';
import { getFullnodeUrl } from '@socialproof/myso/client';
import { QueryClientProvider } from '@tanstack/react-query';
import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';

import { queryClient } from './lib/queryClient';
import { router } from './routes';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<React.StrictMode>
		<QueryClientProvider client={queryClient}>
			<MySoClientProvider
				defaultNetwork="myso:mainnet"
				networks={{
					'myso:testnet': { url: getFullnodeUrl('testnet') },
					'myso:mainnet': { url: getFullnodeUrl('mainnet') },
					'myso:devnet': { url: getFullnodeUrl('devnet') },
				}}
			>
				<WalletProvider>
					<RouterProvider router={router} />
				</WalletProvider>
			</MySoClientProvider>
		</QueryClientProvider>
	</React.StrictMode>,
);
