#include <stdio.h>
int main ()
{
	int account_balance = 10000;
	int credit = 50000;
	int debit = 20000;
	
	// int new_balance = account_balance + credit;
	// int another_balance = account_balance - debit;
	
	account_balance += credit;
	
	int age = 5;
	printf("%i", age % 2);
	return 0;
}