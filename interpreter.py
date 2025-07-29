from parser import parse



def math(current, prevVal, prevType, nextVal, nextType):
    if prevType[0:1] == "A" or nextType[0:1] == "A":
        if prevType[0:1] == nextType[0:1]:
            return prevVal + nextVal
        else:
            raise TypeError
    else:
        if current.startswith("G##"):
            x = float(prevVal + nextVal)
            if prevType.startswith("B##") or prevType.startswith("B--"):
                return bool(x)
            elif prevType.startswith("B#"):
                return int(x)
            else:
                return float(x)
        return None


def interpreter(filename):
    data = parse(filename)
    vars = {}
    for i in range(len(data)):
        try:
            previous = data[i - 1]
            current = data[i]
            next = data[i + 1]

            # PRINT
            if current["type"] == "note" and current["pitch"].startswith("C"):
                if previous["type"] == "text":
                    try:
                        print(float(current["text"]))
                    except ValueError:
                        if current["text"].strip() in vars.keys():
                            print(vars[current["text"].strip()])
                        else:
                            raise NameError
                else:
                    if next["type"] == "text":
                        if data[i + 2]["type"] == "note" and data[i + 2]["pitch"][0:1] in ["A", "B"]:
                            j=1
                            while data[i + j]["type"] == "rest":
                                j = j + 1

                                if data[i + j]["type"] == "note":
                                    break
                            if data[i+j]["pitch"].startswith("G"):
                                k = 1
                                while data[i+j+k]["type"] == "rest":
                                    if data[i+j+k]["type"] == "note":
                                        break
                                if data[i+j+k]["pitch"][0:1] in ["A", "B"]:
                                    math(data[i + j]["pitch"],
                                        next["text"],
                                        data[i + 2]["pitch"],
                                        data[i+j+k-1]["text"],
                                        data[i+j+k]["pitch"])


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
                            if data[i + j]["pitch"].startswith("B#"):
                                vars.update({varName: int(float(data[i + j - 1]["text"]))})
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