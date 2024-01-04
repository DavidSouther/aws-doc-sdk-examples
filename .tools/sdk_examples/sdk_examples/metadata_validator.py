# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

"""
Validator for mapping and example metadata used to generate code example documentation.
This validator uses Yamale (https://github.com/23andMe/Yamale) to compare a schema
against YAML files stored in the metadata folder and runs as a GitHub action.
"""

import datetime
import re
from yamale.validators import Validator, String


class SdkVersion(Validator):
    """Validate that sdk version appears in sdks.yaml."""

    tag = "sdk_version"
    sdks = {}

    def get_name(self):
        return "sdk version found in sdks.yaml"

    def _is_valid(self, value):
        return value in self.sdks


class ServiceName(Validator):
    """Validate that service names appear in services.yaml."""

    tag = "service_name"
    services = {}

    def get_name(self):
        return "service name found in services.yaml"

    def _is_valid(self, value):
        return value in self.services


class ServiceVersion(Validator):
    tag = "service_version"

    def get_name(self):
        return "valid service version"

    def _is_valid(self, value):
        try:
            hyphen_index = len(value)
            for _ in range(3):
                hyphen_index = value.rfind("-", 0, hyphen_index)
            time = datetime.datetime.strptime(value[hyphen_index + 1 :], "%Y-%m-%d")
            isdate = isinstance(time, datetime.date)
        except ValueError:
            isdate = False
        return isdate


class SourceKey(Validator):
    """Validate that curated source keys appear in curated/sources.yaml."""

    tag = "source_key"
    curated_sources: dict[str, any] = {}

    def get_name(self):
        return "source key found in curated/sources.yaml"

    def _is_valid(self, value):
        return value in self.curated_sources


class ExampleId(Validator):
    """
    Validate an example ID starts with a service ID and has underscore-separated
    operation and specializations (like sns_Subscribe_Email).
    """

    tag = "example_id"
    services: dict[str, any] = {}

    def get_name(self):
        return "valid example ID"

    def _is_valid(self, value):
        if not re.fullmatch("^[\\da-z-]+(_[\\da-zA-Z]+)+$", value):
            return False
        else:
            svc = value.split("_")[0]
            return svc == "cross" or svc in self.services


class BlockContent(Validator):
    """Validate that block content refers to an existing file."""

    tag = "block_content"
    block_names: list[str] = []

    def get_name(self):
        return "file found in the cross-content folder"

    def _is_valid(self, value):
        return value in self.block_names


class StringExtension(String):
    """Validate that strings don't contain non-entity AWS usage."""

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.check_aws = bool(kwargs.pop("check_aws", True))
        self.upper_start = bool(kwargs.pop("upper_start", False))
        self.lower_start = bool(kwargs.pop("lower_start", False))
        self.end_punc = bool(kwargs.pop("end_punc", False))
        self.no_end_punc = bool(kwargs.pop("no_end_punc", False))
        self.end_punc_or_colon = bool(kwargs.pop("end_punc_or_colon", False))
        self.end_punc_or_semicolon = bool(kwargs.pop("end_punc_or_semicolon", False))
        self.last_err = "valid string"

    def get_name(self):
        return self.last_err

    def _is_valid(self, value):
        if value == "":
            return True
        valid = True
        if self.check_aws:
            # All occurrences of AWS must be entities or within a word.
            valid = len(re.findall("(?<![&0-9a-zA-Z])AWS(?![;0-9a-zA-Z])", value)) == 0
            if not valid:
                self.last_err = 'valid string: it contains a non-entity usage of "AWS"'
        if valid and self.upper_start:
            valid = str.isupper(value[0])
            if not valid:
                self.last_err = "valid string: it must start with an uppercase letter"
        if valid and self.lower_start:
            valid = str.islower(value[0])
            if not valid:
                self.last_err = "valid string: it must start with a lowercase letter"
        if valid and self.end_punc:
            valid = value[-1] in "!.?"
            if not valid:
                self.last_err = "valid sentence or phrase: it must end with punctuation"
        if valid and self.no_end_punc:
            valid = value[-1] not in "!.?"
            if not valid:
                self.last_err = "valid string: it must not end with punctuation"
        if valid and self.end_punc_or_colon:
            valid = value[-1] in "!.?:"
            if not valid:
                self.last_err = (
                    "valid sentence or phrase: it must end with punctuation or a colon"
                )
        if valid and self.end_punc_or_semicolon:
            valid = value[-1] in "!.?;"
            if not valid:
                self.last_err = "valid sentence or phrase: it must end with punctuation or a semicolon"
        if valid:
            valid = super()._is_valid(value)
        return valid
