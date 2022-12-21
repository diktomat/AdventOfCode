program main;

uses
	SysUtils;

type
	PMonkey = ^TMonkey;
	TMonkey = record
		name: AnsiString;
		hasChilds: Boolean;
		m1: PMonkey;
		m2: PMonkey;
		m1name: AnsiString;
		m2name: AnsiString;
		value: Int64;
		op: Char;
		parent: PMonkey;
	end;

const
	FILENAME = 'input.txt';

var
	fd: TextFile;
	line: AnsiString;
	monkeys: PMonkey;
	monkeyarr: array of PMonkey;
	i: Integer;

function parse(line: AnsiString): TMonkey;
var
	monkey: TMonkey;
	i, state: Integer;
begin
	with monkey do begin
		name := '    ';
		m1name := '    ';
		m2name := '    ';
		hasChilds := false;
		value := 0;
		op := ' '
	end;

	state := 0;
	for i := 0 to length(line) do begin
		if line[i] = ' ' then begin
			state += 1;
			continue;
		end;

		with monkey do begin
			case state of
				0: name[i] := line[i];
				1: if ord(line[i]) > 57 then m1name[i-6] := line[i] 
					else value := value * 10 + ord(line[i]) - 48;
				2: op := line[i];
				3: m2name[i-13] := line[i]
			end;
		end;
	end;

	if monkey.m1name <> '    ' then
		monkey.hasChilds := true;

	parse := monkey;
end;

function findMonkey(name: AnsiString): PMonkey;
var i: Integer;
begin
	for i := 0 to length(monkeyarr) - 1 do begin
		if monkeyarr[i]^.name <> name then
			continue;
		exit(monkeyarr[i]);
	end;
end;

procedure buildTree(node: PMonkey);
begin
	if not node^.hasChilds then exit;

	node^.m1 := findMonkey(node^.m1name);
	node^.m1^.parent := node;
	node^.m2 := findMonkey(node^.m2name);
	node^.m2^.parent := node;

	buildTree(node^.m1);
	buildTree(node^.m2);
end;

function solveTree(node: PMonkey): Int64;
begin
	if not node^.hasChilds then
		exit(node^.value);

	case node^.op of
		'*': solveTree := solveTree(node^.m1) * solveTree(node^.m2);
		'+': solveTree := solveTree(node^.m1) + solveTree(node^.m2);
		'-': solveTree := solveTree(node^.m1) - solveTree(node^.m2);
		'/': solveTree := round(solveTree(node^.m1) / solveTree(node^.m2));
	end;
end;

function contains(arr: array of PMonkey; elem: PMonkey): Boolean;
var i: Integer;
begin
	for i := 0 to length(arr) - 1 do begin
		if arr[i] = elem then exit(true)
	end;
	contains := false;
end;

function solveReversed: Int64;
var
	path: array of PMonkey;
	cur, humnpart, otherpart: PMonkey;
	i: Integer;
	goal: Int64;
	subgoal: Int64;
begin
	setLength(path, 1000);
	i := 0;
	cur := findMonkey('humn');

	while cur^.name <> 'root' do begin
		path[i] := cur;
		cur := cur^.parent;
		i += 1;
	end;
	setLength(path, i);

	if contains(path, monkeys^.m1) then
		goal := solveTree(monkeys^.m2)
	else
		goal := solveTree(monkeys^.m1);

	for i := length(path)-1 downto 1 do begin
		if contains(path, path[i]^.m1) then begin
			humnpart := path[i]^.m1;
			otherpart := path[i]^.m2;
		end else begin
			humnpart := path[i]^.m2;
			otherpart := path[i]^.m1;
		end;
		subgoal := solveTree(otherpart);

		case path[i]^.op of
			'+': goal -= subgoal;
			'*': goal := round(goal / subgoal);
			'-': if humnpart = path[i]^.m1 then goal += subgoal
				else goal := subgoal - goal;
			'/': if humnpart = path[i]^.m1 then goal *= subgoal
				else goal := round(subgoal / goal);
		end;
	end;

	solveReversed := goal;
end;

begin
	assign(fd, FILENAME);
	reset(fd);
	i := 0;
	setLength(monkeyarr, 2000);
	while not eof(fd) do begin
		readln(fd, line);
		monkeyarr[i] := new(PMonkey);
		monkeyarr[i]^ := parse(line);
		i += 1;
	end;
	close(fd);
	setLength(monkeyarr, i);

	monkeys := findMonkey('root');
	buildTree(monkeys);

	writeln(solveTree(monkeys));
	writeln(solveReversed);
end.

