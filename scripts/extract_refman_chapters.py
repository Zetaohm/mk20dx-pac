#!/usr/bin/env python3
"""Extract K20 Reference Manual chapters into individual markdown files using PyMuPDF.

Usage:
    python3 scripts/extract_refman_chapters.py [--pdf PATH] [--outdir PATH]

Defaults:
    --pdf     reference/K20P64M72SF1RM.pdf
    --outdir  reference/refman_chapters
"""

import argparse
import re
import pymupdf
from pathlib import Path


def slugify(title: str) -> str:
    """Convert chapter title to a filesystem-friendly slug."""
    # Remove "Chapter N: " prefix
    title = re.sub(r"^Chapter\s+\d+:\s*", "", title)
    # Lowercase, replace non-alphanum with hyphens, collapse runs
    slug = re.sub(r"[^a-z0-9]+", "-", title.lower()).strip("-")
    return slug


def extract_page_text(page: pymupdf.Page) -> str:
    """Extract text from a single page, preserving structure."""
    blocks = page.get_text("blocks")  # (x0, y0, x1, y1, text, block_no, block_type)
    lines = []
    for b in blocks:
        if b[6] != 0:  # skip image blocks
            continue
        text = b[4].strip()
        if text:
            lines.append(text)
    return "\n\n".join(lines)


def extract_chapters(pdf_path: str, outdir: str) -> None:
    doc = pymupdf.open(pdf_path)
    toc = doc.get_toc()  # [(level, title, page_number), ...]

    # Identify L1 (chapter) entries and their page ranges
    chapters = []
    for i, (level, title, page) in enumerate(toc):
        if level == 1:
            chapters.append({"title": title, "start_page": page, "toc_entries": []})

    # Assign end pages (start of next chapter)
    for i in range(len(chapters)):
        if i + 1 < len(chapters):
            chapters[i]["end_page"] = chapters[i + 1]["start_page"]
        else:
            chapters[i]["end_page"] = len(doc) + 1

    # Collect sub-TOC entries for each chapter
    ch_idx = 0
    for level, title, page in toc:
        if level == 1:
            continue
        # Find which chapter this belongs to
        while ch_idx + 1 < len(chapters) and page >= chapters[ch_idx + 1]["start_page"]:
            ch_idx += 1
        chapters[ch_idx]["toc_entries"].append((level, title, page))

    out = Path(outdir)
    out.mkdir(parents=True, exist_ok=True)

    # Write index file
    index_lines = ["# K20 Sub-Family Reference Manual — Chapter Index\n"]
    index_lines.append(f"Source: `{pdf_path}`\n")
    index_lines.append(f"Total pages: {len(doc)}\n")

    for i, ch in enumerate(chapters):
        # Extract chapter number from title
        m = re.match(r"Chapter\s+(\d+)", ch["title"])
        ch_num = m.group(1) if m else str(i + 1)
        slug = slugify(ch["title"])
        filename = f"ch{ch_num.zfill(2)}-{slug}.md"
        num_pages = ch["end_page"] - ch["start_page"]

        index_lines.append(f"- [{ch['title']}]({filename}) ({num_pages} pages)")

        # Build markdown content for this chapter
        md_lines = [f"# {ch['title']}\n"]
        md_lines.append(f"Pages {ch['start_page']}–{ch['end_page'] - 1} "
                        f"({num_pages} pages)\n")

        # Add sub-TOC
        if ch["toc_entries"]:
            md_lines.append("## Contents\n")
            for level, title, page in ch["toc_entries"]:
                indent = "  " * (level - 2)
                md_lines.append(f"{indent}- {title} (p{page})")
            md_lines.append("")

        md_lines.append("---\n")

        # Extract text page by page (0-indexed internally, TOC is 1-indexed)
        for page_num in range(ch["start_page"] - 1, min(ch["end_page"] - 1, len(doc))):
            page = doc[page_num]
            text = extract_page_text(page)
            if text:
                md_lines.append(f"<!-- Page {page_num + 1} -->\n")
                md_lines.append(text)
                md_lines.append("")

        filepath = out / filename
        filepath.write_text("\n".join(md_lines), encoding="utf-8")
        print(f"  {filename} ({num_pages} pages)")

    # Write index
    index_path = out / "INDEX.md"
    index_path.write_text("\n".join(index_lines) + "\n", encoding="utf-8")
    print(f"\n  INDEX.md ({len(chapters)} chapters)")
    print(f"\nDone: {len(chapters)} chapters extracted to {outdir}/")


def main():
    parser = argparse.ArgumentParser(description="Extract K20 refman chapters to markdown")
    parser.add_argument("--pdf", default="reference/K20P64M72SF1RM.pdf",
                        help="Path to reference manual PDF")
    parser.add_argument("--outdir", default="reference/refman_chapters",
                        help="Output directory for markdown files")
    args = parser.parse_args()
    extract_chapters(args.pdf, args.outdir)


if __name__ == "__main__":
    main()
