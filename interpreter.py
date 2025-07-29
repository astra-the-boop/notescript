from parser import parse

def interpreter(filename):
    data = parse(filename)
    vars = {}
    for i in range(len(data)):
        try:
            previous = data[i - 1]
            current = data[i]
            next = data[i + 1]

            # PRINT

            if current["type"] == "text":
                if next["type"] == "note" and next["pitch"].startswith("C"):
                    if (current["text"].startswith('"') and current["text"].endswith('"')) or (current["text"].startswith("'") and current["text"].endswith("'")):
                        print(current["text"][1:-1])
                    else:
                        try:
                            print(float(current["text"]))
                        except ValueError:
                            if current["text"].strip() in vars.keys():
                                print(vars[current["text"].strip()])
                            else:
                                raise NameError


            if current["type"] == "repeatStart":
                j = 0
                varName = ""
                varDeclare = False
                while data[i + j]["type"] != "repeatEnd":

                    # VARIABLES
                    # DECLARING VARIABLES
                    if data[i + j]["type"] == "note" and data[i + j]["pitch"].startswith("D"):
                        if data[i + j - 1]["type"] == "text":
                            varName = data[i + j - 1]["text"].strip()
                        else:
                            varName = ""
                        varDeclare = True
                    if varDeclare:
                        if data[i + j]["type"] == "note" and data[i + j]["pitch"].startswith("A"):
                            vars.update({varName: data[i + j - 1]["text"]})
                            varDeclare = False
                        elif data[i + j]["type"] == "note" and data[i + j]["pitch"].startswith("B"):
                            if data[i + j - 1]["text"].startswith("B#"):
                                vars.update({varName: int(data[i + j - 1]["text"])})
                            else:
                                vars.update({varName: float(data[i + j - 1]["text"])})
                            varDeclare = False
                        elif data[i + j]["type"] == "note" and data[i + j]["pitch"].startswith("E"):
                            vars.update({varName: bool(data[i + j - 1]["text"])})
                            varDeclare = False
                        else:
                            vars.update({varName: None})


                    j = j + 1


        except IndexError:
            pass






interpreter("Untitled score.musicxml")