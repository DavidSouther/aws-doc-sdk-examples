#!/usr/bin/env python3
# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

import argparse
import config
import logging
import os
import sys
from pathlib import Path
from render import Renderer


def main():
    doc_gen = DocGen(Path(__path__).parent.parent.parent)

    lang_vers = []
    for sdk in doc_gen.sdks:
        for v in doc_gen.sdks[sdk].versions:
            lang_vers.append(f"{sdk}:{v}")

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "--languages",
        choices=lang_vers + ["all"],
        nargs="+",
        help="The languages of the SDK. Choose from: %(choices)s.",
        default=["all"],
    )
    parser.add_argument(
        "--services",
        choices={**doc_gen.services, "all": {}},
        nargs="+",
        help="The targeted service. Choose from: %(choices)s.",
        default=["all"],
    )
    parser.add_argument(
        "--safe",
        action="store_true",
        help=f"Save a copy of the original README as the 'saved_readme' value specified in config.py ({config.saved_readme}).",
    )
    parser.add_argument(
        "--verbose",
        action="store_true",
        help="When set, output verbose debugging info.",
    )
    parser.add_argument(
        "--dry-run",
        action="store_true",
        dest="dry_run",
        help="In dry run, compare current vs generated and exit with failure if they do not match.",
        default=True,  # Change this to default false when we're ready to use this generally.
    )
    parser.add_argument("--no-dry-run", dest="dry_run", action="store_false")
    args = parser.parse_args()

    if "all" in args.languages:
        args.languages = lang_vers

    if "all" in args.services:
        args.services = [*doc_gen.services().keys()]

    if args.verbose:
        logging.basicConfig(level=logging.DEBUG)

    logging.debug(f"Args configuration: {args}")

    if args.dry_run:
        logging.info("Dry run, no changes will be made.")

    skipped = []
    failed = []

    for language_and_version in args.languages:
        (language, version) = language_and_version.split(":")
        if int(version) not in doc_gen.sdks[language].versions:
            logging.debug(f"Skipping {language}:{version}")
        else:
            for service in args.services:
                try:
                    logging.debug(f"Rendering {language}:{version}:{service}")
                    renderer = Renderer(
                        doc_gen, language, service, int(version), args.safe
                    )

                    readme_filename, readme_text = renderer.render()
                    if args.dry_run:
                        with open(readme_filename, "r", encoding="utf-8") as f:
                            readme_current = f.read()
                        if readme_current != readme_text:
                            failed.append(f"{language}:{version}:{service}")
                    else:
                        if args.safe and Path(readme_filename).exists():
                            os.rename(
                                readme_filename,
                                f'{renderer.lang_config["service_folder"]}/{config.saved_readme}',
                            )
                        with open(readme_filename, "w", encoding="utf-8") as f:
                            f.write(readme_text)
                        print(f"Updated {readme_filename}.")
                except FileNotFoundError:
                    skip = f"{language}:{version}:{service}"
                    skipped.append(skip)
                except Exception:
                    skip = f"{language}:{version}:{service}"
                    logging.exception(
                        f"Exception rendering {skip}",
                    )

    skip_list = "\n\t".join(skipped)
    logging.info(f"Run complete. Skipped: {skip_list}")
    if len(failed) > 0:
        failed_list = "\n\t".join(failed)
        logging.warning(f"READMEs with incorrect formatting:\n\t{failed_list}")
    return len(failed)


if __name__ == "__main__":
    sys.exit(main())
