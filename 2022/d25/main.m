#import <Foundation/Foundation.h>

NSString *add(NSString *left, NSString *right) {
	NSDictionary *sums = [NSDictionary
		dictionaryWithObjectsAndKeys:@"00", @"2=", @"01", @"2-", @"02", @"20", @"1=", @"21", @"1-", @"22",
		                             @"0-", @"1=", @"00", @"1-", @"01", @"10", @"02", @"11", @"1=", @"12",
		                             @"0=", @"0=", @"0-", @"0-", @"00", @"00", @"01", @"01", @"02", @"02",
		                             @"-2", @"-=", @"0=", @"--", @"0-", @"-0", @"00", @"-1", @"01", @"-2",
		                             @"-1", @"==", @"-2", @"=-", @"0=", @"=0", @"0-", @"=1", @"00", @"=2", nil];

	int llen = left.length;
	int rlen = right.length;
	int len = llen > rlen ? llen : rlen;
	NSString *sum = @"";
	UTF8Char carry = '0';

	for (NSUInteger i = 1; i <= len; i++) {
		NSUInteger li = llen - i;
		NSUInteger ri = rlen - i;
		UTF8Char l = li < llen ? [left characterAtIndex:li] : '0';
		UTF8Char r = ri < rlen ? [right characterAtIndex:ri] : '0';

		NSString *sumlr = sums[[NSString stringWithFormat:@"%c%c", l, r]];
		NSString *sumcs = sums[[NSString stringWithFormat:@"%c%c", carry, [sumlr characterAtIndex:1]]];
		sum = [NSString stringWithFormat:@"%c%@", [sumcs characterAtIndex:1], sum];

		carry = [sums[[NSString stringWithFormat:@"%c%c", [sumlr characterAtIndex:0],
			                                              [sumcs characterAtIndex:0]]]
			characterAtIndex:1];
	}

	if (carry != '0') {
		sum = [NSString stringWithFormat:@"%c%@", carry, sum];
	}

	return sum;
}

int main() {
	NSArray *input = [[NSString stringWithContentsOfFile:@"input.txt"
	                                            encoding:NSUTF8StringEncoding
	                                               error:nil]
		componentsSeparatedByCharactersInSet:[NSCharacterSet newlineCharacterSet]];

	NSString *sum = input[0];
	for (NSUInteger i = 1; i < input.count; i++) {
		sum = add(sum, input[i]);
	}

	printf("%s\n", [sum UTF8String]);
	return 0;
}
