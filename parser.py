from music21 import converter, note, expressions, bar
import xml.etree.ElementTree as et
import re

def stripNs(tag):
    return re.sub(r'\{.*}','', tag)

def parse(filename):
    score = converter.parse(filename)
    output = []

    tree = et.parse(filename)
    root = tree.getroot()

    def findMeasures():
        for part in root.findall('.//'):
            if stripNs(part.tag) == 'part':
                for measure in part:
                    if stripNs(measure.tag) == 'measure':
                        yield measure
                break

    rawMeasures = list(findMeasures())

    for idx, part in enumerate(score.parts):
        if idx > 0:
            break
        for i, measure in enumerate(part.getElementsByClass("Measure")):
            if i >= len(rawMeasures):
                break
            barlineLeft = measure.leftBarline
            barlineRight = measure.rightBarline
            rawMeasure = rawMeasures[i]



            if isinstance(barlineLeft, bar.Repeat) and barlineLeft.direction == "start":
                output.append({"type": "repeatStart"})

            for barline in rawMeasure.findall(".//"):
                if stripNs(barline.tag) == 'barline':
                    for child in barline:
                        if stripNs(child.tag) == 'ending':
                            endingType = child.attrib.get("type")
                            number = child.attrib.get("number")
                            if endingType == 'start':
                                output.append({"type":"voltaStart", "number":number})

            for element in measure:
                if isinstance(element, note.Note):
                    pitch = element.nameWithOctave
                    duration = element.quarterLength
                    output.append({"type": "note", "pitch": pitch, "duration": duration})
                elif isinstance(element, note.Rest):
                    output.append({"type": "rest", "duration": element.quarterLength})
                elif isinstance(element, expressions.TextExpression):
                    output.append({"type":"text", "text": element.content.strip()})


            for barline in rawMeasure.findall(".//"):
                if stripNs(barline.tag) == 'barline':
                    for child in barline:
                        if stripNs(child.tag) == 'ending':
                            endingType = child.attrib.get("type")
                            number = child.attrib.get("number")
                            if endingType in ["stop", "discontinue"]:
                                output.append({"type": "voltaEnd", "number": number})

            if isinstance(barlineRight, bar.Repeat) and barlineRight.direction == "end":
                output.append({"type": "repeatEnd"})



    return output


print(parse("Untitled score.musicxml"))