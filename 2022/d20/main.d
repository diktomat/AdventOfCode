import std.stdio : writeln;
import std.file : slurp;
import std.algorithm.mutation : swap;
import std.typecons : tuple, Tuple;

long decrypt(long[] input, int iterations = 1, int key = 1) {
	int[] order = new int[input.length];
	Tuple!(int, long)[] position = new Tuple!(int, long)[input.length];
	int len = cast(int) input.length;
	for (int i = 0; i < input.length; i++) {
		order[i] = i;
		position[i] = tuple(i, input[i] * key);
	}

	for (int iter = 0; iter < iterations; iter++) {
		for (int i = 0; i < order.length; i++) {
			int curidx = order[i] % len;
			if (curidx < 0) {
				curidx = len + curidx;
			}

			int cur = position[curidx][1] % (len - 1);
			if (cur < 0) {
				cur = len + cur - 1;
			}

			int start = curidx;
			int ende = (order[i] + cur) % len;
			if (ende < 0) {
				ende = len + ende;
			}

			for (int j = start; j != ende; j = (j + 1) % len) {
				int pos1 = j % len;
				int pos2 = (j + 1) % len;
				if (pos2 < 0) {
					pos2 = len + pos2;
				}

				swap(position[pos1], position[pos2]);
				order[position[pos1][0]]--;
				order[position[pos2][0]]++;
			}
		}
	}

	int zidx = 0;
	for (int i = 0; i < len; i++) {
		if (position[i][1] == 0) {
			zidx = i;
			break;
		}
	}

	return position[(zidx + 1000) % len][1]
		+ position[(zidx + 2000) % len][1]
		+ position[(zidx + 3000) % len][1];
}

void main() {
	long[] input = slurp!(long)("input.txt", "%s");
	writeln(decrypt(input));
	writeln(decrypt(input, 10, 811_589_153));
}
