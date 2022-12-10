#!/usr/bin/env ruby

stacks1 = [[], [], [], [], [], [], [], [], []]
stacks2 = [[], [], [], [], [], [], [], [], []]

File.foreach("input.txt") do |line|
	if line.lstrip.start_with? "["
		for i in (1..33).step(4) do
			stacks1[(i-1)/4].prepend(line[i]) unless line[i] == " "
			stacks2[(i-1)/4].prepend(line[i]) unless line[i] == " "
		end
	elsif line.start_with? "move"
		ins = line.scan(/\d+/).map(&:to_i)

		ins[0].times do
			stacks1[ins[2]-1].push(stacks1[ins[1]-1].pop)
		end
		stacks2[ins[2]-1].push(*stacks2[ins[1]-1].pop(ins[0]))
	# else next
	end
end

stacks1.each { |stack| print stack[-1] }; puts
stacks2.each { |stack| print stack[-1] }; puts
