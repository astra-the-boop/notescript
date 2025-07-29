from music21 import converter, note, expressions, bar

def parse(filename):
    score = converter.parse(filename)
    output = []

    for part in score.parts:
        i = 1
        for measure in part.getElementsByClass("Measure"):
            print(i)
            i = i + 1
            barlineLeft = measure.leftBarline
            barlineRight = measure.rightBarline

            if isinstance(barlineLeft, bar.Repeat) and barlineLeft.direction == "start":
                output.append({"type": "repeatStart"})
                print("a")

            if isinstance(barlineRight, bar.Repeat) and barlineRight.direction == "end":
                output.append({"type": "repeatEnd"})
                print("b")

            for element in measure:
                if isinstance(element, note.Note):
                    pitch = element.nameWithOctave
                    duration = element.quarterLength
                    output.append({"type": "note", "pitch": pitch, "duration": duration})
                elif isinstance(element, note.Rest):
                    output.append({"type": "rest", "duration": element.quarterLength})
                elif isinstance(element, expressions.TextExpression):
                    output.append({"type":"text", "text": element.content.strip()})

    return output


print(parse("Untitled score.musicxml"))