#include <stdio.h>
int main () 
{
	int csc201, csc205, sta205;
	int total;
	float average, percentage;
	
	printf("Enter marks for CSC 201: ");
	scanf("%d", &csc201);
	
	printf("Enter marks for CSC 205: ");
	scanf("%d", &csc205);

	
	printf("Enter marks for STA 205: ");
	scanf("%d", &sta205);
	
	total = csc201 + csc205 + sta205;
	average = total / 3.0;
	percentage = (total /300.0) * 100;
	
	printf("\nResults:\n");
	printf("Total marks: %d\n", total);
	printf("Average marks: %.2f\n", average);	
	printf("Percentage: %2f%%\n", percentage);
	
	return 0;
}