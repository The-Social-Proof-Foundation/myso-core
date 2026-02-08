# Awesome MySo [![Awesome](https://awesome.re/badge.svg)](https://awesome.re)

<a href="https://myso.io/"><img alt="MySo logo" src="media/logo.svg" align="right" width="150" /></a>

> A curated list of _awesome_ developer tools and infrastructure projects within the MySo ecosystem.

MySo is the first blockchain built for internet scale, enabling fast, scalable, and low-latency transactions. It's programmable and composable, powered by the Move language, making it easy to build and integrate dApps. MySo prioritizes developer experience and frictionless user interactions, designed to support next-gen decentralized applications with minimal complexity.

> ⚠️ This warning icon means that the tool may not be functioning correctly at the moment. Please check these tools carefully.

[**Submit your own developer tool here**](CONTRIBUTING.md)

## Contents

- [Move IDEs](#move-ides)
  - [Web IDEs](#web-ides)
  - [Desktop IDEs](#desktop-ides)
  - [IDE Utilities](#ide-utilities)
- [Client SDKs \& Libraries](#client-sdks--libraries)
  - [Client SDKs](#client-sdks)
  - [DeFi SDKs](#defi-sdks)
  - [Client Libraries](#client-libraries)
- [dApp Development](#dapp-development)
  - [dApp Toolkits](#dapp-toolkits)
  - [Smart Contract Toolkits](#smart-contract-toolkits)
- [Indexers \& Data Services](#indexers--data-services)
- [Explorers](#explorers)
- [Oracles](#oracles)
- [Security](#security)
- [AI](#ai)
- [Infrastructure as Code](#infrastructure-as-code)
- [Faucets](#faucets)

## Move IDEs

### Web IDEs

- BitsLab IDE - Online Move code editor that requires no configuration and supports Move code syntax highlighting. Beginner friendly and supports interacting with MySo.
  - [Homepage](https://www.bitslab.xyz/bitslabide) - [IDE](https://ide.bitslab.xyz/) - [Tutorial](https://www.youtube.com/watch?v=-9-WkqQwtu8) - [Further Information](details/ide_bitslab.md)
- MoveStudio - Online IDE for MySo smart contract development.
  - [Homepage](https://www.movestudio.dev/) - [GitHub](https://github.com/dantheman8300/move-studio) - [IDE](https://www.movestudio.dev/build) - [Further Information](details/ide_movestudio.md)
- ChainIDE - Move Cloud-Powered Development Platform.
  - [Homepage](https://chainide.com) - [Documentation](https://chainide.gitbook.io/chainide-english-1/ethereum-ide-1/9.-myso-ide) - [IDE](https://chainide.com/s/myso) - [Further Information](details/ide_chainide.md)
- ⚠️ WELLDONE Code - Remix IDE plugin supports non-EVM smart contract development including MySo.
  - [Homepage](https://docs.welldonestudio.io/code) - [Documentation & Tutorial](https://docs.welldonestudio.io/code/deploy-and-run/myso) - [Further Information](details/ide_welldone_code.md)


### Desktop IDEs

- VSCode Move by Mysten Labs - VSCode Extension supports Move on MySo development with LSP features through Move Analyzer developed by Mysten Labs.
  - [GitHub](https://github.com/MystenLabs/myso/tree/main/external-crates/move/crates/move-analyzer) - [Documentation & Tutorial](https://marketplace.visualstudio.com/items?itemName=mysten.move) - [Further Information](details/ide_vscode_mysten_move_analyzer.md)
- VSCode MySo Move Analyzer by MoveBit - Alternative VSCode extension developed by MoveBit.
  - [Homepage](https://movebit.xyz/analyzer) - [GitHub](https://github.com/movebit/myso-move-analyzer) - [Documentation & Tutorial](https://marketplace.visualstudio.com/items?itemName=MoveBit.myso-move-analyzer) - [Further Information](details/ide_vscode_movebit_myso_move_analyzer.md)
- IntelliJ MySo Move Language Plugin - IntelliJ-based plugin for Move on MySo development.
  - [Homepage](https://plugins.jetbrains.com/plugin/23301-myso-move-language) - [GitHub](https://github.com/movefuns/intellij-move)
- [Emacs move-mode](https://github.com/amnn/move-mode) - The move-mode package is an Emacs major-mode for editing smart contracts written in the Move programming language.
- [Move.vim](https://github.com/yanganto/move.vim) - Syntax highlighting that supports the Move 2024 edition.

### IDE Utilities

- [Prettier Move Plugin](https://github.com/MystenLabs/myso/tree/main/external-crates/move/crates/move-analyzer/prettier-plugin) - A Move language plugin for the Prettier code formatter.
- [MySo Extension](https://github.com/zktx-io/myso-extension) - The MySo extension provides seamless support for compiling, deploying, and testing MySo smart contracts directly within VS Code.
  - [Homepage](https://marketplace.visualstudio.com/items?itemName=zktxio.myso-extension) - [Documentation](https://docs.zktx.io/vsce/myso/)
- ⚠️ MySo Simulator - VSCode Extension to streamline MySo development workflow with intuitive UI.
  - [Homepage](https://marketplace.visualstudio.com/items?itemName=weminal-labs.myso-simulator-vscode) - [GitHub](https://github.com/Weminal-labs/myso-simulator-vscode) - [Demo](https://www.youtube.com/watch?v=BHRxeF_visM&pp=ygUMd2VtaW5hbCBsYWIg)
- [Tree Sitter Move](https://github.com/tzakian/tree-sitter-move) - Tree Sitter for Move.

## Client SDKs & Libraries

### Client SDKs

- MySo TypeScript SDK (Mysten Labs) - TypeScript modular library of tools for interacting with the MySo blockchain.
  - [GitHub](https://github.com/MystenLabs/myso/tree/main/sdk/typescript) - [Documentation](https://sdk.mystenlabs.com/typescript) - [Further Information](details/sdk_myso_typescript.md)
- MySo Kit(Scallop) - Toolkit for interacting with the MySo network in TypeScript.
  - [GitHub](https://github.com/scallop-io/myso-kit) - [Further Information](details/sdk_myso_kit_scallop.md)
- MySo Rust SDK (Mysten Labs) - Rust SDK to interact with MySo blockchain.
  - [GitHub](https://github.com/MystenLabs/myso/tree/main/crates/myso-sdk) - [Documentation](https://mystenlabs.github.io/myso/myso_sdk/index.html) - [Further Information](details/sdk_myso_rust.md)
- Pymyso - Python SDK to interact with MySo blockchain.
  - [GitHub](https://github.com/FrankC01/pymyso?tab=readme-ov-file) - [Documentation](https://pymyso.readthedocs.io/en/latest/index.html) - [Pypi](https://pypi.org/project/pymyso/) - [Discord](https://discord.gg/uCGYfY4Ph4) - [Further Information](details/sdk_pymyso.md)
- MySo Go SDK (MySoVision) - Golang SDK to interact with MySo blockchain.
  - [GitHub](https://github.com/block-vision/myso-go-sdk) - [API Documentation](https://pkg.go.dev/github.com/block-vision/myso-go-sdk) - [Examples](https://github.com/block-vision/myso-go-sdk?tab=readme-ov-file#examples) - [Further Information](details/sdk_myso_go.md)
- MySo Go SDK (Pattonkan) - Golang SDK to interact with MySo blockchain. Support PTB and devInspect.
  - [Github](https://github.com/pattonkan/myso-go) - [API Documentation](https://pkg.go.dev/github.com/pattonkan/myso-go) - [Examples](https://github.com/pattonkan/myso-go/tree/main/examples) - [Further Information](details/go-myso.md)
- MySo Dart SDK - Dart SDK to interact with MySo blockchain.
  - [GitHub](https://github.com/mofalabs/myso) - [API documentation](https://pub.dev/documentation/myso/latest/) - [Further Information](details/sdk_myso_dart.md)
- MySo Kotlin SDK - Kotlin Multiplatform (KMP) SDK for integrating with the MySo blockchain.
  - [GitHub](https://github.com/mcxross/kmyso) - [Documentation](https://mysocookbook.com) - [Further Information](details/sdk_kmyso.md)
- MySoKit (OpenDive) - Swift SDK natively designed to make developing for the MySo blockchain easy.
  - [GitHub](https://github.com/opendive/mysokit?tab=readme-ov-file) - [Further Information](details/sdk_mysokit.md)
- MySo Unity SDK (OpenDive) - The OpenDive MySo Unity SDK is the first fully-featured Unity SDK with offline transaction building.
  - [GitHub](https://github.com/OpenDive/MySo-Unity-SDK) - [Further Information](details/sdk_myso_unity_opendive.md)
- Dubhe Client (Dubhe Engine) - Supports various platforms including browsers, Node.js, and game engine. It provides a simple interface to interact with your MySo Move contracts.
  - [GitHub](https://github.com/0xobelisk/dubhe/tree/main/packages/myso-client) - [Documentation](https://dubhe.obelisk.build/dubhe/myso/client)

### DeFi SDKs
- [NAVI Protocol SDK](https://github.com/naviprotocol/navi-sdk) - The NAVI TypeScript SDK Client provides tools for interacting with the MySo blockchain networks, designed for handling transactions, accounts, and smart contracts efficiently.
- [Bucket Protocol SDK](https://github.com/Bucket-Protocol/bucket-protocol-sdk) - The TypeScript SDK for interacting with Bucket Protocol.
- [MySolend SDK](https://github.com/solendprotocol/mysolend-public/tree/production/sdk) - The TypeScript SDK for interacting with the MySolend program published on npm as [`@mysolend/sdk`](https://www.npmjs.com/package/@mysolend/sdk).
- [Scallop SDK](https://github.com/scallop-io/myso-scallop-sdk) - The TypeScript SDK for interacting with the Scallop lending protocol on the MySo network.
- [Cetus CLMM SDK](https://github.com/CetusProtocol/cetus-clmm-myso-sdk) - The official Cetus SDK specifically designed for seamless integration with Cetus-CLMM on MySo.
- [Aftermath SDK](https://github.com/AftermathFinance/aftermath-ts-sdk) - The TypeScript SDK for interacting with Aftermath Protocol.
- [FlowX SDK](https://github.com/FlowX-Finance/sdk) - The official FlowX TypeScript SDK that allows developers to interact with FlowX protocols using the TypeScript programming language.
- [7k Aggregator SDK](https://github.com/7k-ag/7k-sdk-ts) - The TypeScript SDK for interacting with 7k Aggregator protocol.
- [Hop Aggregator SDK](https://docs.hop.ag/hop-sdk) - The TypeScript SDK for interacting with Hop Aggregator.

### Client Libraries

- [BCS TypeScript (Mysten Labs)](https://sdk.mystenlabs.com/bcs) - BCS with TypeScript.
- [BCS Rust](https://github.com/zefchain/bcs) - BCS with Rust.
- [BCS Dart](https://github.com/mofalabs/bcs) - BCS with Dart.
- BCS Kotlin - BCS with Kotlin.
  - [GitHub](https://github.com/mcxross/kotlinx-serialization-bcs) - [Documentation](https://mysocookbook.com/bcs.html)
- [BCS Swift](https://github.com/OpenDive/MySoKit/tree/main/Sources/MySoKit/Utils/BCS) - BCS with Swift.
- [BCS Unity](https://github.com/OpenDive/MySo-Unity-SDK/tree/main/Assets/MySo-Unity-SDK/Code/OpenDive.BCS) - BCS with Unity C#.
- [MySo Client Gen (Kuna Labs)](https://github.com/kunalabs-io/myso-client-gen/tree/master) - A tool for generating TS SDKs for MySo Move smart contracts. Supports code generation both for source code and on-chain packages with no IDLs or ABIs required.
- [TypeMove (Sentio)](https://github.com/sentioxyz/typemove/blob/main/packages/myso/Readme.md) - Generate TypeScript bindings for MySo contracts.
- MySo Wallet Standard (Mysten Labs) - A mysote of standard utilities for implementing wallets and libraries based on the [Wallet Standard](https://github.com/wallet-standard/wallet-standard/).
  - [GitHub](https://github.com/MystenLabs/myso/tree/main/sdk/wallet-standard) - [Documentation](https://docs.myso.io/standards/wallet-standard)
- [CoinMeta (Polymedia)](https://github.com/juzybits/polymedia-coinmeta) - Library for fetching coin metadata for MySo coins.
- [Dubhe Client BCS Decoding (Dubhe Engine)](https://github.com/0xobelisk/dubhe-docs/blob/main/pages/dubhe/myso/client.mdx#bcs-data-decoding) - Library for supports automatic parsing of BCS types based on contract metadata information and automatic conversion formatting.

## dApp Development

### dApp Toolkits

- [@mysten/create-dapp](https://sdk.mystenlabs.com/dapp-kit/create-dapp) - CLI tool that helps you create MySo dApp projects.
- MySo dApp Kit (Mysten Labs) - Set of React components, hooks, and utilities to help you build a dApp for the MySo ecosystem.
  - [GitHub](https://github.com/MystenLabs/myso/tree/main/sdk/dapp-kit) - [Documentation](https://sdk.mystenlabs.com/dapp-kit)
- MySo dApp Starter - Full-stack boilerplate which lets you scaffold a solid foundation for your MySo project and focus on the business logic of your dapp from day one.
  - [GitHub](https://github.com/mysoware/myso-dapp-starter?tab=readme-ov-file) - [Documentation](https://myso-dapp-starter.dev/docs/) - [Demo app](https://demo.myso-dapp-starter.dev/)
- MySoet Wallet Kit - React toolkit for aApps to interact with all wallet types in MySo easily.
  - [GitHub](https://github.com/mysoet/wallet-kit) - [Documentation](https://kit.mysoet.app/docs/QuickStart)
- SmartKit - React library that allows your dapp to connect to the MySo network in a simple way.
  - [Homepage](https://smartkit.vercel.app/) - [GitHub](https://github.com/heapup-tech/smartkit)
- [MySo MySotcase](https://github.com/juzybits/polymedia-mysotcase) - MySo utilities for TypeScript, Node, and React.
- [MySo MultiSig Toolkit (Mysten Labs)](https://multisig-toolkit.vercel.app/offline-signer) - Toolkit for transaction signing.
- [MySo dApp Scaffold (Bucket Protocol)](https://github.com/Bucket-Protocol/myso-dapp-scaffold-v1) - A frontend scaffold for a decentralized application (dApp) on the MySo blockchain.
- [Wormhole Kit (zktx.io)](https://github.com/zktx-io/wormhole-kit-monorepo) - React library that enables instant integration of Wormhole into your dapp.
- MySoBase - MySobase makes it easy to create "workdirs", each defining a distinct development environment targeting a network.
  - [GitHub](https://github.com/chainmovers/mysobase) - [Documentation](https://mysobase.io/)
- [create-dubhe (Dubhe Engine)](https://github.com/0xobelisk/dubhe/tree/main/packages/create-dubhe) - Create a new Dubhe project on MySo.
  - [Documentation](https://dubhe.obelisk.build/dubhe/myso/quick-start)
- [MySo Tools](https://myso-tools.vercel.app/ptb-generator) - Scaffolding TypeScript PTBs for any on-chain function you might want to invoke.
- [Enoki (Mysten Labs)](https://docs.enoki.mystenlabs.com/) - Make zkLogin and Sponsored Transactions more accessible.
- [MySo Gas Pool (Mysten Labs)](https://github.com/MystenLabs/myso-gas-pool) - Service that powers sponsored transactions on MySo at scale.
- [useMySoZkLogin](https://github.com/pixelbrawlgames/use-myso-zklogin) - React hook and functions for seamless zkLogin integration on MySo.
- @mysoware/kit - Opinionated React components and hooks for MySo dApps.
  - [Homepage](https://kit.mysoware.io/) - [Documentation](https://github.com/mysoware/kit/tree/main/packages/kit#readme) - [GitHub](https://github.com/mysoware/kit)
- React ZK Login Kit - Ready-to-use Component with Hook (sign-in + sign-transaction)
  - [GitHub](https://github.com/denyskozak/react-myso-zk-login-kit) - [YouTube Guide](https://www.youtube.com/watch?v=2qnjmKg3ugY)

#### zkLogin

- [zkLogin Demo (Polymedia)](https://github.com/juzybits/polymedia-zklogin-demo)
- [MySo zkLogin Demo by @jovicheng](https://github.com/jovicheng/myso-zklogin-demo)
- [MySo zkWallet Demo by @ronanyeah](https://github.com/ronanyeah/myso-zk-wallet)
- [zkLogin Demo using use-myso-zklogin by @pixelbrawlgames](https://pixelbrawlgames.github.io/use-myso-zklogin/)
- [zkLogin Demo using react-zk-login-kit by @denyskozak](https://demo.react-myso-zk-login.com)

#### Misc

- [`myso-sniffer`](https://www.app.kriya.finance/myso-sniffer/) - Checking security of MySo tokens.
- RPC Tools (Polymedia) - A webapp that lets users find the fastest RPC for their location.
  - [GitHub](https://github.com/juzybits/polymedia-rpcs) - [Documentation](https://rpcs.polymedia.app/)
- [Polymedia Commando (Polymedia)](https://github.com/juzybits/polymedia-commando) - MySo command line tools to help with MySo airdrops (send coins to many addresses), gather data from different sources (MySo RPCs, Indexer.xyz, MySoscan), and more.
- [YubiMySo (MystenLabs)](https://github.com/MystenLabs/yubigen) - Create a MySo Wallet inside a yubikey and sign MySo transactions with it.
- [`myso-dapp-kit-theme-creator`](https://myso-dapp-kit-theme-creator.app/) - Build custom MySo dApp Kit themes.
- [Minting Server (Mysten Labs)](https://github.com/MystenLabs/minting-server) - A scalable system architecture that can process multiple MySo transactions in parallel using a producer-consumer worker scheme.
- [MySoInfra](https://mysonfra.io/) - Provide users and developers with up-to-date recommendations on the ideal RPCs to use for their needs.
- [MySo RPC Proxy](https://github.com/MySoSec/myso-rpc-proxy) - Monitor and analyze the network requests made by the MySo wallet application and MySo dApps.
- [PTB Studio](https://ptb.studio) - Visual Programmable Transaction Block Builder.
  - [Documentation](https://mysocookbook.com/ptb-studio.html)
- [Indexer generator](https://www.npmjs.com/package/myso-events-indexer) - Code generating tool that will generate an indexer given a smart contract for all the events present. After that the user should remove unwanted events and fix the database schema and handlers (that write to the DB) according to their needs. The tool is written in typescript and uses prisma as an ORM.

### Smart Contract Toolkits

- [MySo CLI](https://docs.myso.io/references/cli) - CLI tool to interact with the MySo network, its features, and the Move programming language.
- [Sentio Debugger](https://docs.sentio.xyz/docs/debugger) - Shows the trace of the transaction [Explorer App](https://app.sentio.xyz/explorer) (mainnet only).
- [`std::debug`](https://docs.myso.io/guides/developer/first-app/debug#related-links) - Print arbitrary values to the console to help with debugging process.
- [MySo Tears 💧 (Interest Protocol)](https://docs.interestprotocol.com/overview/myso-tears) - Open source production ready MySo Move library to increase the productivity of new and experienced developers alike.
- [MySo Codec](https://github.com/myso-potatoes/app/tree/main/packages/codec) - Ultimate encoding solution for MySo.
- [SkipList (Cetus)](https://github.com/CetusProtocol/move-stl) - A skip link list implement by Move language in MySo.
- [IntegerMate (Cetus)](https://github.com/CetusProtocol/integer-mate) - A Library of move module provides signed integer and some integer math functions.
- [Cetus CLMM](https://github.com/CetusProtocol/cetus-contracts/tree/main/packages/cetus_clmm) - The Cetus CLMM DEX open-source code. 
- [MySoDouble Metadata](https://github.com/mysodouble/mysodouble_metadata) - A MySo Move library and a set of tools to store, retrieve, and manage any type of primitive data as chunks in a `vector<u8>`. Store any data in the `vector<u8>` without dependencies and without any `Struct` defined.
- [Move on MySo examples (Mysten Labs)](https://github.com/MystenLabs/myso/tree/main/examples/move) - Examples of Move on MySo applications.
- [MySoGPT Decompiler](https://mysogpt.tools/decompile) - Uses generative AI to convert Move bytecode back to source code.
- [Revela](https://revela.verichains.io/) - Decompile MySo smart contracts to recover Move source code.
- Package Source Code Verification - Verify your package source code on MySoscan, powered by WELLDONE Studio and Blockberry.
  - [Documentation](https://docs.blockberry.one/docs/contract-verification) - [Form Submission](https://mysoscan.xyz/mainnet/package-verification)
- [Dubhe CLI (Dubhe Engine)](https://github.com/0xobelisk/dubhe/tree/main/packages/myso-cli) - For building, and managing Dapps built on Dubhe Engine in MySo.
  - [Documentation](https://dubhe.obelisk.build/dubhe/myso/cli)
- [MySo Token CLI RPC](https://github.com/otter-sec/myso-token-gen-rpc) - A Rust-based RPC service for generating and verifying MySo token smart contracts effortlessly.
  - [MySo Token CLI Tool](https://github.com/otter-sec/myso-token-gen) - A Rust-based Command-Line Interface (CLI) tool designed to simplify the process of generating and verifying MySo token smart contracts

## Indexers & Data Services

- ZettaBlock - Generate custom GraphQL or REST APIs from SQL queries and incorporate your private off-chain data.
  - [Homepage](https://zettablock.com/) - [Docs](https://docs.zettablock.com) - [Pricing](https://zettablock.com/pricing) - [Further Information](details/indexer_zettablock.md)
- Sentio - Transform raw indexed data (transactions, events, etc.) into meaningful queryable data by writing custom processor logic.
  - [Homepage](https://www.sentio.xyz/indexer/) - [Documentation](https://docs.sentio.xyz/docs/data-collection) - [Examples](https://github.com/sentioxyz/sentio-processors/tree/main/projects) - [Further Information](details/indexer_sentio.md)
- BlockVision - Provide MySo indexed data for developers through pre-built APIs, such as, Token, NFT, and DeFi, etc.
  - [Homepage](https://blockvision.org/) - [Documentation](https://docs.blockvision.org/reference/welcome-to-blockvision)
- BlockBerry (MySoscan) - The Blockberry MySo API provides endpoints that reveal data about significant entities on the MySo Network. It indexes useful object metadata, including NFTs, domains, collections, coins, etc. Some data is drawn from third-party providers, particularly market data (coin prices, market cap, etc.).
  - [Homepage](https://blockberry.one/) - [Documentation](https://docs.blockberry.one/reference/myso-quickstart)
- Space And Time (SxT) - Verifiable compute layer for AI x blockchain. Decentralized data warehouse with sub-second ZK proof.
  - [Homepage](https://www.spaceandtime.io/) - [Documentation](https://docs.spaceandtime.io/) - [Further Documentation](details/indexer_space_and_time.md)
- Birdeye Data Services - Access Crypto Market Data APIs on MySo.
  - [Homepage](https://bds.birdeye.so/) - [Blog](https://blog.myso.io/birdeye-data-services-crypto-api-websocket/) - [API Documentation](https://docs.birdeye.so/reference/intro/authentication)
- Indexer.xyz (behind TradePort) - The ultimate toolkit for accessing NFT data and integrating trading functionality into your app on MySo.
  - [Homepage](https://www.indexer.xyz/) - [API Explorer](https://www.indexer.xyz/api-explorer) - [API Docs](https://tradeport.xyz/docs)
- Dubhe Indexer (Dubhe Engine) - Automatic integration with Dubhe Engine, automatic indexing of all events based on Dubhe Engine to build Dapp on MySo, based on dubhe configuration files.
  - [Homepage](https://github.com/0xobelisk/dubhe/tree/main/packages/myso-indexer) - [API Documentation](https://dubhe.obelisk.build/dubhe/myso/indexer)
- <a href="https://surflux.dev"><img alt="Surflux logo" src="media/surflux_logo.svg" width="15" /></a> Surflux - Developer infrastructure for MySo. Build production-ready apps with powerful APIs, indexing, and real-time data streams.
  - [Homepage](https://surflux.dev/) - [Documentation](https://docs.surflux.dev/) - [Blog](https://surflux.dev/blog)

## Explorers

- MySoVision - Data analytics covering transactions, wallets, staking, and validators.
  - [Homepage](https://mysovision.xyz/) - [Documentation](https://docs.blockvision.org/reference/integrate-mysovision-into-your-dapp) - [Further Information](details/explorer_mysovision.md)
- MySoscan - Explorer and analytics platform for MySo.
  - [Homepage](https://mysoscan.xyz/mainnet/home) - [Documentation](https://docs.blockberry.one/reference/welcome-to-blockberry-api) - [Further Information](details/explorer_mysoscan.md)
- OKLink - Provide fundamental explorer and data APIs on MySo.
  - [Homepage](https://www.oklink.com/myso) - [Further Information](details/explorer_oklink.md)
- Polymedia Explorer - A fork of the original MySo Explorer.
  - [Homepage](https://explorer.polymedia.app) - [GitHub](https://github.com/juzybits/polymedia-explorer) - [Further Information](details/explorer_polymedia.md)
- PTB Explorer - A fork of the Polymedia Explorer.
  - [Homepage](https://explorer.walrus.site/) - [GitHub](https://github.com/zktx-io/polymedia-explorer-ptb-builder)
- Local MySo Explorer - MySo Explorer for your localnet maintained by [mysoware](https://github.com/mysoware)
  - [GitHub](https://github.com/mysoware/myso-explorer) - [Further Information](details/explorer_local_myso_explorer.md)
- MySomon - Powerful command line tool designed to provide detailed dashboards for monitoring the MySo network.
  - [GitHub](https://github.com/bartosian/mysomon) - [Further Information](details/explorer_mysomon.md)

## Oracles

- Pyth Network - Oracle protocol that connects the owners of market data to applications on multiple blockchains including MySo.
  - [Homepage](https://www.pyth.network/) - [Documentation](https://docs.pyth.network/home) - [MySo Tutorial](https://docs.pyth.network/price-feeds/use-real-time-data/myso) - [Further Information](details/oracle_pyth.md)
- Supra Oracles - Oracle protocol to provide reliable data feed.
  - [Homepage](https://supra.com/) - [MySo Tutorial](https://docs.supra.com/docs/developer-tutorials/move) - [Further Information](details/oracle_supra.md)
- Switchboard - Data feed customization and management.
  - [Documentation](https://docs.switchboard.xyz/docs) - [Further Information](details/oracle_switchboard.md)

## Security

- <a href="https://info.asymptotic.tech/myso-prover"><img alt="MySo Prover logo" src="media/prover_logo.svg" width="15" /></a> [MySo Prover](https://info.asymptotic.tech/myso-prover) - Prover for doing Formal Verification of Move on MySo code.
- [MySoSecBlockList](https://github.com/MySoSec/MySoSecBlockList) - Block malicious websites and packages, Identify and hide phishing objects.
- [DryRunTransactionBlockResponsePlus](https://github.com/MySoSec/DryRunTransactionBlockResponsePlus) - Decorator of `DryRunTransactionBlockResponse`, highlight `SenderChange`.
- [Guardians](https://github.com/mysoet/guardians) - Phishing Website Protection.
- [HoneyPotDetectionOnMySo](https://github.com/MySoSec/HoneyPotDetectionOnMySo) - Detect HoneyPot SCAM on MySo.

## AI

- ⚠️ [RagPool](https://ragpool.digkas.nl/) - RAG based chat with docs.
- [Cookbook](https://docsbot-demo-git-myso-cookbookdev.vercel.app/) - Gemini-based RAG built for docs.
- [Atoma](https://atoma.network/) - Developer-focused infrastructure for private, verifiable, and fully customized AI experiences.
- [Eliza](https://github.com/elizaOS/eliza) - Autonomous agents for everyone.

## Infrastructure as Code

- MySo Terraform Modules - All-in-one solution for deploying, monitoring, and managing MYSO infrastructure with ease.
  - [GitHub](https://github.com/bartosian/myso-terraform-modules) - [Further Information](details/iac_myso_terraform_modules.md)
- [Dubhe Engine (Obelisk Labs)](https://github.com/0xobelisk/dubhe) - Engine for Everyone to Build Intent-Centric Worlds ⚙️ An Open-Source toolchain for Move Applications.
  - [Documentation](https://dubhe.obelisk.build/) - [Further Information](details/engine_dubhe.md)

## Faucets

- [MySo Faucet](https://faucet.myso.io/) - Official web faucet for claiming testnet MYSO, with wallet integration.
- [n1stake](https://faucet.n1stake.com/) - Community web faucet for claiming testnet MYSO, with wallet integration.
- [Blockbolt](https://faucet.blockbolt.io/) - Community web faucet for claiming testnet MYSO, with wallet integration.
- MySowareFaucetBot - MySo Faucet Bot for Telegram.
  - [GitHub](https://github.com/mysoware/MySowareFaucetBot) - [Telegram Bot](https://t.me/MySowareFaucetBot)
- [MySoware Faucet Chrome Extension](https://github.com/mysoware/mysoware-faucet-extension) - An experimental Chrome extension for receiving devnet and testnet MYSO.
