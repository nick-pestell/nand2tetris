(begin)
    @pressed
    M=0
    @KBD
    D=!M
    D=D+1
    @pressed
    M=D
    @32767
    D=!A
    @pressed
    D=D&M // 1000000000000000 if pressed, 0000000000000000 if not pressed
    D=D-1 // 0111111111111111 if pressed, 1111111111111111 if not pressed
    M=!D //  1000000000000000 if pressed, 0000000000000000 if not pressed
    @SCREEN
    D=A
    @pixel
    M=D
        (startScreen)
        // this bit fills the screen
        @pressed
        D=M
        @pixel
        A=M
        M=D // sets the value of the pixel to the value of @pressed
        @pixel
        M=M+1 // moves the pixel on one
        // this bit does working out if we're finished with the screen
        @SCREEN
        D=A
        @pixel
        D=M-D
        @8192
        D=A-D
        @startScreen
        D;JGT
@begin
0;JEQ