from music21 import converter, note, meter, tempo, stream

def parse(filename):
    score = converter.parse(filename)
    flatScore = score.flatten()
    output = []

    for element in flatScore:
        if isinstance(element, tempo.MetronomeMark):
            output.append({
                "type": "tempo",
                "value": int(element.number),
            })
        elif isinstance(element, meter.TimeSignature):
            output.append({
                "type": "time",
                "numerator": element.numerator,
                "denominator": element.denominator,
            })
        elif isinstance(element, note.Note):
            pitch = element.nameWithOctave
            duration = element.quarterLength
            output.append({"type": "note", "pitch": pitch, "duration": duration})

        elif isinstance(element, note.Rest):
            duration = element.quarterLength
            output.append({
                "type": "rest",
                "duration": duration,
            })


    return output

print(parse("Untitled score.musicxml"))