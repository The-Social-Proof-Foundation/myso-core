# Phase 1 Enterprise Workforce — Scratchpad

## Background and Motivation
Complete Workstream F (GraphQL/REST org dashboards) and begin Workstream G (build/test).

## Project Status Board
- [x] graphql-rest: enterprise reader SQL + GraphQL types + REST mirrors + snapshots
- [x] build-and-test (partial): myso-core reader/graphql/social-server nextest, memory relayer tests, messaging relayer tests

## Executor's Feedback
GraphQL/REST workstream complete. Snapshots regenerated (`schema.graphql`, pipeline registry). Tests green on touched crates.

## Lessons
- GraphQL `MySoAddress::from_str` required explicit type vs `.parse()` on String
- Social GraphQL fields use empty pipeline sets (auto-registered via schema introspection test)
