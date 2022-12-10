#!/usr/bin/env vim -nNesc:let&verbose=1|let&viminfo=""|source%|echo""|qall!

let s:input = readfile("input.txt")
let s:height = len(s:input)
let s:width = len(s:input[0])

" Part 1
let s:vismap = map(range(s:height), {k,v -> map(range(s:width), v:false)})

function! s:create_vismap(r1, r2, vert)
	for i in a:r1
		let max = -1
		for j in a:r2
			let [v,h] = a:vert ? [j,i] : [i,j]

			if s:input[v][h] > max
				let max = s:input[v][h] + 0
				let s:vismap[v][h] = v:true
			endif
		endfor
	endfor
endfunction

call s:create_vismap(range(s:height), range(s:width), v:false)
call s:create_vismap(range(s:height), range(s:width-1, 0, -1), v:false)
call s:create_vismap(range(s:width), range(s:height), v:true)
call s:create_vismap(range(s:width), range(s:height-1, 0, -1), v:true)

let s:count = 0
for line in s:vismap
	for tree in line
		if tree
			let s:count += 1
		endif
	endfor
endfor

" Part 2
function! s:calc_scenscore(i, j)
	let tree = s:input[a:i][a:j]
	let score = 1

	for pos in range(a:j+1, s:width-1)
		if s:input[a:i][pos] >= tree || pos == s:width - 1
			let score = score * (pos-a:j)
			break
		endif
	endfor
	for pos in range(a:j-1, 0, -1)
		if s:input[a:i][pos] >= tree || pos == 0
			let score = score * (a:j-pos)
			break
		endif
	endfor
	for pos in range(a:i+1, s:height-1)
		if s:input[pos][a:j] >= tree || pos == s:height - 1
			let score = score * (pos-a:i)
			break
		endif
	endfor
	for pos in range(a:i-1, 0, -1)
		if s:input[pos][a:j] >= tree || pos == 0
			let score = score * (a:i-pos)
			break
		endif
	endfor

	return score
endfunction

let s:max_score = 0
for i in range(1, s:height-2)
	for j in range(1, s:width-2)
		let s:cur_score = s:calc_scenscore(i, j)
		if s:cur_score > s:max_score
			let s:max_score = s:cur_score
		endif
	endfor
endfor


echo(s:count)
echo(s:max_score)
