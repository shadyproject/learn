#include <stdio.h>

int main(int argc, const char *argv[])
{
	char c = 'A';
	printf("Character as char: %c\n", c);
	printf("Character as int: %d\n", c);
	printf("Character as float: %f\n", c);
	printf("Character as double: %lf\n", c);
	//this crashes the app, likely because it expects a pointer
	//printf("Character as string: %s\n", c);

	int i = 42;
	printf("Integer as int: %d\n", i);
	printf("Integer as char: %c\n", i);
	printf("Integer as float: %f\n", i);
	printf("Integer as double: %lf\n", i);
	//this crashes the app, likely because it expects a pointer
	//printf("integer as string: %s\n", i);

	float f = 3.14;
	printf("Float as int: %d\n", f);
	printf("Float as char: %c\n", f);
	printf("Float as float: %f\n", f);
	printf("Float as double: %lf\n", f);
	//this crashes the app, likely because it expects a pointer
	//printf("Float as string: %s\n", f);


	return 0;
}
