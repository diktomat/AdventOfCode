record Valve, flow : Int32, paths : Array(String)

valves = {} of String => Valve
File.read_lines("input.txt").each { |line|
	name = line[6..7]
	flow = line.match(/(\d+)/).not_nil![0].to_i
	paths = line.scan(/[A-Z]{2}/).map { |match| match[0] }[1..]
	valves[name] = Valve.new(flow, paths)
}

max = 0
timelimit = [30, 26]

2.times do |round|
	(100000<<(round*3)).times do
		pressure = 0
		opened = Set(String).new
		(round + 1).times do
			time = timelimit[round]
			cur = "AA"
			prev = "AA"
			while time >= 0
				valve = valves[cur]
				if valve.flow > 0 && !opened.includes?(cur) && rand > 0.15
					opened.add(cur)
					time -= 1
					pressure += time * valve.flow
				end
				paths = valve.paths.clone
				paths.delete(prev) if paths.size > 1 && rand > 0.05 && paths.includes?(prev)
				prev = cur
				cur = paths[rand(paths.size)]
				time -= 1
			end
		end
		if pressure > max
			max = pressure
		end
	end
	puts max
	max = 0
end
