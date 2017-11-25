/* 
Task : Dereference a null pointer
Date : 14 March 2017
Version : 1.0
Author : Vigneshwer
 */

// Including the header files
#include <iostream>
using namespace std;

int main()
{
	// Random integer variable
	int random_variable;
	// Pointing to an integer 
	int *rand_point; 

	// Assigning random_varible a value of 10
	random_variable = 10;
	// rand points to the memory address of random_variable
	rand_point = &random_variable;
	cout<< "Pointer memory locaiton is : " << rand_point << endl;
	cout<< "Pointer value is : " << *rand_point << endl;

	//Segmentation fault 

	//declaring a null pointer
	// int *pointer = NULL;
	//dereference a null pointer
	// *pointer = 1;

	// Write to a portion of memory marked as read-only

	// Compiler marks the constant string as read-only
	char *str = "Foo"; 
	//Leads to segfault
	*str = 'b'; 
	return 0;
}
	