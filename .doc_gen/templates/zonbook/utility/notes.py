def note_example_types(actions, scenarios, crosses):
    blocks = []
    if actions:
        blocks.append(
            para(
                emphasis("Actions"),
                " are code excerpts from larger programs and must be run in context. While actions show you how to call individual service functions, you can see actions in context in their related scenarios and cross-service examples.",
            )
        )
    if scenarios:
        blocks.append(
            para(
                emphasis("Scenarios"),
                " are code examples that show you how to accomplish a specific task by calling multiple functions within the same service.",
            )
        )
    if crosses:
        blocks.append(
            para(
                emphasis("Cross-service examples"),
                " are sample applications that work across multiple &AWS-services;.",
            )
        )
    return blocks


def note_complete_list():
    return para(
        "For a complete list of &AWS; SDK developer guides and code examples, see ",
        xref(
            linkend="sdk-general-information-section",
            endterm="sdk-general-information-section.title",
        ),
        ". This topic also includes information about getting started and details about previous SDK versions.",
    )
