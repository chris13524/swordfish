## Swordfish Spec

Swordfish instructions are a fixed 64 bits wide. The first 32 bits is for the opcode and the last 32 bits is for an optional parameter.

Swordfish has a single accumulator register. It is 64 bits, unsigned, and defaults to 0.

Address space is limited to 32-bits.


## Instructions

 - **HALT**: stops execution
 - **INP**: reads one 64-bit number from the serial port into the accumulator
 - **OUT**: writes the value of the accumulator to the serial port
 - **LOAD \<address>**: loads the value at \<address> into the accumulator
 - **SAVE \<address>**: saves the value of the accumulator to \<address>
 - **ADD \<address>**: adds the value at \<address> to the accumulator
 - **SUB \<address>**: subtractss the value at \<address> to the accumulator
 - **MUL \<address>**: multiplies the value at \<address> to the accumulator
 - **DIV \<address>**: divides the value at \<address> to the accumulator
 - **JMP \<address>**: jumps to \<address>
 - **IJMP \<address>**: jumps to the value at \<address>
 - **CJMP \<address>**: jumps to \<address> if the accumulator is 0

## Assembler Instructions

 - **LABEL \<label>**: creates the label \<label>
 - **INCLUDE \<file>**: includes the assembly file \<file>
 - **RAW \<file>**: includes the file \<file> directly in the output

