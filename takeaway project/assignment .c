#include <stdio.h>

int main() {
    int choice;
    printf("Select the type of input:\n");
    printf("1. Character\n2. Integer\n3. Float\n");
    printf("Enter your choice (1/2/3): ");
    scanf("%d", &choice);

    switch (choice) {
        case 1: { // Character case
            char inputChar;
            printf("Enter a character: ");
            scanf(" %c", &inputChar);

            printf("Next four characters are: ");
            for (int i = 1; i <= 4; i++) {
                printf("%c ", inputChar + i);
            }
            printf("\nASCII code of '%c': %d\n", inputChar, inputChar);
            printf("Size of character: %lu bytes\n", sizeof(inputChar));
            break;
        }

        case 2: { // Integer case
            int inputInt;
            printf("Enter an integer: ");
            scanf("%d", &inputInt);

            printf("Next four integers in multiples of 3 are: ");
            for (int i = 1; i <= 4; i++) {
                printf("%d ", inputInt + (i * 3));
            }
            printf("\nSize of integer: %lu bytes\n", sizeof(inputInt));
            break;
        }

        case 3: { // Float case
            float inputFloat;
            printf("Enter a float: ");
            scanf("%f", &inputFloat);

            printf("Next four floats in multiples of 3 are: ");
            for (int i = 1; i <= 4; i++) {
                printf("%.2f ", inputFloat + (i * 3));
            }
            printf("\nSize of float: %lu bytes\n", sizeof(inputFloat));
            break;
        }

        default:
            printf("Invalid choice. Please enter 1, 2, or 3.\n");
            break;
    }

    return 0;
}
