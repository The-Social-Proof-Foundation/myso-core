// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

import { useMySoClientContext, useMySoClientQuery, UseMySoClientQueryOptions } from "@mysten/dapp-kit";
import { GetObjectParams, MySoObjectResponse } from "@mysten/myso/client";
import { useQueryClient, UseQueryResult } from "@tanstack/react-query";

export type UseObjectQueryOptions = UseMySoClientQueryOptions<"getObject", MySoObjectResponse>;
export type UseObjectQueryResponse = UseQueryResult<MySoObjectResponse, Error>;
export type InvalidateUseObjectQuery = () => void;

/**
 * Fetches an object, returning the response from RPC and a callback
 * to invalidate it.
 */
export function useObjectQuery(
    params: GetObjectParams,
    options?: UseObjectQueryOptions,
): [UseObjectQueryResponse, InvalidateUseObjectQuery] {
    const ctx = useMySoClientContext();
    const client = useQueryClient();
    const response = useMySoClientQuery("getObject", params, options);

    const invalidate = async () => {
        await client.invalidateQueries({
            queryKey: [ctx.network, "getObject", params],
        });
    };

    return [response, invalidate];
}
