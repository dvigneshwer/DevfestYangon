/* 
Task : Demonstrating buffer overflow
Date : 14 March 2017
Version : 1.0
Author : Vigneshwer
 */
// Including the header files
#include <iostream>
#include <cstring>
using namespace std;

int main(){
	// buffer overflow
	char rand_str[5];
	// write past the end of buffer
	strcpy(rand_str ,"Follow me @dvigneshwer in Twitter");
	// read past end of the buffer
	cout << "6th character " << rand_str[5] << endl;
	cout << "7th character " << rand_str[6] << endl;
	return 0;
}