#!/usr/bin/env python3

# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

"""
Validator for mapping and example metadata used to generate code example documentation.
This validator uses Yamale (https://github.com/23andMe/Yamale) to compare a schema
against YAML files stored in the metadata folder and runs as a GitHub action.
"""

import argparse
import os
import yaml
import yamale
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable, Optional
from yamale import YamaleError
from yamale.validators import DefaultValidators, Validator, String

from sdk_examples.doc_gen import DocGen
from sdk_examples.metadata_errors import MetadataErrors, MetadataParseError
from sdk_examples.metadata_validator import (
    BlockContent,
    ExampleId,
    SdkVersion,
    ServiceName,
    ServiceVersion,
    SourceKey,
    StringExtension,
)


@dataclass
class ValidateYamaleError(MetadataParseError):
    yamale_error: Optional[YamaleError] = field(default=None)

    def message(self):
        return f"Yamale Error: {self.yamale_error.message}"


def validate_files(
    schema_name: Path,
    meta_names: Iterable[Path],
    validators: dict[str, Validator],
    errors: MetadataErrors,
):
    """Iterate a list of files and validate each one against a schema."""

    schema = yamale.make_schema(schema_name, validators=validators)
    for meta_name in meta_names:
        try:
            data = yamale.make_data(meta_name)
            yamale.validate(schema, data)
            print(f"{meta_name.resolve()} validation success! 👍")
        except YamaleError as e:
            errors.append(ValidateYamaleError(file=meta_name, yamale_error=e))
    return errors


def validate_metadata(root: Path, errors: MetadataErrors):
    doc_gen = DocGen(root)
    with open(doc_gen / "metadata" / "sdks.yaml") as sdks_file:
        sdks_yaml: dict[str, any] = yaml.safe_load(sdks_file)

    with open(doc_gen / "metadata" / "services.yaml") as services_file:
        services_yaml = yaml.safe_load(services_file)

    with open(
        doc_gen / "metadata" / "curated" / "sources.yaml"
    ) as curated_sources_file:
        curated_sources_yaml = yaml.safe_load(curated_sources_file)

    BlockContent.block_names = os.listdir(root / "cross-content")
    ExampleId.services = services_yaml
    SdkVersion.sdks = sdks_yaml
    ServiceName.services = services_yaml
    SourceKey.curated_sources = curated_sources_yaml

    validators = DefaultValidators.copy()
    validators[ServiceName.tag] = ServiceName
    validators[ServiceVersion.tag] = ServiceVersion
    validators[SourceKey.tag] = SourceKey
    validators[ExampleId.tag] = ExampleId
    validators[BlockContent.tag] = BlockContent
    validators[String.tag] = StringExtension

    schema_root = Path(__file__).parent / "schema"

    to_validate = [
        # (schema, metadata_glob)
        ("sdks_schema.yaml", "sdks.yaml"),
        ("services_schema.yaml", "services.yaml"),
        # TODO: Switch between strict schema for aws-doc-sdk-examples and loose schema for tributaries
        ("example_strict_schema.yaml", "*_metadata.yaml"),
        ("curated_sources_schema.yaml", "curated/sources.yaml"),
        ("curated_example_schema.yaml", "curated/*_metadata.yaml"),
    ]
    for schema, metadata in to_validate:
        validate_files(
            schema_root / schema,
            (doc_gen / "metadata").glob(metadata),
            validators,
            errors,
        )

    return errors


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--doc-gen",
        default=f"{Path(__file__).parent / '..' / '..' / '.doc_gen'}",
        help="The folder that contains schema and metadata files.",
        required=False,
    )
    args = parser.parse_args()

    errors = validate_metadata(Path(args.doc_gen))

    if errors == 0:
        print("Validation succeeded! 👍👍👍")
    else:
        print("\n********************************************")
        print("* Validation failed, please check the log! *")
        print("********************************************")
        exit(1)


if __name__ == "__main__":
    main()
