#include <stdio.h>

int main() {
	int a;
	
	int *p2a;
	p2a = &a;
	
	printf("%p", p2a);
}