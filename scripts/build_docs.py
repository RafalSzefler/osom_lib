# /// script
# requires-python = ">=3.11"
# dependencies = [
# ]
# ///
#
# This script runs `cargo doc` command with appropriate params (excludes what
# is not needed) and does proper cleanup.
import os
import subprocess
import tomllib
from typing import List
from dataclasses import dataclass

ROOT = os.path.realpath(os.path.join(os.path.dirname(__file__), ".."))
PROJECTS = os.path.join(ROOT, "projects")

@dataclass
class Project:
    name: str
    path: str
    is_private: bool

def get_all_projects() -> List[Project]:
    result = []
    for (root, dirs, files) in os.walk(PROJECTS):
        if "Cargo.toml" not in files:
            continue

        data = None
        with open(os.path.join(root, "Cargo.toml"), "rb") as fo:
            data = tomllib.load(fo)
        
        name = data["package"]["name"]
        is_private = name.startswith("_")
        project = Project(name, root, is_private)
        result.append(project)

    return result

def initial_cleaning():
    subprocess.run(["cargo", "clean", "--doc"], check=True)

def generate_docs(projects: List[Project]):
    cmd = ["cargo", "doc", "-r", "--no-deps", "--workspace"]
    for project in projects:
        if not project.is_private:
            continue
        cmd.append("--exclude")
        cmd.append(project.name)
    
    subprocess.run(cmd, check=True)

def final_cleanup():
    docs = os.path.join(ROOT, "target", "doc")
    lock = os.path.join(docs, ".lock")
    if os.path.exists(lock):
        os.remove(lock)


def main():
    initial_cleaning()
    projects = get_all_projects()
    generate_docs(projects)
    final_cleanup()

if __name__ == "__main__":
    main()
