#include <stdio.h>  

int fibonacci(int n) {  
    if (n <= 1)  
        return n;  
    return fibonacci(n - 1) + fibonacci(n - 2);  
}  

int main() {  
    int n;  
    printf("Enter a positive integer: ");  
    scanf("%d", &n);  
    
    if (n < 0) {  
        printf("Please enter a non-negative integer.\n");  
    } else {  
        printf("Fibonacci of %d is %d\n", n, fibonacci(n));  
    }  
    
    return 0;  
}