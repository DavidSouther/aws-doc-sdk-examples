"""AWS SDK Code Examples Extractor
"""

# Always prefer setuptools over distutils
from setuptools import setup, find_packages
import pathlib

here = pathlib.Path(__file__).parent.resolve()

# Get the long description from the README file
long_description = (here / "README.md").read_text(encoding="utf-8")
setup(
    name="sdk_examples",
    version="0.1.0",
    description="Extract SDK Code Examples Data",
    long_description=long_description,
    long_description_content_type="text/markdown",
    url="https://github.com/awsdocs/aws-doc-sdk-examples",
    author="David Souther",
    author_email="dpsouth@amazon.com",
    classifiers=[
        "Development Status :: 4 - Beta",
        "Intended Audience :: Developers",
        "Topic :: Software Development :: Build Tools",
        "License :: OSI Approved :: Apache 2 License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Programming Language :: Python :: 3 :: Only",
    ],
    keywords="aws docs, aws code examples",
    packages=["sdk_examples"],
    python_requires=">=3.7, <4",
    project_urls={
        "Bug Reports": "https://github.com/awsdocs/aws-doc-sdk-examples/issues",
    },
)
