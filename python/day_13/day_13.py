import json as js

def compare(l1, l2):
    for x, y in zip(l1, l2):
        match (isinstance(x, int), isinstance(y, int)):
            case (True, True) if x == y: continue
            case (True, True): return x < y
            case (False, True): return compare(x, [y])
            case (True, False): return compare([x], y)
            case _: return compare(x, y)
    return len(l1) <= len(l2)

with open('test_input.txt', 'r') as f:
    pairs = map(lambda p: (js.loads(p[0]), js.loads(p[1])), map(lambda p: p.strip().split('\n'), f.read().split('\n\n')))
    print(sum(map(lambda e: e[0] + 1 if e[1] else 0, enumerate(map(lambda p: compare(p[0], p[1]), pairs)))))
