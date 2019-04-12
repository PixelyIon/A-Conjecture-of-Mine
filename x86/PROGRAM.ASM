.model tiny

.data
        intro db "Hey", 0dh, 0ah, "$"
        proof db "The conjecture is proved for all natural numbers smaller or equals to", 0dh, 0ah, "10000!", 0dh, 0ah, "$"

.code
org 100h

start:
        mov dx, offset intro
        mov ah, 9h
        int 21h
          
        mov ax, 0
        call iter

iter:
        call test_num

        add ax, 1
        cmp ax, 10000
        jle short iter
      
        call proved

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
        add bx, 1

        cmp bx, ax
        jle short test_loop
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
        jg short sum_loop
        ret

proved:
        ; Print the message to the screen
        mov dx, offset proof
        mov ah, 9h
        int 21h

        call quit
        ret
disproved:
        mov dx, offset proof
        mov ah, 9h
        int 21h

        call quit
        ret        
quit:
        mov ah, 4ch
        int 21h
end start
