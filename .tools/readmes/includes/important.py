from macros import commented_block


def important(customs: dict[str, str]):
    return block(
        h2("⚠ Important"),
        unordered_list(
            item(
                "Running this code might result in charges to your AWS account. For more details, see ",
                link(href="https://aws.amazon.com/pricing/", title="AWS Pricing"),
                " and ",
                link(title="Free Tier", href="https://aws.amazon.com/free/"),
                ".",
            ),
            item("Running the tests might result in charges to your AWS account."),
            item(
                "We recommend that you grant your code least privilege. At most, grant only the minimum permissions required to perform the task. For more information, see ",
                link(
                    title="Grant least privilege",
                    title="https://docs.aws.amazon.com/IAM/latest/UserGuide/best-practices.html#grant-least-privilege",
                ),
                ".",
            ),
            item(
                "This code is not tested in every AWS Region. For more information, see ",
                link(
                    title="AWS Regional Services",
                    href="https://aws.amazon.com/about-aws/global-infrastructure/regional-product-services",
                ),
                ".",
            ),
        ),
        commented_block("custom.important", customs["important"]),
    )
