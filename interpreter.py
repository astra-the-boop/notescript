from parser import parse

vars = {}

# def compare()

def prnt(data, i):
    previous = data[i - 1]
    current = data[i]
    next = data[i + 1]
    if current["type"] == "note" and current["pitch"].startswith("C"):
        if previous["type"] == "text":
            try:
                print(float(previous["text"]))
            except ValueError:
                if previous["text"].strip() in vars.keys():
                    print(vars[previous["text"].strip()])
                else:
                    # print(previous["text"])
                    # print(vars)
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
        return None

def math(current, prevVal, prevType, nextVal, nextType):
    if prevType[0:1] == "A" or nextType[0:1] == "A":
        if (prevType[0:1] == nextType[0:1] or prevType[0:1] == "D" or nextType[0:1] == "D") and (current.startswith("G#") and not current.startswith("G##")):
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
        elif current.startswith("G--"):
            x = float(prevVal) / float(nextVal)
            if prevType.startswith("B##") or prevType.startswith("B--"):
                return bool(x)
            elif prevType.startswith("B#"):
                return int(x)
            else:
                return float(x)
        elif current.startswith("G-"):
            x = float(prevVal) - float(nextVal)
            if prevType.startswith("B##") or prevType.startswith("B--"):
                return bool(x)
            elif prevType.startswith("B#"):
                return int(x)
            else:
                return float(x)
        return None



def interpreter(filename):
    data = parse(filename)
    repeatUse = []
    runMode = "run" #run: regular run; eval: for ... evaluating shit; def: defining shit

    for i in range(len(data)):
        try:
            previous = data[i - 1]
            current = data[i]
            next = data[i + 1]

            # PRINT
            if runMode == "run":
                prnt(data, i)



            if current["type"] == "repeatStart":
                j = 0
                varName = ""

                while data[i + j]["type"] != "repeatEnd":
                    # VARIABLES
                    # DECLARING VARIABLES
                    try:
                        if data[i + j]["type"] == "note" and data[i + j]["pitch"].startswith("D") and repeatUse[-1] != "var" and runMode == "run":
                            if data[i + j - 1]["type"] == "text":
                                varName = data[i + j - 1]["text"].strip()
                            else:
                                print(data[i+j-1]["type"])
                                varName = ""
                            runMode = "def"
                            repeatUse.append("var")
                        if repeatUse[-1] == "var" and runMode == "def":
                            if data[i + j]["type"] == "note":
                                if data[i + j]["pitch"][0:1] in ["A", "B"]:
                                    if data[i+j]["pitch"][0:3] in ["B##", "B--"]:
                                        vars.update({varName: data[i+j]["pitch"].startswith("B##")})
                                    else:
                                        vars.update({varName:typecast(data[i+j], data[i+j-1])})
                                        if data[i+j+1]["type"] == "note" and data[i+j+1]["pitch"].startswith("G"):
                                            if data[i+j+2]["type"] == "text" and data[i+j+3]["type"] == "note" and data[i+j+3]["pitch"].startswith("D"):
                                                vars.update({varName:math(data[i+j+1], data[i+j-1]["text"], data[i+j]["pitch"], vars[data[i+2]["text"]], data[i+j+3]["pitch"])})
                                            else:
                                                vars.update({varName:math(data[i+j+1]["pitch"], data[i+j-1]["text"], data[i+j]["pitch"], data[i+j+2]["text"],data[i+j+3]["pitch"])})
                                        break
                                elif data[i + j]["pitch"].startswith("D#"):
                                    vars.update({varName: vars[data[i+j-1]["text"]]})
                                    if data[i+j+1]["type"] == "note" and data[i+j+1]["pitch"].startswith("G"):
                                        if data[i+j+2]["type"] == "text" and data[i+j+3]["type"] == "note" and data[i+j+3]["pitch"].startswith("D"):
                                            vars.update({varName:math(data[i+j+1], vars[data[i+j-1]["text"]], data[i+j]["pitch"], vars[data[i+2]["text"]], data[i+j+3]["pitch"])})
                                        else:
                                            vars.update({varName:math(data[i+j+1]["pitch"], vars[data[i+j-1]["text"]], data[i+j]["pitch"], data[i+j+2]["text"],data[i+j+3]["pitch"])})
                                    break
                            elif runMode == "def":
                                vars.update({varName: None})

                    except IndexError:
                        if data[i + j]["type"] == "note" and data[i + j]["pitch"].startswith("D") and repeatUse[-1] != "var" and runMode == "run":
                            if data[i + j - 1]["type"] == "text":
                                varName = data[i + j - 1]["text"].strip()
                            else:
                                print(data[i + j - 1]["type"])
                                varName = ""
                            runMode = "def"
                            repeatUse.append("var")
                    j += 1

                    # IF STATEMENTS
                    if data[i + j]["type"] == "note" and data[i + j]["pitch"].startswith("E") and runMode != "eval":
                        runMode = "eval"
                        repeatUse.append("ifeval")





            if current["type"] == "repeatEnd":
                runMode = "run"
                repeatUse.pop()



        except IndexError:
            pass






interpreter("Untitled score.musicxml")