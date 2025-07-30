from parser import parse

def typecast(type, val):
    if type["pitch"].startswith("A"):
        return str(val["text"])
    elif type["pitch"][0:3] in ["B##", "B--"]:
        return bool(val["text"])
    elif type["pitch"].startswith("B#"):
        return int(float(val["text"]))
    elif type["pitch"].startswith("B"):
        return float(val["text"])
    else:
        print(type["pitch"])
        return None

def math(current, prevVal, prevType, nextVal, nextType):
    if prevType[0:1] == "A" or nextType[0:1] == "A":
        if prevType[0:1] == nextType[0:1]:
            return prevVal + nextVal
        else:
            raise TypeError
    else:
        if current.startswith("G##"):
            x = float(prevVal) * float(nextVal)
            if prevType.startswith("B##") or prevType.startswith("B--"):
                return bool(x)
            elif prevType.startswith("B#"):
                return int(x)
            else:
                return float(x)
        elif current.startswith("G#"):
            x = float(prevVal) + float(nextVal)
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
                        print(float(previous["text"]))
                    except ValueError:
                        if previous["text"].strip() in vars.keys():
                            print(vars[previous["text"].strip()])
                        else:
                            raise NameError
                else:
                    if (next["type"] == "text"
                            and data[i + 2]["type"] == "note"
                            and data[i + 2]["pitch"][0:1] in ["A", "B"]):
                        if (data[i + 3]["type"] == "note"
                            and data[i + 3]["pitch"].startswith("G")
                            and data[i + 4]["type"] == "text"
                            and data[i + 5]["type"] == "note"
                            and data[i + 5]["pitch"][0:1] in ["A", "B"]):
                            print(math(
                                data[i + 3]["pitch"],
                                next["text"],
                                data[i + 2]["pitch"],
                                data[i + 4]["text"],
                                data[i + 5]["pitch"]
                            ))
                        else:
                            print(next["text"])



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
                        if data[i + j]["type"] == "note" and data[i + j]["pitch"][0:1] in ["A", "B"]:
                            vars.update({varName:typecast(data[i+j], data[i+j-1])})
                            varDeclare = False
                        else:
                            vars.update({varName: None})


                    j += 1





        except IndexError:
            pass
    print(vars)






interpreter("Untitled score.musicxml")