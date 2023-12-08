def eval(line):
    hand, bid = line.split()
    hand = hand.translate(str.maketrans('TJQKA', face))
    best = max(type(hand.replace('0', r)) for r in hand)
    return best, hand, int(bid)

def type(hand):
    return sorted(map(hand.count, hand), reverse=True)

for face in 'ABCDE', 'A0CDE':
    print(sum(rank * bid for rank, (*_, bid) in
        enumerate(sorted(map(eval, open('data/day7.txt'))), start=1)))
    # print((rank, bid) for rank, (*_, bid) in
    #     enumerate(sorted(map(eval, open('data/day7.txt'))), start=1))

    # with open('res.txt', 'w') as f:
    #     for (rank, (*_, bid)) in enumerate(sorted(map(eval, open('data/day7.txt'))), start=1):
    #         f.write(f"{bid},\n")