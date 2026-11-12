import os
import re
import sys

link_re = re.compile(r'\[([^\]]+)\]\(([^)]+)\)')
img_re = re.compile(r'<img[^>]+src=[\'"]([^\'"]+)[\'"]')
broken = 0

for root, dirs, files in os.walk('.'):
    if any(p in root for p in ['.git', 'target', 'dist', 're-workspace']):
        continue
    for f in files:
        if f.endswith('.md'):
            path = os.path.join(root, f)
            with open(path, 'r', encoding='utf-8') as fp:
                content = fp.read()
            for text, target in link_re.findall(content):
                if target.startswith(('http://', 'https://', 'file://', '#', 'mailto:')):
                    continue
                clean_target = target.split('#')[0]
                if not clean_target:
                    continue
                target_path = os.path.normpath(os.path.join(root, clean_target))
                if not os.path.exists(target_path):
                    print(f'Broken link in {path}: [{text}]({target}) -> {target_path}')
                    broken += 1
            for src in img_re.findall(content):
                if src.startswith(('http://', 'https://', 'file://', 'data:')):
                    continue
                src_path = os.path.normpath(os.path.join(root, src))
                if not os.path.exists(src_path):
                    print(f'Broken image in {path}: src="{src}" -> {src_path}')
                    broken += 1

print(f'Total broken links or images: {broken}')
sys.exit(1 if broken > 0 else 0)
