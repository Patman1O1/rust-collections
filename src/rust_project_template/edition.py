# Builtin Imports
import re
from typing import Final

class Edition(object):
    PATTERN: Final[re.Pattern[str]] = re.compile(r'(\d{4})')

    def __init__(self, year: str) -> None: # raises TypeError, PatternError
        self.year: str = year
        return

    def __str__(self) -> str: return self._year

    @property
    def year(self) -> str: return self._year

    @year.setter
    def year(self, value: str) -> None: # raises TypeError, PatternError
        if not isinstance(value, str):
            raise TypeError(f'Expected \'value\' to be of type {str.__name__}' +
                            f', got {type(value).__name__}')
        if not Edition.PATTERN.match(value):
            raise re.PatternError('Expected \'value\' to be a 4 character numeric string' +
                                  f', got {value}')
        self._year: str = value
        return

LATEST_EDITION: Final[Edition] = Edition('2024')
