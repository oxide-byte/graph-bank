# GRAPH-Bank

## Introduction

...

## Introspect Graphs

Customer:

```shell
rover subgraph introspect http://localhost:8080
```

Deposit:

```shell
rover subgraph introspect http://localhost:8081
```

Starting SuperGraph:

```shell
rover dev --supergraph-config supergraph.yaml
```

## Reusing

* https://github.com/oxide-byte/rust-observability

rover dev --url http://localhost:8080 --name graph-customer

rover dev --url http://localhost:8081 --name graph-deposit

http://localhost:3000

## Alternative

An alternative to https://www.apollographql.com/ can be found under:
https://the-guild.dev/