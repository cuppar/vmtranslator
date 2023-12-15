# Jack Compiler Backend

vmtranslator is the compiler backend of language Jack, the compiler frontend is [`jackc`](https://github.com/cuppar/jackc).

- `vmtranslator` transfer a `XXX.vm` vm program to a `XXX.asm` file.
- The `XXX.asm` file is the `Hack` assembly code file, it can be translate to `Hack` machine language by [`assembler`](https://github.com/cuppar/assembler).
- `Hack` is a very simple assembly language, it has only two type instruction, `A`(Address) instruction and `C`(Compute) instruction.

## Example

```bash
$ vmtranslator Main.vm
```

### VM code

Main.vm

```
function Main.fibonacci 0
	push argument 0
	push constant 2
	lt                     
	if-goto N_LT_2        
	goto N_GE_2
label N_LT_2               // if n < 2 returns n
	push argument 0        
	return
label N_GE_2               // if n >= 2 returns fib(n - 2) + fib(n - 1)
	push argument 0
	push constant 2
	sub
	call Main.fibonacci 1  // computes fib(n - 2)
	push argument 0
	push constant 1
	sub
	call Main.fibonacci 1  // computes fib(n - 1)
	add                    // returns fib(n - 1) + fib(n - 2)
	return
```

### Assembly code

Main.asm

```
@256
D=A
@SP
M=D
// start call ========================
// push return_address
@Bootstrap$ret.0
D=A
@SP
A=M
M=D
@SP
M=M+1
// push LCL
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
// push ARG
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THIS
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THAT
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
// ARG=SP-5-n_args
@SP
D=M
@5
D=D-A
@0
D=D-A
@ARG
M=D
// LCL=SP
@SP
D=M
@LCL
M=D
// goto function_name
@Sys.init
0;JMP
(Bootstrap$ret.0)
// end call ========================
(Main.fibonacci)
// start ======== push ARG 0
// D=ARG+0
@ARG
D=M
@0
D=D+A
A=D
D=M
// stack[SP]=D
@SP
A=M
M=D
// SP++
@SP
M=M+1
// end ======== push ARG 0

// start ======== push constant 2
// D=2
@2
D=A
// stack[SP]=D
@SP
A=M
M=D
// SP++
@SP
M=M+1
// end ======== push constant 2

// start ======= lt
// start ======== pop temp 0
// SP--
@SP
M=M-1
// R13=addr(temp+0)
@5
D=A
@R13
M=D
// D=stack[SP]
@SP
A=M
D=M
// *R13=D
@R13
A=M
M=D
// end ======== pop temp 0

// start ======== pop temp 1
// SP--
@SP
M=M-1
// R13=addr(temp+1)
@6
D=A
@R13
M=D
// D=stack[SP]
@SP
A=M
D=M
// *R13=D
@R13
A=M
M=D
// end ======== pop temp 1

// start ======= temp0 := temp1 < temp0
@5
A=A+1
D=M
@5
M=D-M
D=M
M=0
@HIT_Main.vm.3
D;JLT
@CONTINUE_Main.vm.3
0;JMP
(HIT_Main.vm.3)
@5
M=-1
(CONTINUE_Main.vm.3)
// end ======= temp0 := temp1 < temp0

// start ======== push temp 0
// D=temp+0
@5
D=M
// stack[SP]=D
@SP
A=M
M=D
// SP++
@SP
M=M+1
// end ======== push temp 0

// end ======= lt

@SP
AM=M-1
D=M
@Main.fibonacci$N_LT_2
D;JNE
@Main.fibonacci$N_GE_2
0;JMP
(Main.fibonacci$N_LT_2)
// start ======== push ARG 0
// D=ARG+0
@ARG
D=M
@0
D=D+A
A=D
D=M
// stack[SP]=D
@SP
A=M
M=D
// SP++
@SP
M=M+1
// end ======== push ARG 0

// start return ========================
// frame(R13)=LCL
@LCL
D=M
@R13
M=D
// return_address(R14)=*(frame-5)
@R13
D=M
@5
A=D-A
D=M
@R14
M=D
// *ARG=pop()
@SP
AM=M-1
D=M
@ARG
A=M
M=D
// SP=ARG+1
@ARG
D=M+1
@SP
M=D
// THAT=*(frame-1)
@R13
D=M
@1
A=D-A
D=M
@THAT
M=D
// THIS=*(frame-2)
@R13
D=M
@2
A=D-A
D=M
@THIS
M=D
// ARG=*(frame-3)
@R13
D=M
@3
A=D-A
D=M
@ARG
M=D
// LCL=*(frame-4)
@R13
D=M
@4
A=D-A
D=M
@LCL
M=D
// goto return_address
@R14
A=M
0;JMP
// end return ========================
(Main.fibonacci$N_GE_2)
// start ======== push ARG 0
// D=ARG+0
@ARG
D=M
@0
D=D+A
A=D
D=M
// stack[SP]=D
@SP
A=M
M=D
// SP++
@SP
M=M+1
// end ======== push ARG 0

// start ======== push constant 2
// D=2
@2
D=A
// stack[SP]=D
@SP
A=M
M=D
// SP++
@SP
M=M+1
// end ======== push constant 2

// start ======= sub
// start ======== pop temp 0
// SP--
@SP
M=M-1
// R13=addr(temp+0)
@5
D=A
@R13
M=D
// D=stack[SP]
@SP
A=M
D=M
// *R13=D
@R13
A=M
M=D
// end ======== pop temp 0

// start ======== pop temp 1
// SP--
@SP
M=M-1
// R13=addr(temp+1)
@6
D=A
@R13
M=D
// D=stack[SP]
@SP
A=M
D=M
// *R13=D
@R13
A=M
M=D
// end ======== pop temp 1

// start ======= temp0 = temp1 - temp0
@5
A=A+1
D=M
@5
M=D-M
// end ======= temp0 = temp1 - temp0

// start ======== push temp 0
// D=temp+0
@5
D=M
// stack[SP]=D
@SP
A=M
M=D
// SP++
@SP
M=M+1
// end ======== push temp 0

// end ======= sub

// start call ========================
// push return_address
@Main.fibonacci$ret.0
D=A
@SP
A=M
M=D
@SP
M=M+1
// push LCL
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
// push ARG
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THIS
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THAT
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
// ARG=SP-5-n_args
@SP
D=M
@5
D=D-A
@1
D=D-A
@ARG
M=D
// LCL=SP
@SP
D=M
@LCL
M=D
// goto function_name
@Main.fibonacci
0;JMP
(Main.fibonacci$ret.0)
// end call ========================
// start ======== push ARG 0
// D=ARG+0
@ARG
D=M
@0
D=D+A
A=D
D=M
// stack[SP]=D
@SP
A=M
M=D
// SP++
@SP
M=M+1
// end ======== push ARG 0

// start ======== push constant 1
// D=1
@1
D=A
// stack[SP]=D
@SP
A=M
M=D
// SP++
@SP
M=M+1
// end ======== push constant 1

// start ======= sub
// start ======== pop temp 0
// SP--
@SP
M=M-1
// R13=addr(temp+0)
@5
D=A
@R13
M=D
// D=stack[SP]
@SP
A=M
D=M
// *R13=D
@R13
A=M
M=D
// end ======== pop temp 0

// start ======== pop temp 1
// SP--
@SP
M=M-1
// R13=addr(temp+1)
@6
D=A
@R13
M=D
// D=stack[SP]
@SP
A=M
D=M
// *R13=D
@R13
A=M
M=D
// end ======== pop temp 1

// start ======= temp0 = temp1 - temp0
@5
A=A+1
D=M
@5
M=D-M
// end ======= temp0 = temp1 - temp0

// start ======== push temp 0
// D=temp+0
@5
D=M
// stack[SP]=D
@SP
A=M
M=D
// SP++
@SP
M=M+1
// end ======== push temp 0

// end ======= sub

// start call ========================
// push return_address
@Main.fibonacci$ret.1
D=A
@SP
A=M
M=D
@SP
M=M+1
// push LCL
@LCL
D=M
@SP
A=M
M=D
@SP
M=M+1
// push ARG
@ARG
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THIS
@THIS
D=M
@SP
A=M
M=D
@SP
M=M+1
// push THAT
@THAT
D=M
@SP
A=M
M=D
@SP
M=M+1
// ARG=SP-5-n_args
@SP
D=M
@5
D=D-A
@1
D=D-A
@ARG
M=D
// LCL=SP
@SP
D=M
@LCL
M=D
// goto function_name
@Main.fibonacci
0;JMP
(Main.fibonacci$ret.1)
// end call ========================
// start ======= add
// start ======== pop temp 2
// SP--
@SP
M=M-1
// R13=addr(temp+2)
@7
D=A
@R13
M=D
// D=stack[SP]
@SP
A=M
D=M
// *R13=D
@R13
A=M
M=D
// end ======== pop temp 2

// start ======== pop temp 3
// SP--
@SP
M=M-1
// R13=addr(temp+3)
@8
D=A
@R13
M=D
// D=stack[SP]
@SP
A=M
D=M
// *R13=D
@R13
A=M
M=D
// end ======== pop temp 3

// start ======= temp0 = temp1 + temp0
@7
A=A+1
D=M
@7
M=D+M
// end ======= temp0 = temp1 + temp0

// start ======== push temp 2
// D=temp+2
@7
D=M
// stack[SP]=D
@SP
A=M
M=D
// SP++
@SP
M=M+1
// end ======== push temp 2

// end ======= add

// start return ========================
// frame(R13)=LCL
@LCL
D=M
@R13
M=D
// return_address(R14)=*(frame-5)
@R13
D=M
@5
A=D-A
D=M
@R14
M=D
// *ARG=pop()
@SP
AM=M-1
D=M
@ARG
A=M
M=D
// SP=ARG+1
@ARG
D=M+1
@SP
M=D
// THAT=*(frame-1)
@R13
D=M
@1
A=D-A
D=M
@THAT
M=D
// THIS=*(frame-2)
@R13
D=M
@2
A=D-A
D=M
@THIS
M=D
// ARG=*(frame-3)
@R13
D=M
@3
A=D-A
D=M
@ARG
M=D
// LCL=*(frame-4)
@R13
D=M
@4
A=D-A
D=M
@LCL
M=D
// goto return_address
@R14
A=M
0;JMP
// end return ========================
// end the program
(END)
@END
0;JMP
```

