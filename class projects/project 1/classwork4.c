#include <stdio.h>

int main() {
    char operator;
    double num1, num2, result;

    printf("Welcome to the calculator!\n");
    printf("Select operation:\n");
    printf(" + : Addition\n");
    printf(" - : Subtraction\n");
    printf(" * : Multiplication\n");
    printf(" / : Division\n");
    printf("Enter 'q' to quit\n");

    while (1) {
        printf("\nEnter an operator: ");
        scanf(" %c", &operator);

        if (operator == 'q') {
            printf("Exiting the calculator. Goodbye!\n");
            break;
        }

        printf("Enter first number: ");
        scanf("%lf", &num1);
        printf("Enter second number: ");
        scanf("%lf", &num2);

        switch (operator) {
            case '+':
                result = num1 + num2;
                printf("%.2lf + %.2lf = %.2lf\n", num1, num2, result);
                break;
            case '-':
                result = num1 - num2;
                printf("%.2lf - %.2lf = %.2lf\n", num1, num2, result);
                break;
            case '*':
                result = num1 * num2;
                printf("%.2lf * %.2lf = %.2lf\n", num1, num2, result);
                break;
            case '/':
                if (num2 == 0) {
                    printf("Error! Division by zero.\n");
                } else {
                    result = num1 / num2;
                    printf("%.2lf / %.2lf = %.2lf\n", num1, num2, result);
                }
                break;
            default:
                printf("Invalid operator! Please try again.\n");
        }
    }

    return 0;
}
