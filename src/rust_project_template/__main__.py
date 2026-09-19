# Builtin Imports
import sys
import traceback

# Pip Imports
import click

# Local Imports
from rust_project_template.project import Project
from rust_project_template.project_type import ProjectType

@click.command('create-project')
@click.argument('project_name', required=True, type=click.STRING)
@click.argument('project_type', required=True, type=click.Choice(['Executable', 'Library']))
@click.argument('project_author', required=True, type=click.STRING)
@click.argument('project_description', required=False, type=click.STRING, default='')
def main(project_name: str,
         project_type: str,
         project_author: str,
         project_description: str) -> int: # raises SystemExit
    try:
        # Create a new instance of Project
        project: Project = Project(
            project_name,
            ProjectType.EXECUTABLE if project_type == 'Executable' else ProjectType.LIBRARY,
            project_author,
            project_description)

        # Render the project
        project.render()

        return 0
    except Exception as e:
        traceback.print_exception(e)
        raise SystemExit(1)

if __name__ == '__main__': sys.exit(main())