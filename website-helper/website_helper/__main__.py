#!/usr/bin/env python3
# -*- coding: utf-8 -*-

from typing import Dict, List
import asyncio
import pathlib
import sys

from playwright.async_api import Browser, async_playwright

base_url = "http://localhost:9999/"

out_dir = pathlib.Path(sys.argv[1]).resolve()

finished_pages: Dict[str, str] = {}

pending_pages: List[str] = []


async def crawl_page(browser: Browser, url: str) -> None:
    page = await browser.new_page()

    print(f"Reading: {url}")

    await page.goto(url)
    await asyncio.sleep(1)

    content = await page.content()
    finished_pages[url] = content

    links = await page.query_selector_all("a")

    for link in links:
        next_url = await link.get_attribute("href")

        if next_url is not None and next_url.startswith("/"):
            next_url = base_url + next_url[1:]

        if next_url is not None and (
            (next_url.startswith(base_url) or next_url.startswith("/"))
            and next_url not in finished_pages
            and next_url not in pending_pages
        ):

            pending_pages.append(next_url)


async def crawl_pages() -> None:
    async with async_playwright() as playwright:
        browser = await playwright.firefox.launch()

        await crawl_page(browser, base_url)
        await crawl_page(browser, base_url + "404")

        while pending_pages:
            page = pending_pages.pop()

            await crawl_page(browser, page)

    for (url, content) in finished_pages.items():
        path = out_dir / url.removeprefix(base_url)

        if path == out_dir or url.endswith("/"):
            path = path / "index.html"

        else:
            path = path.with_suffix(".html")

        print(f"Creating: {url} -> {path}")

        path.parent.mkdir(parents=True, exist_ok=True)

        with path.open("w+") as f:
            f.write(content)


def main() -> None:
    asyncio.run(crawl_pages())


if __name__ == "__main__":
    main()
