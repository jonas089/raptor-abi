# Claptrap
This project took inspiration from
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
