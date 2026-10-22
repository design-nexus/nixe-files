#!/usr/bin/env python3
"""Check the installed decoder boundary using trusted fixtures and a private sandbox."""
from pathlib import Path
import subprocess
root = Path(__file__).resolve().parents[1]
binary = root/'target/release/nixe-files'
helper = root/'target/release/nixe-files-rar'
for fixture, password, content in [('version.rar', None, b'unrar-0.4.0'), ('encrypted.rar', b'unrar', b'target\nCargo.lock\n')]:
    command = ['bwrap','--unshare-all','--die-with-parent','--new-session','--clearenv',
               '--ro-bind','/usr','/usr','--symlink','usr/lib','/lib','--symlink','usr/lib','/lib64',
               '--symlink','usr/bin','/bin','--proc','/proc','--dev','/dev','--tmpfs','/tmp','--dir','/app',
               '--ro-bind',str(binary),'/app/strata','--ro-bind',str(helper),'/app/nixe-files-rar',
               '--ro-bind',str(root/'tests/fixtures/rar'/fixture),'/input.rar',
               '--','/app/strata','--preview-helper','extract-rar','/input.rar']
    if password is not None: command.append('0')
    result = subprocess.run(command, input=password, capture_output=True, timeout=20)
    assert result.returncode == 0, result.stderr.decode(errors='replace')
    assert result.stdout.startswith(b'STRRAR03') and content in result.stdout
print('PASS: sandboxed RAR helper IPC, plain and password-protected archives')
