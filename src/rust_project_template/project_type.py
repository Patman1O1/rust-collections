# Builtin Imports
from enum import Enum

class ProjectType(Enum):
    EXECUTABLE = 'Executable'
    LIBRARY = 'Library'

    def __str__(self) -> str: return self.value
