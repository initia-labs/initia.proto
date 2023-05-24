# initia.proto

## Maintenance

This section is for maintainers of this repo, not users.

### Prerequisites

* dotnet
* grpc_csharp_plugin

### Getting started

```sh
make init
```

### Rebuilding types

```sh
make proto-gen
```

### Building package

```sh
make build
```

### Publishing package (including rebuilding types and building package)
```sh
NUGET_API_KEY=... make all
```