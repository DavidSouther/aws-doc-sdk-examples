from macros import commented_block

def overview(sdk, service, customs):
    return block(
        h2("Overview"),
        para(f"Shows how to use the {sdk['long']} to work with {service['long']}."),
        commented_block("custom.overview", customs['overview']),
        para(emphasis(f"{service['short']} {service["blurb"]}"))
    )
