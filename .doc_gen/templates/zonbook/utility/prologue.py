def prologue(isSnapshot):
    prologue_xml_literal = """
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE chapter PUBLIC "-//OASIS//DTD DocBook XML V4.5//EN" "file://zonbook/docbookx.dtd"[
    <!ENTITY % xinclude SYSTEM "file://AWSShared/common/xinclude.mod">
    %xinclude;
"""
    if isSnapshot:
        prologue_xml_literal += (
            """<!ENTITY % phrases-code-examples SYSTEM "phrases-code-examples.ent">"""
        )
    else:
        prologue_xml_literal += """<!ENTITY % phrases-code-examples SYSTEM "file://AWSShared/code-samples/docs/phrases-code-examples.ent">"""
    prologue_xml_literal += """
    %phrases-code-examples;
    <!ENTITY % phrases-shared SYSTEM "file://AWSShared/common/phrases-shared.ent">
    %phrases-shared;
]>
"""
    return prologue_xml_literal
