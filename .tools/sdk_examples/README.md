# AWS SDK Code Examples Extractor

This library allows AWS SDK Code Examples engineers programmatic access to the range of metadata in SoS.

## Motivation:

1. SoS Metadata
1. Snippet Extraction
1. WRITEME
1. Ailly
1. Zexii
1. Directories / Developer Center

all have some notion of "do something on S service in K language/SDK"

`sdk_examples` provides a convenient entry point, `load`, which will review a folder structure for `.doc_gen` SoS metadata and embedded SOS snippet sections.

This can then be used further for validation, reviewing snippets, etc.
