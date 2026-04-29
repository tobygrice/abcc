WaCC pg 3:
A C compiler typically processes source code in four stages:
1. The lexer breaks up the source code into a list of tokens. If a program is like a book, tokens are like individual words.
2. The parser converts the list of tokens into an abstract syntax tree (AST), which represents the program in a form we can easily traverse and analyse.
3. The assembly generation pass converts the AST into assembly. At this stage, we still represent the assembly instructions in a data structure that the compiler can understand, not as text.
4. The code emission pass writes the assembly code to a file so the assembler and linker can turn it into an executable.

![[Pasted image 20260429152311.png]]

### Assembly
```c
int main(void) {
	return 2;
}
```

`gcc -S -O -fno-asynchronous-unwind-tables -fcf-protection=none return_2.c`

```nasm
	.globl main
main:
	movl $2, %eax
	ret
```
- The `.globl main` directive tells the assembler that `main` is a global symbol.
	- allows other object files to refer to `main`
	- this is recorded in the symbol table
- On the second line, we use main as a label for the code that follows it. 
	- Labels consist of a string or number followed by a colon. 
	- A label marks the location that a symbol refers to.
	- This particular label defines `main` as the address of the `movl` instruction on the following line.
- `eax` is the return value register. There is also `rax`, the 64-bit equivalent
- The `l` suffix in `movl` indicates that the operands to this instruction are longwords, or 32-bit integers (in x64 assembly, unlike most modern implementations of C, long means 32 bits). A `movq` instruction operates on quadwords, or 64-bit integers.

What calls main? Where does it return to?
The linker adds a bit of wrapper code called `crt0` to handle setup before main runs and teardown after it exits. (The `crt` stands for C Runtime.) This wrapper code does the following:
1. Makes a function call to main. This is why main needs to be globally visible; if it isn’t, `crt0` can’t call it.
2. Retrieves the return value from main.
3. Invokes the exit system call, passing it the return value from main. Then, exit handles whatever work needs to happen inside the operating system to terminate the process and turn the return value into an exit code

Therefore, the compiler can treat main like a normal function - the linker handles setup/teardown.