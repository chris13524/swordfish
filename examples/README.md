# Swordfish examples

## Building assembler and VM

cd into both the asm and vm directories and run `cargo build --release`.

## echo.asm

Build the program: `../asm/target/release/asm <echo.asm >echo.out`

Run the program: `../vm/target/release/vm echo.out`

Pass 8 bytes and then they will get echoed back to you. (e.g. type in 7 characters and then hit enter)

Ok, let's try using files on-disk:

Write the 64-bit number 0x01 to `echo.in.data`: `echo -n "\x00\x00\x00\x00\x00\x00\x00\x01" > echo.in.data`

`../vm/target/release/vm echo.out <echo.in.data >echo.out.data`

```
hexedit echo.out.data
00 00 00 00  00 00 00 01
```

## add.asm

Write the number we want to add to the first number to add.x.data`: `echo -n "\x00\x00\x00\x00\x00\x00\x00\xD0" > add.x.data`

Build the program: `../asm/target/release/asm <add.asm >add.out`

Write the input number 0x0A to `add.in.data`: `echo -n "\x00\x00\x00\x00\x00\x00\x00\x0A" > add.in.data`

Run the program: `../vm/target/release/vm add.out <add.in.data >add.out.data`

```
hexedit add.out.data
00 00 00 00  00 00 00 DA
```

## cjmp.asm

Write our one's and zero's to files:

```
echo -n "\x00\x00\x00\x00\x00\x00\x00\x00" > cjmp.zero.data
echo -n "\x00\x00\x00\x00\x00\x00\x00\x01" > cjmp.one.data
```

Build the program: `../asm/target/release/asm <cjmp.asm >cjmp.out`

```
echo -n "\x00\x00\x00\x00\x00\x00\x00\x00" | ../vm/target/release/vm cjmp.out | od -t x1
echo -n "\x00\x00\x00\x00\xFF\x00\x00\x00" | ../vm/target/release/vm cjmp.out | od -t x1
```
