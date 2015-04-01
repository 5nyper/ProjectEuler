v = [1 , 2 , 3 , 5 , 8 , 13 , 21 , 34 , 55]

res = 0

23.times do
	x = v[v.length - 2] + v[v.length - 1]
	v.push(x)
end

v.length.times do |i|
	if v[i] % 2 == 0
		res += v[i]
	end
end

puts res
