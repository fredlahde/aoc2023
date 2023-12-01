import re

input = open("input.txt", 'r').read()

sample = """
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
one3sdnfone1ddjlzhgninethreenine
"""

spelled = {
    "one": '1',
    "two": '2',
    "three": '3',
    "four": '4',
    "five": '5',
    "six": '6',
    "seven": '7',
    "eight": '8',
    "nine": '9'
}


sum = 0
for l in input.split('\n'):
    print(l)
    if l == "":
        continue
    t = [None] * len(l)

    last_found = (None, None)
    for s in spelled.keys():
        found = [m.start() for m in re.finditer(s, l)]
        for ff in found:
            t[ff] = spelled[s]

    for idx, c in enumerate(l):
        if 48 <= ord(c) <= 57:
            t[idx] = c

    print(t, end="")
    tt = []
    for ttt in t:
        if ttt is not None:
            tt.append(ttt)

    if len(tt) > 1:
        temp = int(f"{tt[0]}{tt[len(tt)-1]}")
        print("\t", temp)

        sum += temp
        continue
    if len(tt) > 0:
        temp = int(f"{tt[0]}{tt[0]}")
        print("\t", temp)
        sum += temp


print(sum)
