#include <stdio.h>

int main() {
	int a;
	float b;
	double c;
	char name[6];
	
	int *p2a; 
	p2a = &a;
	a = 10;
	
	printf("Value: %i \n Address: %p \n", *p2a, p2a);
	a = 90;
	printf("Value: %i \n Address: %p \n", *p2a, p2a);
	*p2a = 5;
	printf("Value: %i \n Address: %p \n", *p2a, p2a);
	
	// puts("Pointers to all the variables declared: ");
	// printf("a: %p: \n", &a);
	// printf("b: %p: \n", &b);
	// printf("c: %p: \n", &c);
	// printf("name[0]: %p: ", name);
	
	// for (int i=0; i < 6; i++) {
	//	printf("name[%d]: %p \n", i, &name);
	//}
	
	// scanf("%s", name);
	// printf("%s", name);
	
	// int *p2a;
	// p2a = &a;
	
	// printf("%p", p2a);
}