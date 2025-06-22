from collections import Counter
import sys


def main():
    filename = sys.argv[1]
    print(filename)
    with open(filename) as f:
        lines = f.readlines()

    print(len(lines))

    counter = Counter()
    for line in lines:
        line = line.strip()
        parts = line.split(" ")
        op = parts[0]
        key = parts[1]

        if op == "P":
            counter[key] += 1
        elif op == "U":
            counter[key] += 1
        elif op == "S":
            counter[key] += 1
        else:
            print(op)

    print(counter)
    print(counter.total())


if __name__ == "__main__":
    main()
