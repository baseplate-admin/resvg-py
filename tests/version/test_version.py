from importlib.metadata import version

import resvg_py


def test_version_matches_distribution_metadata():
    assert resvg_py.__version__ == version("resvg_py")


def test_resvg_version_is_string():
    assert isinstance(resvg_py.__resvg_version__, str)
