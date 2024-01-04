# Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
# SPDX-License-Identifier: Apache-2.0

import argparse
import config
import logging

from pathlib import Path

from sdk_examples.doc_gen import DocGen

from render import Renderer


def main():
    doc_gen = DocGen(Path(__path__).parent.parent.parent)

    lang_vers = []
    for sdk in doc_gen.sdks:
        vers = ", ".join([str(v) for v in doc_gen.sdks[sdk].versions])
        lang_vers.append(f"{sdk}: {vers}")

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "language",
        metavar="sdk_language",
        choices=[doc_gen.sdks[s].name for s in doc_gen.sdks],
        help="The language of the SDK. Choose from: %(choices)s.",
    )
    parser.add_argument(
        "sdk_version",
        help=f"The major version of the SDK. Must match a version of the specified SDK: {', '.join(lang_vers)}",
    )
    parser.add_argument(
        "service",
        metavar="service",
        choices=[doc_gen.services[s].name for s in doc_gen.services],
        help="The targeted service. Choose from: %(choices)s.",
    )
    parser.add_argument(
        "--svc_folder",
        help="Overrides the folder template to specify the service example folder.",
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
    args = parser.parse_args()

    if int(args.sdk_version) not in doc_gen.sdks[args.language].versions:
        parser.print_usage()
        print(
            f"writeme.py: error: argument sdk_verion: invalid choice for "
            f"{args.language}: {args.sdk_version} (for {args.language}, choose from "
            f"{', '.join([str(v) for v in doc_gen.sdks[args.language].versions])})"
        )
        return

    if args.verbose:
        logging.basicConfig(level=logging.DEBUG)

    try:
        renderer = Renderer(
            doc_gen,
            args.language,
            args.service,
            args.sdk_version,
            args.safe,
            svc_folder=args.svc_folder,
        )
        renderer.render()
    except Exception as err:
        print("*** Something went wrong! ***")
        raise err


if __name__ == "__main__":
    main()
