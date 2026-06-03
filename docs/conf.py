# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information
import datetime
import resvg_py

project = "resvg_py"
author = resvg_py.__author__
copyright = f"2024-{datetime.date.today().year}, {author}"
release = resvg_py.__version__

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.intersphinx",
    "sphinx.ext.viewcode",
    "sphinx_tabs.tabs",
    "sphinx_copybutton",
    "sphinx_iconify",
    "sphinx_contributors",
    "sphinxcontrib.mermaid",
]

templates_path = ["_templates"]
exclude_patterns = ["_build", "Thumbs.db", ".DS_Store"]

# -- autodoc configuration ---------------------------------------------------
autodoc_member_order = "groupwise"
autodoc_typehints = "description"

# -- copybutton configuration ------------------------------------------------
copybutton_prompt_text = r">>> |\.\.\. "
copybutton_only_copy_prompt_lines = True

# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = "shibuya"
html_static_path = ["_static"]

html_theme_options = {
    "accent_color": "indigo",
    "github_url": "https://github.com/baseplate-admin/resvg-py",
}

# -- contributors configuration ----------------------------------------------
contributors_github_repo = "baseplate-admin/resvg-py"
