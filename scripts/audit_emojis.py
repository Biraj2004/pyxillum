import os
import sys

emoji_ranges = [
    (0x1F600, 0x1F64F), (0x1F300, 0x1F5FF), (0x1F680, 0x1F6FF),
    (0x1F700, 0x1F77F), (0x1F780, 0x1F7FF), (0x1F800, 0x1F8FF),
    (0x1F900, 0x1F9FF), (0x1FA00, 0x1FA6F), (0x1FA70, 0x1FAFF),
    (0x2600, 0x26FF), (0x2700, 0x27BF)
]

def has_emoji(text):
    for ch in text:
        cp = ord(ch)
        for start, end in emoji_ranges:
            if start <= cp <= end:
                return True
    return False

found = 0
for root, dirs, files in os.walk('.'):
    if any(p in root for p in ['.git', 'target', 'dist', 're-workspace']):
        continue
    for f in files:
        if f.endswith(('.md', '.rs', '.toml', '.rc', '.manifest', '.wxs', '.json', '.svg', '.py')):
            path = os.path.join(root, f)
            with open(path, 'r', encoding='utf-8', errors='ignore') as fp:
                for idx, line in enumerate(fp, 1):
                    if has_emoji(line):
                        print(f'Emoji in {path}:{idx}: {line.strip()}')
                        found += 1

print(f'Total emojis found: {found}')
sys.exit(1 if found > 0 else 0)
