.model tiny

.stack 100h

.data
        buffer db 20 DUP(30h)
        invalid db " is not a natural number!", 0dh, 0ah, 0dh, 0ah, "$"

        proof db "The conjecture is proved for all natural numbers smaller or equals to 10000!", 0dh, 0ah, "$"

        intro1 db 0dh, 0ah, "This program is a simple test for the following conjecture:", 0dh, 0ah, "$"
        intro2 db 0dh, 0ah, "Let S: N -> N be the sum of the digits of a positive integer.", 0dh, 0ah, "$"
        intro3 db "For all A and B in N, S(A + B) = S(A) + S(B) - 9k, where k is an integer.", 0dh, 0ah, "$"
        intro4 db 0dh, 0ah, "What value would you like to test the conjecture for? $"

.code
        org 100h

start:
        mov ax, @data
        mov ds, ax

        ; Print the intro messages to the screen
        mov ah, 9h
        mov dx, offset intro1
        int 21h
        mov dx, offset intro2
        int 21h
        mov dx, offset intro3
        int 21h
        mov dx, offset intro4
        int 21h

        ; Retrieve user input
        call read_uint
        mov bx, ax
          
        mov ax, 0
        call iter

read_uint:
        ; Load the user input into the buffer
        mov ax, cs
        mov ds, ax
        mov dx, 0
        mov buffer[0], 20 ; Configure the buffer length
        mov dl, offset buffer
        mov ah, 0ah
        int 21h

        mov ax, 0 ; Initialize the register were the result will be stored
        mov cx, 2 ; Initialize a counter
        mov si, offset buffer
        add si, 3

read_loop:
        mov bx, 0 ; Clear bx
        mov bl, [si] ; Read a character into bx
        inc cx ; Increment the counter
        inc si ; Increment the source index

        ; Check if it's a numeric character
        cmp bl, 30h
        jb short invalid_input
        cmp bl, 39h
        ja short invalid_input

        ; Convert the character code into a decimal digit
        sub bl, 30h

        ; ax = ax * 10 + bx
        push cx
        mov cx, 10
        mul cx
        pop cx
        add ax, bx

        ; Jump if the counter is still smaller than the buffer length
        cmp cl, buffer[0]
        jb short read_loop
        cmp cl, buffer[1]
        jb short read_loop                
        ret

invalid_input:
        mov ah, 2

        ; Print '\''
        mov dx, 27h
        int 21h

        mov si, offset buffer
        mov cx, 0

invalid_input_loop:
        mov dl, [si]
        int 21h
        inc cx
        inc si

        cmp cl, 6
        jbe short invalid_input_loop

invalid_input_end:
        ; Print '\''
        mov dx, 27h
        int 21h

        ; Print the rest of the message
        mov ah, 9h
        mov dx, offset invalid
        int 21h

        call quit
        ret

; Iterate ax from 0 to 10000
iter:
        call test_num

        inc ax
        cmp ax, 10000
        jbe short iter
      
        call proved

; Iterate bx from 0 to ax
test_num:
        mov bx, 0
test_loop:
        push bx
        push ax

        call test_pair
        cmp dx, 0
        jne short disproved

        pop ax
        pop bx
        inc bx

        cmp bx, ax
        jbe short test_loop
        ret        

test_pair:
        ; Calculate the 1st + 2nd into cx
        mov cx, ax
        add cx, bx
        
        push cx ; Push 1st + 2nd to the stack
        push bx ; Push 2nd to the stack

        call sum_digits ; Calculate S(1st) into bx

        pop ax ; Store the value of 2nd in ax
        push bx ; Push S(1st) to the stack
        call sum_digits ; Calculate S(2nd) into bx
        
        pop ax ; Store S(1st) in ax 
        add ax, bx ; Store S(1st) + S(2nd) in ax
        pop bx ; Store 1st + 2nd in bx
        push ax ; Push S(1st) + S(2nd) to the stack

        mov ax, bx ; Store 1st + 2nd in ax    
        call sum_digits ; Calculate S(1st + 2nd) into bx

        pop ax ; Store S(1st) + S(2nd) in ax
        sub bx, ax ; Calculate S(1st + 2nd) - (S(1st) + S(2nd)) into bx

 
        mov ax, bx ; Store S(1st + 2nd) - (S(1st) + S(2nd)) in ax
        mov cx, 9 ; Set the devident to 9
        mov dx, 0 ; Clear the register where the rest will be stored
        div cx

        ret

sum_digits:
        mov cx, 10 ; Store the devident in cx
        mov bx, 0 ; Clear the register where the result will be stored        
sum_loop:
        mov dx, 0 ; Clear the rest of the division

        div cx ; Divide ax by cx
        add bx, dx ; Add the rest of the division ax/cx to bx

        ; Loop until ax equals 0
        cmp ax, 0
        ja short sum_loop
        ret

proved:
        mov ah, 9h
        mov dx, offset proof
        int 21h

        call quit
        ret

disproved:
        mov ah, 9h
        mov dx, offset proof
        int 21h

        call quit
        ret        
quit:
        int 20h
end start
