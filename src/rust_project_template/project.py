# Builtin Imports
import os
from enum import Enum
from pathlib import Path
from typing import Final
import shutil

# Pip Imports
from jinja2 import Environment, FileSystemLoader

# Local Imports
from rust_project_template.edition import Edition, LATEST_EDITION
from rust_project_template.format import to_screaming_case, to_pascal_case
from rust_project_template.project_type import ProjectType

class Project(object):
    ROOT: Final[Path] = Path(__file__).resolve().parents[2]

    @staticmethod
    def _create_env(search_path: Path) -> Environment:
        env: Environment = Environment(
            loader=FileSystemLoader(search_path),
            keep_trailing_newline=True,
            trim_blocks=True,
            lstrip_blocks=True,
        )

        env.filters['to_screaming_case'] = to_screaming_case
        env.filters['to_pascal_case'] = to_pascal_case

        return env

    def __init__(self,
                 name: str,
                 project_type: ProjectType,
                 author: str,
                 description: str='',
                 edition: Edition=LATEST_EDITION) -> None: # raises TypeError
        self.name: str = name
        self.type: ProjectType = project_type
        self.author: str = author
        self.edition: Edition = edition
        self.description: str = description
        return

    @property
    def name(self) -> str: return self._name

    @property
    def type(self) -> ProjectType: return self._type

    @property
    def author(self) -> str: return self._author

    @property
    def edition(self) -> Edition: return self._edition

    @name.setter
    def name(self, value: str) -> None: # raises TypeError
        if not isinstance(value, str):
            raise TypeError(f'Expected \'value\' to be of type {str.__name__}' +
                            f', got {type(value).__name__}')
        self._name: str = value
        return

    @type.setter
    def type(self, value: ProjectType) -> None: # raises TypeError
        if not isinstance(value, ProjectType):
            raise TypeError(f'Expected \'value\' to be of type {ProjectType.__name__}' +
                            f', got {type(value).__name__}')
        self._type: ProjectType = value
        return

    @author.setter
    def author(self, value: str) -> None: # raises TypeError
        if not isinstance(value, str):
            raise TypeError(f'Expected \'value\' to be of type {str.__name__}' +
                            f', got {type(value).__name__}')
        self._author: str = value
        return

    @edition.setter
    def edition(self, value: Edition) -> None: # raises TypeError
        if not isinstance(value, Edition):
            raise TypeError(f'Expected \'value\' to be of type {Edition.__name__}' +
                            f', got {type(value).__name__}')
        self._edition: Edition = value
        return

    def render(self) -> None:
        template_dir: Path = Project.ROOT/'template'
        env = Project._create_env(template_dir)

        for path in template_dir.rglob('*.j2'):
            rel: Path = path.relative_to(template_dir)

            # Interpolate {{ }} in every path segment, then drop the .j2 suffix
            parts: list[str] = [env.from_string(p).render(project=self) for p in rel.parts]
            parts[-1] = parts[-1].removesuffix('.j2')
            dest: Path = Project.ROOT.joinpath(*parts)

            dest.parent.mkdir(parents=True, exist_ok=True)
            dest.write_text(
                env.get_template(rel.as_posix()).render(project=self), encoding='utf-8')

        # Write the project description to the README.md file
        with open(Project.ROOT/'README.md', 'w', encoding='utf-8') as readme_md:
            readme_md.write(f'# {self.name}\n\n{self.description}\n')

        # Remove template/
        shutil.rmtree(Project.ROOT/template_dir)

        # Remove .github/
        shutil.rmtree(Project.ROOT/'.github')

        # Remove Python related files and directories
        os.unlink(Project.ROOT/'pyproject.toml')
        shutil.rmtree(Project.ROOT/'src'/'rust_project_template')
        shutil.rmtree(Project.ROOT/'src'/'rust_project_template.egg-info')

        # Remove Rust related files
        if self.type == ProjectType.EXECUTABLE:
            os.unlink(Project.ROOT/'src'/'lib.rs')
        elif self.type == ProjectType.LIBRARY:
            os.unlink(Project.ROOT/'src'/'main.rs')
        return