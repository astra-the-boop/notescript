from parser import parse

def interpreter(filename):
    data = parse(filename)
    for i in range(len(data)):
        try:
            current = data[i]
            next = data[i + 1]
            if current["type"] == "text":
                if next["type"] == "note" and next["pitch"].startswith("C"):
                    print(current["text"])
        except IndexError:
            pass



interpreter("Untitled score.musicxml")