print(sum([[[int(group[0] + group[-1]) for group in ["".join([char for char in line if char.isdigit()])]][0]][0] for line in open("input.txt", "r")]))
