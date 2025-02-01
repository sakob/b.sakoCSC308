#include <stdio.h>

int main() {
	int a;
	float b;
	double c;
	char name[6];
	
	puts("Pointers to all the variables declared: ");
	printf("a: %p: \n", &a);
	printf("b: %p: \n", &b);
	printf("c: %p: \n", &c);
	printf("name[0]: %p: ", name);
	
	// scanf("%s", name);
	// printf("%s", name);
	
	// int *p2a;
	// p2a = &a;
	
	// printf("%p", p2a);
}