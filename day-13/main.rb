def find_mirror(grid)
  (1...grid.length).each do |r|
    above = grid[0...r].reverse
    below = grid[r..]

    min_length = [above.length, below.length].min
    above = above[0...min_length]
    below = below[0...min_length]

    return r if above == below
  end

  0
end

def find_smudged_mirror(grid)
  (1...grid.length).each do |r|
    above = grid[0...r].reverse
    below = grid[r..]

    min_length = [above.length, below.length].min
    above = above[0...min_length]
    below = below[0...min_length]

    diff_count = above.zip(below).sum do |x, y|
      x.zip(y).count { |a, b| a != b }
    end

    return r if diff_count == 1
  end

  0
end

res1 = 0
res2 = 0

File.read("input.txt").split("\n\n").each do |block|
  grid = block.split("\n").map { |line| line.chars }

  row1 = find_mirror(grid)
	col1 = find_mirror(grid.transpose)
  res1 += row1 * 100 + col1

	row2 = find_smudged_mirror(grid)
	col2 = find_smudged_mirror(grid.transpose)
  res2 += row2 * 100 + col2
end

puts res1
puts res2