(BEGIN)
    @KBD
    D=M
    @BLACK
    D;JGT

    // clear
    @0
    D=A
    @screenColour
    M=D
    @FILLSCREEN
    0;JEQ

        // black
        (BLACK)
        @0
        D=A-1
        @screenColour
        M=D
        @fillScreen
        0;JEQ

    (FILLSCREEN)
    @SCREEN
    D=A
    @pixel
    M=D
        (FILL)
        @screenColour
        D=M
        @pixel
        A=M
        M=D
        @pixel
        M=M+1
        @SCREEN
        D=A
        @pixel
        D=M-D
        @8192
        D=A-D
        @FILL
        D;JGT
@BEGIN
0;JEQ