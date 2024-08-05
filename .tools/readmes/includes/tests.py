cpp_test = """
cd <BUILD_DIR>
cmake <path-to-root-of-this-source-code> -DBUILD_TESTS=ON
make
ctest
"""


def tests(lang_config):
    test_blocks = []

    if lang_config["name"] != "C++":
        test_blocks.append(
            para(
                "To find instructions for running these tests, see the ",
                link(title="README", href=f"{lang_config['readme']}#Tests"),
                f"in the `{lang_config['base_folder']}` folder.",
            )
        )

    if lang_config["name"] == "C++" and lang_config["sdk_ver"] == 1:
        test_blocks.append(code_block(lang="sh", code=cpp_test))

    return block(
        blockquote("⚠ Running tests might result in charges to your AWS account."),
        *test_blocks,
        comment_block("custom.tests", customs["tests"]),
    )
