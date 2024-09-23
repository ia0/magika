#include <stdio.h>
#include <string.h>

extern int magika_all_in_one(const char *ptr, int len, char *out, int *cap);

int main(int argc, char **argv) {
	if (argc < 2) return -1;
	char out[32] = {0};
	int cap = 31;
	int ret = magika_all_in_one(argv[1], strlen(argv[1]), out, &cap);
	if (ret < 0) return ret;
	printf("%s\n", out);
	return 0;
}
