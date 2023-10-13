# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information
import os

project = "cftime-rs"
copyright = "2023, Antoine Gibek"
author = "Antoine Gibek"

REPO_URL = "https://github.com/antscloud/cftime-rs"
# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    "sphinx.ext.autodoc",
    "sphinx.ext.napoleon",
    "sphinx.ext.linkcode",
    "myst_parser",
    "autoapi.extension",
]

autoapi_dirs = [os.path.abspath("../../../python/cftime_rs")]
autoapi_add_toctree_entry = False


def linkcode_resolve(domain, info):
    if domain != "py":
        return None
    if not info["module"]:
        return None
    filename = info["module"].replace(".", "/")
    return f"{REPO_URL}/tree/main/{filename}.py"


myst_heading_anchors = 3


source_suffix = {
    ".rst": "restructuredtext",
    ".txt": "markdown",
    ".md": "markdown",
}

# -- Options for autodoc ----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/extensions/autodoc.html#configuration


templates_path = ["_templates"]
exclude_patterns = []


# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_sidebars = {
    "**": [
        "navbar-logo.html",
        "search-field.html",
        "sbt-sidebar-nav.html",
    ]
}

# The theme to use for HTML and HTML Help pages.  See the documentation for
# a list of builtin themes.
#
html_theme = "sphinx_book_theme"

html_theme_options = {
    "repository_url": REPO_URL,
    "use_repository_button": True,
    "use_issues_button": True,
    "use_download_button": True,
    "use_sidenotes": True,
    "home_page_in_toc": False,
    "show_toc_level": 2,
}

html_static_path = ["_static"]

html_title = "cftime-rs"
