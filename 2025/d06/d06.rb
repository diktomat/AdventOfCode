#!/usr/bin/env ruby

def part1(input)
	stacks = []
	File.readlines(input).each do |line|
		i = 0
		line.split do |col|
			stacks.push [] unless stacks.length > i
			stacks[i].push col
			i += 1
		end
	end

	stacks.map do |col|
		op = { '+' => :+, '*' => :* }[col.pop]
		col.map(&:to_i).reduce op
	end.sum
end

def part2(input)
	ops = []
	stacks = File.readlines(input).map(&:chars).transpose.map do |col|
		ops.push({ '+' => :+, '*' => :* }[col.pop])
		col.join.to_i
	end

	ops.compact.map do |op|
		res = stacks.take_while { |num| num != 0 }.reduce op
		stacks = stacks.drop_while { |num| num != 0 }.drop 1
		res
	end.sum
end

def test_part1
	exit false unless p(p(part1('sample.txt')) == 4_277_556)
	exit false unless p(p(part1('input.txt')) == 4_364_617_236_318)
end

def test_part2
	exit false unless p(p(part2('sample.txt')) == 3_263_827)
	exit false unless p(p(part2('input.txt')) == 9_077_004_354_241)
end

case ARGV
when ['test']
	test_part1
	test_part2
	puts 'Success!'
when ['sample']
	puts part1 'sample.txt'
	puts part2 'sample.txt'
when ['-p']
	puts part1 'input.txt'
	puts part2 'input.txt'
else
	puts 'Day 6'
	print "\tPart 1 Sample: ", part1('sample.txt'), ', Input: ', part1('input.txt'), "\n"
	print "\tPart 2 Sample: ", part2('sample.txt'), ', Input: ', part2('input.txt'), "\n"
end
