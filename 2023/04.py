f = open("data/04.txt")
content = f.readlines()
for line in range(0, len(content)):
    content[line] = content[line].strip()

sum_of_points: int = 0
for line in range(0, len(content)):
    winners = 0
    no_adnotation = content[line].replace(f"Card {line+1}: ", "").strip().replace("  ", " ")
    winning_numbers = no_adnotation[:(no_adnotation.index("|"))].strip().split(" ")
    numbers = no_adnotation[(no_adnotation.index("|") + 1):].strip().split(" ")
    for wnumber in winning_numbers:
        if wnumber in numbers:
            winners += 1
    sum_of_points += 2 ** (winners - 1) if winners else 0
print(sum_of_points)