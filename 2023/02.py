import string

f = open("data/02.txt")
content = f.readlines()
for line in range(0, len(content)):
    content[line] = content[line].strip()

tally, powertally = 0, 0
for line in range(0, len(content)):
    maxred, maxgreen, maxblue, gameid = 0, 0, 0, line+1
    game = content[line].replace(f"Game {line+1}: ", "")
    draws = game.replace(" ", "").split(";")
    for draw in draws:
        cubes = draw.split(",")
        for cube in cubes:
            cubenum = int(cube.strip(string.ascii_letters))
            cubecolor = cube.strip(string.digits)
            if cubecolor == "red" and cubenum > maxred:
                maxred = cubenum
            elif cubecolor == "green" and cubenum > maxgreen:
                maxgreen = cubenum
            elif cubecolor == "blue" and cubenum > maxblue:
                maxblue = cubenum
    if maxred <= 12 and maxgreen <= 13 and maxblue <= 14:
        tally += gameid
    powertally += maxred*maxgreen*maxblue
print(f"The sum of game IDs with a max of 12 red cubes, 13 green and 14 blue ones is {tally}")
print(f"The sum of the powers of the sets of cubes used in all games is equal to {powertally}")
