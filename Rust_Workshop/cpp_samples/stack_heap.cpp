/* 
Task : Playing with stack and heap
Date : 14 March 2017
Version : 1.0
Author : Vigneshwer
 */

// Including the header files
#include <iostream>
#include <stdio.h>
#include <vector>
using namespace std;

// function to multiply by 2 
double multipleTwo (double input) {
    double twice = input * 2.0;
    return twice;
}

// main function
int main (int argc, char *argv[])
{
    // allocation to stack
    int age_val = 17;
    double salary_val = 234.67;
    double List_val[3] = {3.2, 1.3, 4.9};
    printf("Your salary is %.3f\n", multipleTwo(salary_val));

    // locates value in stack
    int stackArray[3] = { 1, 2, 3 };
    cout << stackArray[0] << "\n";

    // default locates value in heap as it is dynmaic
   	vector<int> vec;
   	int i;

   	// push 5 values into the vector
   	for(i = 0; i < 5; i++){
		vec.push_back(i);
	}
   	// printing the values of the vector
   	cout << vec[0] << "\n";
    return 0;
}