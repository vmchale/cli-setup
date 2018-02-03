import re
from thefuck.utils import replace_argument

def match(command):
    return ('Did you mean ' in command.output and 'The subcommand' in command.output and 'wasn\'t recognized' in command.output)

def get_new_command(command):
    broken = command.script_parts[1]
    fix = re.findall(r'Did you mean \'([a-zA-z]*)\'', command.output)[0]

    return replace_argument(command.script, broken, fix)

enabled_by_default = True

priority = 100
