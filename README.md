# notescript

this is an esolang that uses music scores in the musicxml (uncompressed) format to do the thingy

## How to use:
download repo and then run this in the directory
```uv run cli.py "<filename>.musicxml"```

## Documentation:
### Types
All of the following except for booleans require staff text attached for their values
- Strings: A
- Floats: B
- Integers: B#
- Booleans: Bbb for false; B## for true (note booleans do not require staff text)
- Variables: D (unless if it's calling another variable within the defining of a variable then it's D#)

### Comments
Anything not on the topmost staff will be considered a comment

### Print
The note for print is C
If you're printing the value of a variable, add a staff text to the note with the variable's name
If you're printing anything else, add another note with what you're printing's type and attach a staff text to that with its value (if it's a math formula, see Math)

### Math
Currently only addition is supported
- Put two values before and after a ~~Gbb, Gb,~~ G#~~, or G##~~ to start
- G# adds the values on the two sides

### Declaring variables
- To start a variable declaration and definition, add an open repeat to the start of a measure
- Then add a D note with a staff note attached with the variable's name
- Then add another note with what you want the variable's value to be. Variable names can also be the result of math equations
- End off the definition with a close repeat at the end of the measure
- If you want to reference another variable within a variable, use D# with the variable's name

> If you want to try it out, run the interpreter using the included `demo.musicxml` file. If you want to see what's happening, open the file in a program such as MuseScore and it should include comments on the bottom staff.