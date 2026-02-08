// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getFullnodeUrl, MySoClient } from '@mysten/myso/client';

export const client = new MySoClient({ url: getFullnodeUrl('testnet') });
