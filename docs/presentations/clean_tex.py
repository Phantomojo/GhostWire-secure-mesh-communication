#!/usr/bin/env python3
"""
Clean pandoc's LaTeX output for the GhostWire Bible build.

pandoc passes raw HTML through as literal text, and our chapters use HTML for
alignment and page furniture. Strip the tags, keep the prose. Also removes the
duplicate markdown title block from the cover chapter, since the LaTeX
\\gwcover already renders a proper cover page.
"""
import glob
import os
import re
import sys

build = sys.argv[1]
cover = os.path.join(build, "00_cover_toc_index.tex")

# HTML tags -> LaTeX equivalents, applied before the bare-tag strip.
REPLACEMENTS = [
    (r"<br\s*/?>", r"\\\\ "),
    (r"</?div[^>]*>", ""),
    (r"</?p[^>]*>", ""),
    (r"</?span[^>]*>", ""),
    (r"</?center>", ""),
    (r"</?strong>", r"\\textbf{"),  # needs its own closer; handled below
    (r"</?em>", r"\\textit{"),
    (r"<img[^>]*src=[\"']([^\"']+)[\"'][^>]*>", None),  # image
]

# Self-closing/void tags that carry no content to keep.
VOID = re.compile(
    r"<(hr|img|br|input|meta|link|source)\b[^>]*>", re.I
)

TAG = re.compile(r"</?[a-zA-Z][^>]*>")
ATTRS = re.compile(r'<[^>]*?src=["\']([^"\']+)["\'][^>]*>', re.I)


def clean(path: str, is_cover: bool) -> None:
    with open(path, encoding="utf-8") as fh:
        text = fh.read()

    # Images: KEEP pandoc's \includegraphics — the rendered diagrams are the whole
    # point of this build step. Only the LaTeX cover macro's logo (which points at
    # an SVG pandoc cannot embed) is dropped, and that lives in ghostwire-brand.tex.

    # <strong>/<b> -> \textbf{...}  (paired, content preserved)
    text = re.sub(r"<(strong|b)\b[^>]*>(.*?)</\1>", r"\\textbf{\2}", text, flags=re.S | re.I)
    text = re.sub(r"<(em|i)\b[^>]*>(.*?)</\1>", r"\\textit{\2}", text, flags=re.S | re.I)

    for pat, rep in REPLACEMENTS:
        if rep is None:
            continue
        text = re.sub(pat, rep, text, flags=re.I)

    text = VOID.sub("", text)
    text = TAG.sub("", text)

    if is_cover:
        # The LaTeX cover page replaces the markdown title block; keep only the
        # reading-guide prose that follows.
        text = re.sub(r"\\section\*\{GhostWire Bible\}.*?(?=\\section)",
                      "", text, flags=re.S)
        text = re.sub(r"\\tableofcontents", "", text)

    # Each chapter carries its own "Table of Contents" subsection. The PDF has
    # one document-level \tableofcontents instead, so drop the per-chapter ones.
    text = re.sub(
        r"\\subsection\{Table of Contents\}.*?(?=\\subsection\{|\\section\{)",
        "",
        text,
        flags=re.S,
    )
    # ...and any unenumerated manual contents list that follows one.
    text = re.sub(
        r"\\begin\{enumerate\}\s*\\def\\labelenumi\{\\arabic\{\w+\}\.\}\s*\\tightlist.*?\\end\{enumerate\}",
        "",
        text,
        flags=re.S,
    )

    # Collapse the blank lines the removals leave behind.
    text = re.sub(r"\n{3,}", "\n\n", text)

    with open(path, "w", encoding="utf-8") as fh:
        fh.write(text)


files = sorted(glob.glob(os.path.join(build, "*.tex")))
for path in files:
    if os.path.basename(path) in {"body.tex", "full.tex", "ghostwire-brand.tex"}:
        continue
    clean(path, os.path.basename(path) == "00_cover_toc_index.tex")

print(f"cleaned {len(files) - 1} chapter files")
