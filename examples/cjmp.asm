# cjmp.asm
# Takes in a value, if the value is 0 output 1, otherwise output 0.

INP
CJMP is_zero
LOAD zero
OUT
HALT

LABEL is_zero
LOAD one
OUT
HALT

LABEL zero
RAW cjmp.zero.data
LABEL one
RAW cjmp.one.data
