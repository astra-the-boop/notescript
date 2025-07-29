from parser import parse

def interpreter(filename):
    data = parse(filename)
    for i in range(len(data)):
        try:
            previous = data[i - 1]
            current = data[i]
            next = data[i + 1]

            # PRINT

            if current["type"] == "text":
                if next["type"] == "note" and next["pitch"].startswith("C"):
                    print(current["text"])

            # VARIABLES

        # DECLARING VARIABLES
            if current["type"] == "repeatStart":
                j = 0

                while data[i + j]["type"] != "repeatEnd":
                    if data[i + j]["type"] == "note" and data[i + j]["pitch"].startswith("D"):



        except IndexError:
            pass



interpreter("Untitled score.musicxml")