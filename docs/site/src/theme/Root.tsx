// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

import React from 'react';
import GlossaryProvider from '@site/src/shared/components/Glossary/GlossaryProvider';

export default function Root({ children }: { children: React.ReactNode }) {
  return <GlossaryProvider>{children}</GlossaryProvider>;
}