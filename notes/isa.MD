# SeqStack ISA

## Memory and Number Encoding

Memory is 2^15 bytes, byte addressable.

All values are two's complement 16.16 bit fixed point. When an instruction utilizes the stack for adresses, the fractional portion (the bottom 16 bits) are ignored.

## Instruction Format

Instructions are a byte each. The 3 most significant bits determine the instruction type, and how to interpret the rest of the instruction. Generally, the next n bits determine the specific instruction, with the remaining bits determining the adressing mode or other parameters. Operations do not generally support all adressing modes. See the list of all operations below.

## Adressing Modes

Where supported, addressing modes are represented by two bits in the instruction byte.

* 11 - Immediate (Read value from next 2-4 bytes in memory)
* 10 - Index Stack (Read value from next 2 bytes in memory, treat as memory address, use stack top as offset from that address)
* 01 - Index Immediate (Read offset from next 2 bytes in memory, address from top of stack, fractional portion ignored)
* 00 - Stack (Read value from top of stack)

## Instruction List and Format

* 111 - Stack Manipulation
  * Adressing Modes: Immediate, Index Stack, Index Immediate, Stack. Stack treats the top of the stack as a memory address (ignores fractional portion).
    * 111 - Push
    * 000 - Store
  * Adressing Modes: Stack.
    * 110 - Pop
    * 101 - Dup
    * 100 - Rot
    * 011 - MovToRts
    * 010 - MovFromRts
* 110 - Arithmetic
  * Adressing Modes: Stack only.
    * 11 - Add
    * 10 - Sub
    * 01 - Mul
    * 00 - Div
* 101 - Bit Manipulation
  * Adressing Modes: Stack only.
    * 111 - Shl
    * 110 - Shr
    * 101 - Rol
    * 100 - Ror
    * 011 - And
    * 010 - Or
    * 001 - Xor
    * 000 - Not
* 100 - Port Ops
  * Adressing Modes: Stack only. The remaining three bits determine which port the operation corresponds to.
    * 11 - PortPush
    * 10 - PortPop
    * 01 - PortGet
    * 00 - PortClear
* 011 - Control Flow
  * Adressing Modes: Immediate, Index Stack, Index Immediate, Stack. For comparison instructions, the top two values on the stack are compared. If the instruction jumps, the third value on the stack is used (if that adressing mode uses the stack).
    * 111 - JMP
    * 110 - CALL
    * 101 - RET
    * 100 - BEQ
    * 011 - BNEQ
    * 010 - BGT
    * 001 - BLT
* 010 - Interrupt
  * SETI - set interrupt (one for each port??)
* 001 - Misc.
  * 1 - BRK
    * Stop execution.
  * 0 - NOP
    * Do nothing.
