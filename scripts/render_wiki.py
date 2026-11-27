"""Publish the docs as a GitHub wiki.

The files under docs/ are the source of truth. This script writes one wiki
page per doc, rewrites links between those pages so they stay on the wiki,
and builds the sidebar a first visit uses.
"""

import posixpath
import re
import sys
from pathlib import Path

BLOB = "https://github.com/taiocean1213/Alex-XP/blob/main/"
LINK = re.compile(r"\[([^\]]*)\]\(([^)\s]+)\)")

# source path, wiki page name, sidebar label, sidebar section
PAGES = [
    ("docs/welcome.md", "Home", "Home", "Start"),
    ("docs/tutorial.md", "Tutorial", "Tutorial", "Start"),
    ("docs/examples.md", "Examples", "Examples", "Start"),
    ("docs/setup.md", "Setup", "Setup", "Start"),
    ("docs/everyday.md", "Everyday", "Everyday macros", "Language"),
    ("docs/ownership.md", "Ownership", "Ownership", "Language"),
    ("docs/oop.md", "OOP", "OOP", "Language"),
    ("docs/solid.md", "SOLID", "SOLID", "Language"),
    ("docs/index.md", "Guide", "Full guide", "Language"),
    ("docs/patterns/factory-method.md", "Factory-Method", "Factory Method", "Creational"),
    ("docs/patterns/abstract-factory.md", "Abstract-Factory", "Abstract Factory", "Creational"),
    ("docs/patterns/builder.md", "Builder", "Builder", "Creational"),
    ("docs/patterns/prototype.md", "Prototype", "Prototype", "Creational"),
    ("docs/patterns/singleton.md", "Singleton", "Singleton", "Creational"),
    ("docs/patterns/adapter.md", "Adapter", "Adapter", "Structural"),
    ("docs/patterns/bridge.md", "Bridge", "Bridge", "Structural"),
    ("docs/patterns/composite.md", "Composite", "Composite", "Structural"),
    ("docs/patterns/decorator.md", "Decorator", "Decorator", "Structural"),
    ("docs/patterns/facade.md", "Facade", "Facade", "Structural"),
    ("docs/patterns/flyweight.md", "Flyweight", "Flyweight", "Structural"),
    ("docs/patterns/proxy.md", "Proxy", "Proxy", "Structural"),
    ("docs/patterns/chain.md", "Chain", "Chain of Responsibility", "Behavioral"),
    ("docs/patterns/command.md", "Command", "Command", "Behavioral"),
    ("docs/patterns/interpreter.md", "Interpreter", "Interpreter", "Behavioral"),
    ("docs/patterns/iterator.md", "Iterator", "Iterator", "Behavioral"),
    ("docs/patterns/mediator.md", "Mediator", "Mediator", "Behavioral"),
    ("docs/patterns/memento.md", "Memento", "Memento", "Behavioral"),
    ("docs/patterns/observer.md", "Observer", "Observer", "Behavioral"),
    ("docs/patterns/state.md", "State", "State", "Behavioral"),
    ("docs/patterns/strategy.md", "Strategy", "Strategy", "Behavioral"),
    ("docs/patterns/template-method.md", "Template-Method", "Template Method", "Behavioral"),
    ("docs/patterns/visitor.md", "Visitor", "Visitor", "Behavioral"),
]

PAGE_BY_PATH = {source: name for source, name, _label, _section in PAGES}


def rewrite_url(source_rel: str, url: str) -> str:
    if url.startswith(("http://", "https://", "mailto:", "#")):
        return url
    if "#" in url:
        path_part, anchor = url.split("#", 1)
        anchor = "#" + anchor
    else:
        path_part, anchor = url, ""
    if path_part.startswith("./"):
        path_part = path_part[2:]
    if path_part == "":
        return anchor
    target = posixpath.normpath(posixpath.join(posixpath.dirname(source_rel), path_part))
    page = PAGE_BY_PATH.get(target)
    if page:
        return page + anchor
    return BLOB + target + anchor


def rewrite_line(source_rel: str, line: str) -> str:
    return LINK.sub(
        lambda match: f"[{match.group(1)}]({rewrite_url(source_rel, match.group(2))})",
        line,
    )


def render_page(source_rel: str, text: str) -> str:
    rendered = []
    in_fence = False
    for line in text.splitlines(keepends=True):
        if line.lstrip().startswith("```"):
            in_fence = not in_fence
            rendered.append(line)
            continue
        rendered.append(line if in_fence else rewrite_line(source_rel, line))
    body = "".join(rendered)
    if body and not body.endswith("\n"):
        body += "\n"
    note = (
        "\n---\n\n"
        f"Edit [{source_rel}]({BLOB}{source_rel}) in the repository. "
        "This wiki page is overwritten when that file changes.\n"
    )
    return body + note


def render_sidebar() -> str:
    sections = []
    for _source, name, label, section in PAGES:
        if not sections or sections[-1][0] != section:
            sections.append((section, []))
        sections[-1][1].append(f"- [{label}]({name})")
    lines = []
    for section, items in sections:
        lines.append(f"**{section}**")
        lines.append("")
        lines.extend(items)
        lines.append("")
    return "\n".join(lines).rstrip() + "\n"


def render_footer() -> str:
    return (
        "These pages are published from "
        "[docs/](https://github.com/taiocean1213/Alex-XP/tree/main/docs). "
        "Edit the files there. Changes made on the wiki are overwritten.\n"
    )


def render_tree(root: Path, output: Path) -> None:
    output.mkdir(parents=True, exist_ok=True)
    for source, name, _label, _section in PAGES:
        text = (root / source).read_text(encoding="utf-8")
        (output / f"{name}.md").write_text(render_page(source, text), encoding="utf-8", newline="\n")
    (output / "_Sidebar.md").write_text(render_sidebar(), encoding="utf-8", newline="\n")
    (output / "_Footer.md").write_text(render_footer(), encoding="utf-8", newline="\n")


def main() -> None:
    if len(sys.argv) != 3:
        print("usage: render_wiki.py REPO_ROOT OUTPUT_DIR", file=sys.stderr)
        sys.exit(2)
    render_tree(Path(sys.argv[1]), Path(sys.argv[2]))


if __name__ == "__main__":
    main()
