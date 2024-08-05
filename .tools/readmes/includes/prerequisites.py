from macros import commented_block


def prerequisites(lang_config, service, customs: dict[str, str]):
    prereqs = []
    if lang_config["name"] != "C++":
        prereqs.append(
            para(
                "For prerequisites, see the ",
                link(title="README", href=f"{lang_config['readme']}#Prerequisites"),
                f" in the `{lang_config['base_folder']}` folder.",
            )
        )

    if lang_config["name"] == "Python" and lang_config["sdk_ver"] == 3:
        prereqs.append(
            para(
                "Install the packages required by these examples by running the following in a virtual environment:"
            )
        )
        prereqs.append(code_block("python -m pip install -r requirements.txt"))

    elif lang_config["name"] == "C++" and lang_config["sdk_ver"] == 1:
        prereqs.append(
            para(
                "Before using the code examples, first complete the installation and setup steps\n"
                "for ",
                link(
                    title="Getting Started",
                    href="https://docs.aws.amazon.com/sdk-for-cpp/v1/developer-guide/getting-started.html",
                ),
                "in the AWS SDK for\n"
                "C++ Developer Guide.\n"
                "This section covers how to get and build the SDK, and how to build your own code by using the SDK with a\n"
                "sample Hello World-style application.\n",
            )
        )
        prereqs.append(
            para(
                "Next, for information on code example structures and how to build and run the examples, see ",
                link(
                    title="Getting started with the AWS SDK for C++ code examples",
                    href="https://docs.aws.amazon.com/sdk-for-cpp/v1/developer-guide/getting-started-code-examples.html",
                ),
                ". ",
            ),
        )

    if service == "bedrock":
        prereqs.append(
            blockquote(
                "⚠ You must request access to a model before you can use it. If you try to use the model (with the API or console) before you have requested access to it, you will receive an error message. For more information, see ",
                link(
                    title="Model access",
                    href="https://docs.aws.amazon.com/bedrock/latest/userguide/model-access.html",
                ),
                ".",
            )
        )

    return h3(
        "Prerequisites",
        *prereqs,
        commented_block("custom.prerequisites", customs["prerequisites"]),
    )
