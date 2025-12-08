# notescript

this is an esolang that uses music scores in the musicxml (uncompressed) format to do the thingy

## How to use:

install notescript using cargo

```bash
cargo install --git https://github.com/astra-the-boop/notescript.git --branch rust-rewrite
```
after which; you'll get a command something like

```bash
warning: be sure to add `/home/astra/.cargo/bin` to your PATH to be able to run the installed binaries
```

the `/home/astra/.cargo/bin` part can be different for your device; here we'll use <address> as a placeholder

then
```bash
export PATH="$PATH:<address>"
```

after which, you can run the interpreter anywhere!
just do
```bash
notescript-rust <filename>.musicxml
```

## Documentation:
### Types
All of the following except for booleans require staff text attached for their values
- Strings: A
- Floats: B
- ~~Integers: B#~~ currently doesn't work
- Booleans: Bbb for false; B## for true (note booleans do not require staff text)
- Variables: D (unless if it's calling another variable within the defining of a variable then it's D#)

### Comments
Anything not on the topmost staff will be considered a comment

### Print
The note for print is C
If you're printing the value of a variable, add a staff text to the note with the variable's name
If you're printing anything else, add another note with what you're printing's type and attach a staff text to that with its value (if it's a math formula, see Math)

### Math
Currently does not work

### Declaring variables
- To start a variable declaration and definition, add an open repeat to the start of a measure
- Then add a D note with a staff note attached with the variable's name
- Then add another note with what you want the variable's value to be. Variable names can also be the result of math equations
- End off the definition with a close repeat at the end of the measure
- If you want to reference another variable within a variable, use D# with the variable's name

> If you want to try it out, run the interpreter using the included `demo.musicxml` file. If you want to see what's happening, open the file in a program such as MuseScore and it should include comments on the bottom staff.
>

<img src="Screenshot%202025-12-09%20at%2003.17.53.png">