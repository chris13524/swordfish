# add.asm
# Take a value in, adds the value of `add.bin` to it, outputs the value.

LABEL top
INP
ADD data
OUT
JMP top

LABEL data
RAW add.x.data
