def hello(example_sets, example_suffix="tablist", prefix, isSnapshot):
    hello = example_sets['Hello']
    if not hello.Examples:
        return block()
    example_prefix = f"prefix_" if prefix else ""
    include_docs = "" if isSnapshot else "file://AWSShared/code-samples/docs/"
    return block(
        para(emphasis(role="bold", "Get started")),
        block(
            collapsible(_props={"expand-section"="_collapse_all_"}, *[
                section(id=f"{example_prefix}example_{ex.ExampleId}_section",
                        info(
                            title(id=f"{example_prefix}example_{ex.ExampleId}_section.title", ex.Title),
                            titleabbrev(id=f"{example_prefix}example_{ex.ExampleId}_section.titleabbrev",ex.TitleAbbrev),
                            abstract(para(ex.Title))),
                    xi_include(href=f"{include_docs}{ex.ExampleId}_desc.xml"),
                    xi_include(href=f"{include_docs}{ex.ExampleId}_{example_suffix}.xml"),)
            ] for ex in hello.Examples)
        )
    )