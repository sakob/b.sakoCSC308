#include <stdio.h>

int main() {
	int a;
	float b;
	double c;
	char name[] = "Blessing";
	int *p2a; 
	p2a = &a;
	a = 10;
	
	// Pointer Arithmetics
	for (int i=0; i < 8; i++) {
		printf("%c \n", name[i]);
	}
	puts("");
	
	char *p2name = name;
	for (int i =0; i < 8; i++) {
		printf("%c \n", *p2name);
		p2name++;
	}
	
}