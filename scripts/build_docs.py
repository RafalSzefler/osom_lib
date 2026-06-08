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
from typing import List, Iterator
from dataclasses import dataclass

ROOT = os.path.realpath(os.path.join(os.path.dirname(__file__), ".."))
PROJECTS = os.path.join(ROOT, "projects")

@dataclass(frozen=True)
class Project:
    name: str
    path: str
    is_private: bool
    dependencies: frozenset[str]

def get_all_projects() -> List[Project]:
    result = []
    for (root, dirs, files) in os.walk(PROJECTS):
        if "Cargo.toml" not in files:
            continue

        data = None
        with open(os.path.join(root, "Cargo.toml"), "rb") as fo:
            data = tomllib.load(fo)
        
        name = data["package"]["name"]
        is_private = name.startswith("priv_")
        dependencies = set()
        for dependency in data["dependencies"]:
            if dependency.startswith("osom_lib_"):
                dependencies.add(dependency)
        project = Project(
            name=name,
            path=root,
            is_private=is_private,
            dependencies=dependencies,
        )
        result.append(project)

    return result

def initial_cleaning():
    subprocess.run(["cargo", "clean", "--doc"], check=True)

def generate_docs(projects: List[Project]):
    core_cmd = ["cargo", "doc", "-r", "--all-features", "--no-deps"]
    for project in projects:
        if project.is_private:
            continue
        cmd = core_cmd.copy()
        cmd.append("--package")
        cmd.append(project.name)
        subprocess.run(cmd, check=True)

def traverse_dependencies(project: Project, project_map: dict[str, Project], seen: set[str]) -> Iterator[Project]:
    if project.name in seen:
        return
    seen.add(project.name)

    yield project

    for dependency in project.dependencies:
        if dependent_project := project_map.get(dependency):
            if dependent_project.name in seen:
                continue
            yield from traverse_dependencies(dependent_project, project_map, seen)

def topologically_sort_projects(projects: List[Project]) -> List[Project]:
    project_map = {project.name: project for project in projects}
    result = []
    seen = set()

    for project in projects:
        for dependency in traverse_dependencies(project, project_map, seen):
            result.append(dependency)

    result.reverse()

    return result

def final_cleanup():
    docs = os.path.join(ROOT, "target", "doc")
    for (root, dirs, files) in os.walk(docs):
        for file in files:
            if file == ".lock":
                os.remove(os.path.join(root, file))

def main():
    initial_cleaning()
    projects = get_all_projects()
    projects = topologically_sort_projects(projects)
    generate_docs(projects)
    final_cleanup()

if __name__ == "__main__":
    main()
