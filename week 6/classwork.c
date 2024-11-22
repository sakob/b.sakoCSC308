#include <stdio.h>

int main() {
    char foodtype; 
    int quantity;
    int price = 0;
    int totalcost = 0;
    char choice;

    printf("Welcome to Mama Cas restaurant\n");
    printf("Menu\n");
    printf("P = Pounded Yam/Edinkaiko Soup - 3200\n");
    printf("F = Fried Rice & Chicken - 3000\n");
    printf("A = Amala & Ewedu Soup - 2500\n");
    printf("E = Eba & Egusi Soup - 2000\n");
    printf("W = White Rice & Stew - 2500\n");

    do {
        // Input food type
        printf("\nEnter food type (P, F, A, E, W): ");
        scanf(" %c", &foodtype); // Space before %c to handle trailing newline

        // Input quantity
        printf("Enter quantity: ");
        scanf("%d", &quantity);

        // Determine price based on food type
        switch (foodtype) {
            case 'P':
            case 'p':
                price = 3200;
                break;
            case 'F':
            case 'f':
                price = 3000;
                break;
            case 'A':
            case 'a':
                price = 2500;
                break;
            case 'E':
            case 'e':
                price = 2000;
                break;
            case 'W':
            case 'w':
                price = 2500;
                break;
            default:
                printf("Invalid food type selected. Please try again.\n");
                continue; // Skip the rest of the loop for invalid input
        }

        // Calculate cost for the current order and add to total
        int cost = price * quantity;
        totalcost += cost;
        printf("Cost for this order: N%d\n", cost);

        // Ask if the user wants to order more
        printf("Do you want to order more food? (Y/N): ");
        scanf(" %c", &choice); // Space before %c to handle trailing newline

    } while (choice == 'Y' || choice == 'y');

    // Display the final total cost
    printf("\nTotal cost of all orders: N%d\n", totalcost);

    return 0;
}
