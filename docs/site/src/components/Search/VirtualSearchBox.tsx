// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

import React from "react";
import { useSearchBox } from "react-instantsearch";

export default function VirtualSearchBox({ query }: { query: string }) {
  const { refine } = useSearchBox();
  React.useEffect(() => {
    refine(query);
  }, [query, refine]);
  return null;
}
