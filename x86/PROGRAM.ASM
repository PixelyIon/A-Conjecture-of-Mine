.model tiny

.code
org 100h

start:
        mov ax, 32 ; 1st
        mov bx, 16 ; 2nd

        call test_pair
        
        ; Quit the program
        mov ah, 4ch
        int 21h

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

        cmp dx, 0
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
        jg short sum_loop
        ret

end start