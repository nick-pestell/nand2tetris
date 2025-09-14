(begin)
    @KBD
    D=M
    @black
    D;JGT

    // clear
    @0
    D=A
    @screenColour
    M=D
    @fillScreen
    0;JEQ

    // black
    (black)
    @0
    D=A-1
    @screenColour
    M=D
    @fillScreen
    0;JEQ

    (fillScreen)
    @SCREEN
    D=A
    @pixel
    M=D
        (fill)
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
        @fill
        D;JGT
@begin
0;JEQ