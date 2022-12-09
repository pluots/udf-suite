#!/usr/bin/env python3

import argparse
import re
from datetime import date
from enum import Enum
from glob import glob
from inspect import cleandoc

VERSION_PATTERN = 'version = "(\d+)\.(\d+)\.(\d+)"'
CHANGELOG_TEMPLATE = (
    cleandoc(
        """
  <!-- next-header -->\n
  ## [Unreleased] - ReleaseDate\n
  ### Added\n
  ### Changed\n
  ### Removed\n
"""
    )
    + "\n"
)


def get_args():
    parser = argparse.ArgumentParser(description="Release a new UDF suite version.")
    parser.add_argument("level", choices=("major", "minor", "patch"))
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--tag", required=True)
    args = parser.parse_args()
    return args


def get_cargo_tomls() -> list[str]:
    return glob("**/Cargo.toml")


class SemvarLevel(Enum):
    MAJOR = 1
    MINOR = 2
    PATCH = 3


def get_next_semvar(groups: tuple[str, str, str], level: SemvarLevel) -> str:
    s_major, s_minor, s_patch = groups
    major, minor, patch = int(s_major), int(s_minor), int(s_patch)
    if level == SemvarLevel.MAJOR:
        major += 1
        minor, patch = 0, 0
    elif level == SemvarLevel.MINOR:
        minor += 1
        patch = 0
    elif level == SemvarLevel.PATCH:
        patch += 1

    return f"{major}.{minor}.{patch}"


def update_all_tomls(level: SemvarLevel, execute: bool = False) -> str:
    next_version: str | None = None
    next_version_toml: str | None = None

    for toml in get_cargo_tomls():
        with open(toml, "r") as f:
            lines = f.readlines()

        res = []

        for line in lines:
            match = re.match(VERSION_PATTERN, line)
            if match and next_version is None:
                next_version = get_next_semvar(
                    match.groups(), SemvarLevel[level.upper()]
                )
                next_version_toml = f'version = "{next_version}"'

            if match:
                res.append(next_version_toml)
                print(f"{toml}: updated '{line.strip()}' -> '{next_version}'")
            else:
                res.append(line)

        if execute:
            with open(toml, "w") as f:
                f.writelines(res)

    assert next_version is not None

    return next_version


def update_changelog(version: str, tag_name: str, execute: bool):
    with open("CHANGELOG.md", "r") as f:
        s = f.read()

    s = s.replace("[Unreleased]", f"[{version}]")
    s = s.replace("...HEAD", f"[{tag_name}]", 1)
    s = s.replace("ReleaseDate", f"{date.today()}", 1)
    s = s.replace("<!-- next-header -->", CHANGELOG_TEMPLATE, 1)
    s = s.replace(
        "<!-- next-url -->",
        cleandoc(
            f"""
        <!-- next-url -->

        [Unreleased]: https://github.com/pluots/udf-suite/compare/{tag_name}...HEAD\
    """
        ),
        1,
    )

    print(s)

    if execute:
        with open("CHANGELOG.md", "w") as f:
            f.write(s)


def main():
    args = get_args()

    if args.execute:
        print("Executing\n")
    else:
        print("Starting dry run\n")

    print("Updating Cargo.toml files")
    next_version = update_all_tomls(args.level, args.execute)

    print("\nUpdating changelog")
    update_changelog(next_version, args.tag, args.execute)


if __name__ == "__main__":
    main()
