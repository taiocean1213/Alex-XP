"""Render the GitHub wiki Home page from README.md.

The README stays the source of truth. This script rewrites relative links so
they still open from the wiki, and prepends a line that tells editors to
change the README instead of the wiki.
"""

import re
import sys

BLOB = "https://github.com/taiocean1213/Alex-XP/blob/main/"
NOTICE = (
    "> This page is generated from "
    "[README.md](https://github.com/taiocean1213/Alex-XP/blob/main/README.md). "
    "Edit that file. Changes made here are overwritten.\n\n"
)
LINK = re.compile(r"\[([^\]]*)\]\(([^)\s]+)\)")


def rewrite_link(label: str, url: str) -> str:
    if url.startswith(("http://", "https://", "mailto:", "#")):
        return f"[{label}]({url})"
    path = url[2:] if url.startswith("./") else url
    return f"[{label}]({BLOB}{path})"


def rewrite_line(line: str) -> str:
    return LINK.sub(lambda match: rewrite_link(match.group(1), match.group(2)), line)


def render(readme: str) -> str:
    rendered = []
    in_fence = False
    for line in readme.splitlines(keepends=True):
        if line.lstrip().startswith("```"):
            in_fence = not in_fence
            rendered.append(line)
            continue
        rendered.append(line if in_fence else rewrite_line(line))
    body = "".join(rendered)
    if body and not body.endswith("\n"):
        body += "\n"
    return NOTICE + body


def main() -> None:
    if len(sys.argv) != 3:
        print("usage: render_wiki_home.py README.md Home.md", file=sys.stderr)
        sys.exit(2)
    source = open(sys.argv[1], encoding="utf-8").read()
    with open(sys.argv[2], "w", encoding="utf-8", newline="\n") as out:
        out.write(render(source))


if __name__ == "__main__":
    main()
