#!/usr/bin/env raku
use v6.d;

grammar Monkey {
	rule TOP       { <monkey><items><operation><test><iftrue><iffalse> }
	rule monkey    { 'Monkey' <num>':' }
	rule items     { 'Starting items:' <num>+ % ', ' }
	rule operation { 'Operation: new = old' <op> [<num> | 'old'] }
	rule test      { 'Test: divisible by' <num> }
	rule iftrue    { 'If true: throw to monkey' <num> }
	rule iffalse   { 'If false: throw to monkey' <num> }
	token num      { \d+ }
	token op       { ['*' | '+'] }
}

my @monkeys;
my $input = slurp "input.txt";
for $input.split("\n\n") -> $inmonkey {
	my %match = Monkey.subparse($inmonkey).hash;
	my %monkey =
		items    => [%match<items><num>.clone, %match<items><num>.clone],
		op       => %match<operation><op>,
		opnum    => %match<operation><num>,
		test     => %match<test><num>,
		iftrue   => %match<iftrue><num>,
		iffalse  => %match<iffalse><num>,
		business => [0, 0];
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
