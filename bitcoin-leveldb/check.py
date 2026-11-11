#!/usr/bin/env python3
"""Run the discovered LevelDB subsystem test suite with finite resource limits."""
from pathlib import Path
import runpy
import sys

sys.dont_write_bytecode = True
if __name__ == '__main__':
    repository = Path(__file__).absolute().parent.parent
    runner = repository / 'contrib/test/leveldb-subsystem.py'
    sys.argv = [str(runner), '--repo', str(repository), *sys.argv[1:]]
    runpy.run_path(str(runner), run_name='__main__')
