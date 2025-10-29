# -*- coding: utf-8 -*-

"""
A Rust backend library aiming to provide high performance
implementations for OpenSpeleo project.
"""

import importlib.metadata
from typing import Any

__version__ = importlib.metadata.version("ospl_rcompass")


from ospl_rcompass import _rust_lib


def convert_xls_json_to_dat(data: dict[str, Any]) -> bytes:
    return _rust_lib.convert_xls_json_to_dat(data)
