 gcc return_2.s -o return_2
 	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 15, 0	sdk_version 15, 5
	.globl	_main                           ; -- Begin function main //assembler directive => a statement that provides direction for the assembler.main is  a symbol, a name for a memory address  it tells the assembler that  that main is a global symbol and can be referenced from other files.
	.p2align	
_main:                                  ; @main  //main is used as a label to mark the beginning of the function; Label consists of an identifier followed by a colon.
	.cfi_startproc                               //the label defines main as  the address of the mov instruction in this line 
; %bb.0:
	mov	w0, #2                          ; =0x2 //mov is an instruction that moves the value 2 into register w0
	ret                                      //ret is an instruction that returns from the function, using the value in w0 as the return value
	.cfi_endproc
                                        ; -- End function
.subsections_via_symbols


//assembler records facts in a  section of the object file called the symbol table. 
//The symbol table contains a list of all the symbols in the program, along with their addresses and other information. When the linker combines multiple object files into a single executable, it uses the symbol table to resolve references to symbols that are defined in other files.

//who is the caller of main? The operating system. When the program is loaded into memory, the OS sets up the initial state of the program, including the stack and registers, and then jumps to the address of main to start executing the program.