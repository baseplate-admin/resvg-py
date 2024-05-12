import resvg_py


def test_version_is_string():
    assert isinstance(resvg_py.__version__,str)  