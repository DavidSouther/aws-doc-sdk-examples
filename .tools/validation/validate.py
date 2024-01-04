# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

import argparse
import yaml
from dataclasses import dataclass
from pathlib import Path
from sys import exit
from typing import Optional

from sdk_examples.doc_gen import DocGen
from sdk_examples.metadata import parse as parse_metadata, Example
from sdk_examples.metadata_errors import MetadataErrors, MetadataError
from sdk_examples.snippets import collect_snippets, Snippet

from metadata_validator import validate_metadata
from project_validator import check_files, verify_sample_files


def validate_zexii(metadata_path: Path, errors: MetadataErrors) -> list[Example]:
    doc_gen, doc_gen_errors = DocGen.from_root(metadata_path)
    errors.extend(doc_gen_errors)

    if len(doc_gen_errors) > 0:
        return

    metadata = []

    for path in metadata_path.glob("*_metadata.yaml"):
        if path.name == "cross_metadata.yaml":
            continue
        with open(path, encoding="utf-8") as file:
            meta = yaml.safe_load(file)
        meta_list, metadata_errors = parse_metadata(path.name, meta, doc_gen)
        metadata.extend(meta_list)
        errors.maybe_extend(metadata_errors)

    return metadata


@dataclass
class MissingSnippet(MetadataError):
    tag: Optional[str] = None

    def prefix(self):
        return f"for {self.tag}"

    def message():
        return "missing snippet"


@dataclass
class DuplicateSnippetFile(MetadataError):
    snippet_file: Optional[str] = None

    def prefix(self):
        return f"for {self.snippet_file}"

    def message():
        return "duplicate snippet_file"


@dataclass
class MissingSnippetFile(MetadataError):
    snippet_file: Optional[str] = None

    def prefix(self):
        return f"for {self.snippet_file}"

    def message():
        return "missing snippet_file"


def validate_snippets(
    root: Path,
    metadata: list[Example],
    snippets: dict[str, Snippet],
    errors: MetadataErrors,
):
    snippet_files = set()
    for example in metadata:
        for lang in example.languages:
            language = example.languages[lang]
            for version in language.versions:
                for excerpt in version.excerpts:
                    for snippet_tag in excerpt.snippet_tags:
                        if snippet_tag not in snippets:
                            # Ensure all metadata snippets are found
                            errors.append(
                                MissingSnippet(file=example.file, tag=snippet_tag)
                            )
                    for snippet_file in excerpt.snippet_files:
                        if snippet_file in snippet_files:
                            # Ensure no snippet_files duplicates
                            errors.append(
                                DuplicateSnippetFile(
                                    file=example.file, snippet_file=snippet_file
                                )
                            )
                        if not (root / snippet_file).exists():
                            # Ensure all snippet_files exist
                            errors.append(
                                MissingSnippetFile(
                                    file=example.file, snippet_file=snippet_file
                                )
                            )
                        snippet_files.add(snippet_file)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--root",
        default=f"{Path(__file__).parent.parent.parent}",
        help="The root path from which to search for files to check. The default is the root of the git repo (two up from this file).",
    )
    parser.add_argument(
        "--doc-gen",
        default=f"{Path(__file__).parent.parent.parent / '.doc_gen'}",
        help="The folder that contains schema and metadata files. The default is .doc_gen in the root of this repo.",
        required=False,
    )
    args = parser.parse_args()
    root_path = Path(args.root).resolve()
    doc_gen = Path(args.doc_gen).resolve()

    errors = MetadataErrors()

    check_files(root_path, errors)
    verify_sample_files(root_path, errors)
    validate_metadata(doc_gen, errors)
    metadata = validate_zexii(doc_gen / "metadata", errors)
    snippets, snippet_errors = collect_snippets(root_path)
    errors.extend(snippet_errors)
    validate_snippets(root_path, metadata, snippets, errors)

    error_count = len(errors)
    if error_count > 0:
        print(errors)
        print(f"{error_count} errors found, please fix them.")
    else:
        print("All checks passed, you are cleared to check in.")

    return error_count


if __name__ == "__main__":
    exit(main())
