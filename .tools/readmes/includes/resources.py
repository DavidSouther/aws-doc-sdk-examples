def resources(sdk, service, lang_config, customs):
    return block(
        h2("Additional resources"),
        unordered_list(
            item(
                link(
                    href=service["guide"]["url"],
                    title=f"{service['short']} {service['guide']['subtitle']}",
                )
            ),
            item(
                link(href=service["api_ref"], title=f"{service['short']} API Reference")
            ),
            item(
                link(
                    href=lang_config["sdk_api_ref"],
                    title=f"{sdk['short']} {service['short']} reference",
                )
            ),
        ),
        commented_block("custom.resources", customs["resources"]),
    )
