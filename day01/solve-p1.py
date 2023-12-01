input = open("input.txt", 'r').read()

sample = """
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"""


sum = 0
for l in input.split('\n'):
    if l == "":
        continue
    t = []
    for c in l:
        if 48 <= ord(c) <= 57:
            t.append(c)

    if len(t) > 1:
        sum += int(f"{t[0]}{t[len(t)-1]}")
        continue
    if len(t) > 0:
        sum += int(f"{t[0]}{t[0]}")


print(sum)
