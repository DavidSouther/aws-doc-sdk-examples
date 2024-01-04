# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

from dataclasses import dataclass
from typing import Optional
from pathlib import Path
from shutil import copyfile, rmtree

from sdk_examples.file_utils import get_files
from sdk_examples.metadata_errors import MetadataErrors, MetadataError

SNIPPET_START = "snippet-start:["
SNIPPET_END = "snippet-end:["


@dataclass
class Snippet:
    id: str
    file: str
    line_start: int
    line_end: int
    code: str


@dataclass
class SnippetError(MetadataError):
    line: Optional[int] = None
    tag: Optional[str] = None

    def prefix(self):
        super().prefix() + f" at l{self.line} for {self.tag}: "


@dataclass
class DuplicateSnippetStartError(SnippetError):
    def message(self):
        return "duplicate snippet-start tag"


@dataclass
class DuplicateSnippetEndError(SnippetError):
    def message(self):
        return "duplicate snippet-end tag"


@dataclass
class MissingSnippetStartError(SnippetError):
    def message(self):
        return "snippet-end with no matching start"


@dataclass
class MissingSnippetEndError(SnippetError):
    def message(self):
        return "snippet-start with no matching end"


@dataclass
class SnippetAlreadyWritten(MetadataError):
    def message(self):
        return "Snippet file already exists, which means this tag is defined more than once in separate files."


@dataclass
class MetadataUnicodeError(MetadataError):
    err: Optional[UnicodeDecodeError] = None

    def message(self):
        return f" unicode error: {str(self.err)}"


def _tag_from_line(token, line):
    tag_start = line.find(token) + len(token)
    tag_end = line.find("]", tag_start)
    return line[tag_start:tag_end].strip()


def find_snippets(file: Path, errors: MetadataErrors) -> dict[str, Snippet]:
    snippets = {}
    open_tags = set()
    with open(file, encoding="utf-8") as snippet_file:
        try:
            for line_idx, line in enumerate(snippet_file.readlines()):
                if SNIPPET_START in line:
                    tag = _tag_from_line(SNIPPET_START, line)
                    if tag in snippets:
                        errors.append(
                            DuplicateSnippetStartError(
                                file=file, line=line_idx, tag=tag
                            )
                        )
                    else:
                        snippets[tag] = Snippet(
                            id=tag, file=file, line_start=line_idx, line_end=-1, code=""
                        )
                        open_tags.add(tag)
                elif SNIPPET_END in line:
                    tag = _tag_from_line(SNIPPET_END, line)
                    if tag not in snippets:
                        errors.append(
                            MissingSnippetStartError(file=file, line=line_idx, tag=tag)
                        )
                    elif tag not in open_tags:
                        errors.append(
                            DuplicateSnippetEndError(file=file, line=line_idx, tag=tag)
                        )
                    else:
                        open_tags.remove(tag)
                        snippets[tag].line_end = line_idx
                else:
                    for tag in open_tags:
                        snippets[tag].code += line
        except UnicodeDecodeError as err:
            errors.append(MetadataUnicodeError(file=file, err=err))
    for tag in open_tags:
        errors.append(
            MissingSnippetEndError(file=file, line=snippets[tag].line_start, tag=tag)
        )
    return snippets


def collect_snippets(root: Path) -> (dict[str, Snippet], MetadataErrors):
    snippets = {}
    errors = MetadataErrors()
    for file in get_files(root, validator_config.skip):
        snippets.update(find_snippets(file, errors))
    return snippets, errors


def clear(folder: Path):
    if folder.exists():
        rmtree(folder, True)
    folder.mkdir()


def write_snippets(root: Path, snippets: dict[str, Snippet]):
    errors = MetadataErrors()
    for tag in snippets:
        name = root / f"{tag}.txt"
        if name.exists():
            errors.append(SnippetAlreadyWritten(file=name))
        else:
            with open(name, "w", encoding="utf-8") as file:
                file.write(snippets[tag].code)
    return errors


def _tag_from_file(self, name):
    return f"{self.mirror}/{name}".replace("/", ".")


def write_snippet_file(folder: Path, snippet_file: Path):
    name = _tag_from_file(snippet_file)
    dest = folder / f"{name}.txt"
    if not dest.exists():
        copyfile(folder / snippet_file, dest)


if __name__ == "__main__":
    root = Path(__file__).parent.parent.parent
    snippets, errors = collect_snippets(root)
    print(f"Found {len(snippets)} snippets")
    out = root / ".snippets"
    clear(out)
    errors.maybe_extend(write_snippets(out, snippets))
    if len(errors) > 0:
        print(errors)
