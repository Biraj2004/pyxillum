import os
import sys

# Banned terms per clean-room legal immunity rules
banned_terms = [
    "com.spled", "scenex", "scene-x", "spled"
]

found = 0
for root, dirs, files in os.walk('.'):
    if any(p in root for p in ['.git', 'target', 'dist', 're-workspace']):
        continue
    for f in files:
        if f.endswith(('.md', '.rs', '.toml', '.rc', '.manifest', '.wxs', '.json', '.py', '.svg')):
            path = os.path.join(root, f)
            with open(path, 'r', encoding='utf-8', errors='ignore') as fp:
                for idx, line in enumerate(fp, 1):
                    lower = line.lower()
                    for term in banned_terms:
                        if term in lower:
                            # Skip this script itself
                            if "audit_cleanroom.py" in path:
                                continue
                            print(f'Banned term "{term}" in {path}:{idx}: {line.strip()}')
                            found += 1

print(f'Total banned terms found: {found}')
sys.exit(1 if found > 0 else 0)
