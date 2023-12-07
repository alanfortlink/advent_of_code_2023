lines = open('input.txt', 'r').readlines()
t = int(lines[0].split(":")[1].replace(" ", ""))
d = int(lines[1].split(":")[1].replace(" ", ""))
print(len(list(filter(lambda i: (t - i) * i > d, range(0, t)))))
