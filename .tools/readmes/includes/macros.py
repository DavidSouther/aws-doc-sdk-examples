def commented_block(name: str, blurb: str):
    return block(comment(f"{name}.start"), blurb, comment(f"{name}.end"))

def list_example(example):
    file_link = link(title=example['title_abbrev'], href=example['file'])
    api_ = example['api'] or ""
    if api_:
        api_ = f" ({api_})"
    return item(file_link, api_)

def list_examples(examples):
    unordered_list(*[item(example)] for example in examples)