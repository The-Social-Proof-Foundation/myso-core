/*
// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0
*/

//
// This code exports the tabs-md.client.js code
// as a plugin that Docusaurus can use

export default function tabsMdClient() {
  return {
    name: 'tabs-md-client',
    getClientModules() {
      return [require.resolve('../../js/tabs-md.client.js')];
    },
  };
}
