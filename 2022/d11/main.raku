#!/usr/bin/env raku
use v6;

grammar Monkey {
	token TOP       { <monkey><items><operation><test><iftrue><iffalse> }
	token monkey    { 'Monkey ' <num> ':'\n }
	token items     { '  Starting items: ' [<num> ', '?]+\n }
	token operation { '  Operation: new = old ' <op>' '[<num> | 'old']\n }
	token test      { '  Test: divisible by ' <num>\n }
	token iftrue    { '    If true: throw to monkey ' <num>\n }
	token iffalse   { '    If false: throw to monkey ' <num> }
	token num       { \d+ }
	token op        { ['*' | '+'] }
}

my @monkeys;
my $input = slurp "input.txt";
for $input.split("\n\n") -> $inmonkey {
	my %match = Monkey.subparse($inmonkey).hash;
	my %monkey;
	%monkey<items> = [%match<items><num>.clone, %match<items><num>.clone];
	%monkey<op> = %match<operation><op>;
	%monkey<opnum> = %match<operation><num>;
	%monkey<test> = %match<test><num>;
	%monkey<iftrue> = %match<iftrue><num>;
	%monkey<iffalse> = %match<iffalse><num>;
	%monkey<business> = [0, 0];
	@monkeys.push(%monkey)
}

my $mod = 1;
$mod *= $_<test> for @monkeys;

for ^2 -> $round {
	for ^($round == 0 ?? 20 !! 10000) {
		for @monkeys -> %monkey {
			for %monkey<items>[$round][] -> $item {
				%monkey<business>[$round]++;

				my $new = %monkey<opnum> // $item;
				$new = %monkey<op> eq '*' ?? $item * $new !! $item + $new;
				$new = $round == 0 ?? $new div 3 !! $new % $mod;

				@monkeys[%monkey{$new %% %monkey<test> ?? 'iftrue' !! 'iffalse'}]<items>[$round].push($new);
			}
			%monkey<items>[$round] = []
		}
	}
}

for ^2 -> $round {
	my @businesses;

	for @monkeys -> %monkey {
		@businesses.push(%monkey<business>[$round]);
	}

	@businesses = @businesses.sort;
	say @businesses[*-1] * @businesses[*-2];
}
