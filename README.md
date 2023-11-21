<img src="https://github.com/jonas089/raptor-abi/blob/master/resources/logo.webp" alt="Alt text" title="Logo" width="500" height="500"/>

## Supported casper_types
- Bool
- I32
- I64
- U8
- U32
- U64
- U128
- U256
- U512
- Unit
- String
- Key
- URef
- PublicKey

- Any

```

██╗███╗░░██╗██╗░░██╗██╗
██║████╗░██║██║░██╔╝██║
██║██╔██╗██║█████═╝░██║
██║██║╚████║██╔═██╗░╚═╝
██║██║░╚███║██║░╚██╗██╗
╚═╝╚═╝░░╚══╝╚═╝░░╚═╝╚═╝
```
a smart contract ABI generator for Polkadot

Claptrap is essentially a compact implementation of Ink for Casper smart contracts.

As of today, Claptrap supports metadata generation at build-time for **Entry Point** definitions.

## src/ink.rs
Artificial example contract that demonstrates the implementation of **Entry Points** using Claptrap macros.

## code_generator
Procedural macro definitions for contract ABI generation inspired by Ink!.

## helpers
Deadcode Type definitions and filehandlers.

## test.sh
Compile ink.rs => output.json

test.sh deletes the target folder (if present) because otherwise the ink contract is not built from source => no or empty output.json.

## Direct comparison
Native Casper:
```rust
EntryPoint::new("test", vec![...], CLType::Unit, EntryPointAccess::Public, EntryPointType::Contract);
```
Claptrap:
```rust
#[derive(Default)]
#[derive(InkCasperMacro)]
struct NewEntryPointArgs1{
    sender: Key,
    recipient: Key,
    amount: U64,
    id_first:U256,
    text: String
}
let ep = NewEntryPointArgs1::default();
let ep_parameters = ep.get_params();
let entry_point = EntryPoint::new("test", ep_parameters.clone(), CLType::Unit, EntryPointAccess::Public, EntryPointType::Contract);
```
The additional syntax is required as Claptrap parses the struct at compile time using a proc macro to generate valuable metadata for the contract.

