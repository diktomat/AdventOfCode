// I have no experience with C whatsoever beyond knowing the syntax and how to
// use man pages and Google. I guess this is brittle and insecure and should not
// be used for anything.
// But it got me the correct results, so I'm happy with it!

#include <stdio.h>
#include <stdlib.h>

int main() {
	FILE *fp;
	char *line = NULL;
	size_t linecap = 0;

	long part1 = 0;
	long part2 = 0;
	fp = fopen("input.txt", "r");
	if (fp == NULL)
		exit(EXIT_FAILURE);

	while (getline(&line, &linecap, fp) > 0) {
		long nums[4];
		char *ptr;
		for (int i = 0; i < 4; i++) {
			nums[i] = strtol(line, &ptr, 10);
			line = &ptr[1]; // skip `,` and `-`
		}

		if ((nums[0] >= nums[2] && nums[1] <= nums[3]) ||
		    (nums[2] >= nums[0] && nums[3] <= nums[1]))
			part1++;

		if ((unsigned)(nums[0] - nums[2]) <= (nums[3] - nums[2]) ||
		    (unsigned)(nums[1] - nums[2]) <= (nums[3] - nums[2]) ||
		    (unsigned)(nums[2] - nums[0]) <= (nums[1] - nums[0]) ||
		    (unsigned)(nums[3] - nums[0]) <= (nums[1] - nums[0]))
			part2++;
	}
	printf("%ld\n", part1);
	printf("%ld\n", part2);

	fclose(fp);
	return EXIT_SUCCESS;
}
