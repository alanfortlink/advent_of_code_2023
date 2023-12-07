import sys

values = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}


def get(line, start, end, inc):
    for i in range(start, end, inc):
        for k, v in values.items():
            if line[i:].startswith(k):
                return v
        if line[i].isdigit():
            return int(line[i])


def q2(L):
    D = get(L, 0, len(L), 1)
    U = get(L, len(L)-1, -1, -1)
    return 10 * D + U


print(sum(map(lambda x: q2(x), open(sys.argv[1], 'r').readlines())))
