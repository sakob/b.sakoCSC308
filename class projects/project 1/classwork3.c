#include <stdio.h>

int main() {
    char input;

    // Prompt user for input
    printf("Enter a character between A and J or a and j: ");
    scanf(" %c", &input);

    // Check if input is within range
    if ((input >= 'A' && input <= 'J') || (input >= 'a' && input <= 'j')) {
        printf("The next 6 characters are: ");
        for (int i = 1; i <= 6; i++) {
            printf("%c ", input + i);
        }
        printf("\n");
    } else {
        printf("Invalid input. Please enter a character between A and J or a and j.\n");
    }

    return 0;
}
