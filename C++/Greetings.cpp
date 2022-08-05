#include <iostream>
#include <string>
using namespace std;

int main() {

	string name;
	string answer;
	/* Depending on how you're feeling 
	you can change the var from "good" to whatever you want*/
	string response = "good";
	
	cout << "What's your name? " << endl;
	cin>>name;
	cout << name << endl;
	if (name >= answer) {
	    cout << "Hi: " << name << endl;
	}
	else {
	    cout << "Who Is You? " << endl;
	}
	
	cout << "How Are You Today, " << name << "?"<< endl;
	cin >> response;
	cout << response << endl;
	
	if (response == "good") {
	    cout << "That's good, I'm Glad to Hear " << endl;
	}
	
	else {
	    cout << "I'm Sorry to Hear That, " << name << endl;
	}
	return 0;
}
