from music21 import converter, note, expressions, bar

def parse(filename):
    score = converter.parse(filename)
    output = []

    for idx, part in enumerate(score.parts):
        if idx > 0:
            break
        for measure in part.getElementsByClass("Measure"):
            barlineLeft = measure.leftBarline
            barlineRight = measure.rightBarline

            if isinstance(barlineLeft, bar.Repeat) and barlineLeft.direction == "start":
                output.append({"type": "repeatStart"})

            for element in measure:
                if isinstance(element, note.Note):
                    pitch = element.nameWithOctave
                    duration = element.quarterLength
                    output.append({"type": "note", "pitch": pitch, "duration": duration})
                elif isinstance(element, note.Rest):
                    output.append({"type": "rest", "duration": element.quarterLength})
                elif isinstance(element, expressions.TextExpression):
                    output.append({"type":"text", "text": element.content.strip()})

            if isinstance(barlineRight, bar.Repeat) and barlineRight.direction == "end":
                output.append({"type": "repeatEnd"})



    return output


print(parse("Untitled score.musicxml"))