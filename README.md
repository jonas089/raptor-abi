# Claptrap
a casper smart contract metadata extractor.

## Developer notes
Metadata (output.json) is an artifact that's generated at build-time.

Target folder has to be removed before re-generating metadata for the same contract.

If this is not done, Rust will not recompile the contract from source and therefore not generate Metadata.
