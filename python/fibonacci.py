def main():
    try: 
        n = int(input("Enter n to calculate F(n) :"))
        if n < 0:
            print("Sorry but we cannot calculate it for negative numbers, Please enter non-negative one")
        else:
            result = fibonacci(n)
            print(f"The {n}-th Fibonacci number is : {result}")
    except ValueError:
        print("Invalid input. Please enter a non-negative integer.")
        
def fibonacci(n):
    if n==0:
        return 0
    elif n==1:
        return 1
    return fibonacci(n-1) + fibonacci(n-2)        
        
if __name__ == "__main__":  
    main()
    