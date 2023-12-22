using DataStructures

@enum CellType begin
  Super = 1
  Plot = 2
  Rock = 3
end

struct State
  x::Int
  y::Int
  steps::Int
end

struct ParsedGrid
  s_position::Tuple{Integer,Integer}
  grid::Array{Array{CellType}}
end

function heavyMathematicalComputations(parsed_grid::ParsedGrid)::Integer
  height = length(parsed_grid.grid)
  grid_width = 26501365 ÷ height - 1
  odd = (grid_width ÷ 2 * 2 + 1)^2
  even = ((grid_width + 1) ÷ 2 * 2)^2

  odd_points = countSteps(parsed_grid.grid, (parsed_grid.s_position[2], parsed_grid.s_position[1]), height * 2 + 1)
  even_points = countSteps(parsed_grid.grid, (parsed_grid.s_position[2], parsed_grid.s_position[1]), height * 2)

  corner_t = countSteps(parsed_grid.grid, (height, parsed_grid.s_position[1]), height - 1)
  corner_r = countSteps(parsed_grid.grid, (parsed_grid.s_position[2], 1), height - 1)
  corner_b = countSteps(parsed_grid.grid, (1, parsed_grid.s_position[1]), height - 1)
  corner_l = countSteps(parsed_grid.grid, (parsed_grid.s_position[2], height), height - 1)

  small_tr = countSteps(parsed_grid.grid, (1, height), height ÷ 2 - 1)
  small_tl = countSteps(parsed_grid.grid, (height, height), height ÷ 2 - 1)
  small_br = countSteps(parsed_grid.grid, (1, 1), height ÷ 2 - 1)
  small_bl = countSteps(parsed_grid.grid, (height, 1), height ÷ 2 - 1)

  large_tr = countSteps(parsed_grid.grid, (1, height), height * 3 ÷ 2 - 1)
  large_tl = countSteps(parsed_grid.grid, (height, height), height * 3 ÷ 2 - 1)
  large_br = countSteps(parsed_grid.grid, (1, 1), height * 3 ÷ 2 - 1)
  large_bl = countSteps(parsed_grid.grid, (height, 1), height * 3 ÷ 2 - 1)

  return odd * odd_points +
         even * even_points +
         corner_t + corner_r + corner_b + corner_l +
         (grid_width + 1) * (small_tr + small_tl + small_br + small_bl) +
         grid_width * (large_tr + large_tl + large_br + large_bl)
end

function countSteps(grid::Array{Array{CellType}}, s_position::Tuple{Integer,Integer}, steps::Integer)::Integer
  res = Set{Tuple{Int,Int}}()
  visited = Set{Tuple{Int,Int}}()
  queue = Deque{State}()

  push!(queue, State(s_position[1], s_position[2], steps))
  push!(visited, (s_position[1], s_position[2]))

  while !isempty(queue)
    popped = popfirst!(queue)
    if popped.steps % 2 == 0
      push!(res, (popped.x, popped.y))
    end
    if popped.steps == 0
      continue
    end

    for (dX, dY) in [(popped.x, popped.y + 1), (popped.x, popped.y - 1), (popped.x + 1, popped.y), (popped.x - 1, popped.y)]
      if dX < 1 || dX > length(grid[1]) || dY < 1 || dY > length(grid) || grid[dY][dX] == Rock || (dX, dY) in visited
        continue
      end
      push!(queue, State(dX, dY, popped.steps - 1))
      push!(visited, (dX, dY))
    end
  end

  return length(res)
end

function parseGrid(input::String)::ParsedGrid
  sX, sY = -1, -1
  lines = split(input, "\n")
  grid = [Array{CellType}(undef, length(lines[1])) for _ in 1:length(lines)]

  for (y, line) in enumerate(lines)
    for (x, ch) in enumerate(line)
      if ch == 'S'
        sX, sY = x, y
        grid[y][x] = Super
      elseif ch == '#'
        grid[y][x] = Rock
      else
        grid[y][x] = Plot
      end
    end
  end

  return ParsedGrid((sX, sY), grid)
end

function main()
  input = read("./input.txt", String)
  # example_input = read("./example_input.txt", String)

  parsed_grid = parseGrid(input)
  res1 = countSteps(parsed_grid.grid, parsed_grid.s_position, 64)
  println("Result 1: real - $res1")

  res2 = heavyMathematicalComputations(parsed_grid)
  println("Result 2: real - $res2")
end

main()