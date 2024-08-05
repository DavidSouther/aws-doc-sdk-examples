from typing import Optional

from macros import list_examples, commented_block
from prerequisites import prerequisites


def example_block(title: str, description: Optional[str], examples):
    return block(h3(title), description, list_examples(examples))


def code_examples(
    hello,
    actions,
    scenarios,
    custom_categories: dict[str, list],
    crosses,
    customs: dict[str, str],
):
    blocks = []
    if hello:
        blocks.append(example_block("Get started", None, hello))
    if actions:
        blocks.append(
            example_block(
                "Single actions",
                "Code excerpts that show you how to call individual service functions.",
                actions,
            )
        )
    if scenarios:
        blocks.append(
            example_block(
                "Scenarios",
                "Code examples that show you how to accomplish a specific task by calling multiple functions within the same service. ",
                scenarios,
            )
        )

    for cat, examples in custom_categories.items():
        blocks.append(example_block(cat, None, examples))

    if crosses:
        blocks.append(
            example_block(
                "Cross-service examples",
                "Sample applications that work across multiple AWS services.",
                crosses,
            )
        )

    return block(
        h2("Code Examples"),
        prerequisites(),
        *blocks,
        commented_block("custom.examples", customs["examples"])
    )
