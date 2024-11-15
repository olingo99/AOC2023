# Problem 3 - tree collisions

# Part 1
with open('test.txt', 'r') as fd:
    lines = [line.rstrip() for line in fd]

NumberOfChars = len(lines[0])
Position = 0
Speed = 3 # Three right, one down per move - the down part is done by the for loop

NumberOfTrees = 0
for line in lines:
    if line[Position%NumberOfChars] == "#":
        NumberOfTrees = NumberOfTrees + 1
    Position = Position + Speed

print("Part 1: The number of tree collisions is: " + str(NumberOfTrees))

# Part 2
Speed = [[1,1], [3,1], [5,1], [7,1], [1,2]]
Multiplied = 1
for velocity in Speed:
    Position = 0
    NumberOfTrees = 0
    lines = lines[0::velocity[1]] # Select the amount of lines needed.
    for line in lines:
        if line[Position % NumberOfChars] == "#":
            NumberOfTrees = NumberOfTrees + 1
        Position = Position + velocity[0]
    Multiplied = Multiplied * NumberOfTrees
    print("Part 2: With a speed of " + str(velocity) + ", you encounter " + str(NumberOfTrees) + " trees.")

print("Part 2: Multiplied together, the amount of trees hit results in: " + str(Multiplied))
