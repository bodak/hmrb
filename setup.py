#!/usr/bin/env python

import os
import re
from pathlib import Path
from typing import List

from setuptools import Extension, find_packages, setup

import hmrb as root

extension_modules = ("core", "lang", "node", "protobuffer", "response_pb2")

try:
    from setuptools_rust import RustExtension, Binding
except ImportError:
    import subprocess

    print(
        "\nsetuptools_rust is required before install - https://pypi.python.org/pypi/setuptools-rust"
    )
    print("attempting to install with pip...")
    print(subprocess.check_output(["pip", "install", "setuptools_rust"]))
    from setuptools_rust import RustExtension

try:
    from Cython.Build import cythonize

    # * Using Cython to compile
    extensions = cythonize(
        [f"hmrb/compat/v1/{module}.py" for module in extension_modules],
        compiler_directives={"language_level": "3str"},
    )
except ImportError:
    if all(
        [
            os.path.isfile(f"hmrb/compat/v1/{module}.c")
            for module in extension_modules
        ]
    ):
        # * Using pre-compiled C files
        extensions = [
            Extension(module, [f"hmrb/compat/v1/{module}.c"])
            for module in extension_modules
        ]
    else:
        # * Using native Python
        extensions = []

with open("README.md") as f:
    long_description = f.read()

def read_requirements(file: str) -> List[str]:
    if not Path(file).is_file():
        raise FileNotFoundError(file)
    with open(file) as fd:
        unparsed_requirements = fd.read()
        return re.findall(r"[\w-]+==[\d.]+", unparsed_requirements)

setup_params = dict(
    name='hmrb',
    version=root.__version__,
    packages=find_packages(".", exclude=("tests",)),
    zip_safe=False,
    include_package_data=False,
    description="Hammurabi",
    author="Babylon Health",
    author_email="kristian.boda@babylonhealth.com",
    url="https://github.com/babylonhealth/hmrb",
    license="Apache License 2.0",
    long_description=long_description,
    long_description_content_type="text/markdown",
    setup_requires=["cython<0.30", "setuptools_rust"],
    rust_extensions=[
        RustExtension(
            "hmrb",
            "hmrb/rust/Cargo.toml",
            debug=False,
            binding=Binding.NoBinding,
        )
    ],
    ext_modules=extensions,
    install_requires=read_requirements('requirements.txt'),
    entry_points={
        'console_scripts': [
            'hmrb = hmrb.__main__:main',
        ]
    }
)


def main() -> None:
    setup(**setup_params)


if __name__ == '__main__':
    main()
