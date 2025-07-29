from music21 import converter, note, meter, tempo, stream, expressions, bar

def parse(filename):
    score = converter.parse(filename)
    flatScore = score.flatten()
    output = []

    for part in score.parts:
        for measure in part.getElementsByClass("Measure"):
            if measure.leftBarline and isinstance(measure.leftBarline, bar.Repeat):
                if measure.leftBarline.direction == "start":
                    output.append({"type": "repeatStart"})

                if measure.rightBarline and isinstance(measure.rightBarline, bar.Repeat):
                    if measure.rightBarline.direction == "end":
                        output.append({"type": "repeatEnd"})

                for element in measure:
                    if isinstance(element, note.Note):
                        pitch = element.nameWithOctave
                        duration = element.duration.quarterLength
                        output.append({"type":"note", "pitch":pitch, "duration":duration})
                    elif isinstance(element, note.Rest):
                        duration = element.quarterLength
                        output.append({"type":"rest", "duration":duration})
                    elif isinstance(element, expressions.TextExpression):
                        output.append({"type":"text", "text":element.content.strip()})

    return output

print(parse("Untitled score.musicxml"))