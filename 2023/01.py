f = open("data/01.txt")
content = f.readlines()
for i in range(0, len(content)):
    content[i] = content[i].strip()

tally = 0
for i in range(0, len(content)):
    num1, num2 = -1, -1
    for j in str(content[i]):
        if j.isdigit():
            if num1 == -1:
                num1 = int(j)
            num2 = int(j)
    if num1 != -1 and num2 != -1:
        concat = f"{str(num1)}{str(num2)}"
        tally += int(concat)
print(f"The tally for arabic digits is {tally}")

tally = 0
allowed = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"]
for i in range(0, len(content)):
    num1, num1index, num2, num2index = -1, -1, -1, -1
    # DIGIT ANALYSIS
    for j in str(content[i]):
        if j.isdigit():
            if num1 == -1:
                num1 = int(j)
                num1index = int(content[i].index(j))
            num2 = int(j)
            num2index = int(content[i].index(j))
    # WORD ANALYSIS
    for word in allowed:
        for j in range(0, len(content[i])):
            findindex = content[i].find(word, j)
            if (findindex != -1 and (findindex < num1index or num1index == -1)) or num1 == -1:
                num1 = allowed.index(word) + 1
                num1index = findindex
            if (findindex != -1 and findindex > num2index) or num2 == -1:
                num2 = allowed.index(word) + 1
                num2index = findindex
    # TALLYING UP
    if num1 != -1 and num2 != -1:
        concat = f"{str(num1)}{str(num2)}"
        tally += int(concat)
        # print(f"{concat}")  # , "OK" if concat in ["29", "83", "13", "24", "42", "14", "76"] else "!!")
    else:
        print("!!")
print(f"The tally for alphanumeric digits is {tally}")
