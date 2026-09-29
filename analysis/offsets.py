import re

def parse(path):
    txt = open(path).read().splitlines()
    start = next(i for i, l in enumerate(txt)
                 if 'Link::tick()}`:' in l and 'ManuallyDrop' not in l)
    end = next(i for i, l in enumerate(txt[start + 1:], start + 1)
               if l.startswith('print-type-size type:'))
    block = txt[start:end]
    total = int(re.search(r': (\d+) bytes', block[0]).group(1))
    variants = {}
    cur = None
    for l in block[1:]:
        m = re.match(r'print-type-size\s+variant `(\w+)`: (\d+) bytes', l)
        if m:
            cur = m.group(1)
            variants[cur] = {'size': int(m.group(2)), 'items': []}
            continue
        m = re.match(r'print-type-size\s+(padding|local .*?|upvar .*?|field .*?): (\d+) bytes(.*)', l)
        if m and cur:
            name, sz, rest = m.group(1), int(m.group(2)), m.group(3)
            off = re.search(r'offset: (\d+)', rest)
            variants[cur]['items'].append(
                (name, sz, int(off.group(1)) if off else None))
    return total, variants

def self_offset(v):
    off = 0
    for name, sz, explicit in v['items']:
        if explicit is not None:
            off = explicit
        if name.startswith('upvar'):
            return off
        off += sz
    return None

for path in ['pts-lib.txt', 'pts-bin.txt']:
    total, vs = parse(path)
    print(f"== {path}: tick total = {total}B")
    for name, v in vs.items():
        s = sum(sz for _, sz, _ in v['items'])
        print(f"  {name:10s} size={v['size']:4d} items_sum={s:4d} self_off={self_offset(v)}")
