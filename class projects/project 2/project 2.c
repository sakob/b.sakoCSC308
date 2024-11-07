#include <stdio.h>

int main() {
    int age;
    char experience;

    // Get input for experience and age
    printf("Do you have experience? (y/n): ");
    scanf(" %c", &experience);
    printf("Enter your age: ");
    scanf("%d", &age);

    // Determine salary based on experience and age
    if (experience == 'y' || experience == 'Y') {  // Experienced person
        if (age >= 40) {
            printf("Your salary is N560,000\n");
        } else if (age >= 30 && age < 40) {
            printf("Your salary is N480,000\n");
        } else if (age < 28) {
            printf("Your salary is N300,000\n");
        } else {
            printf("Age does not match any specified salary range.\n");
        }
    } else if (experience == 'n' || experience == 'N') {  // Inexperienced person
        printf("Your salary is N100,000\n");
    } else {
        printf("Invalid input for experience.\n");
    }

    return 0;
}
