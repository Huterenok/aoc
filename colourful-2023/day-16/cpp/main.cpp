#include <deque>
#include <fstream>
#include <iostream>
#include <ostream>
#include <sstream>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>

enum Tile { Dot, Horiz, Vert, UpRight, UpLeft };

typedef std::vector<std::vector<Tile>> Grid;
typedef std::tuple<int, int, int, int> EnergizeState;
typedef std::pair<int, int> Point;

struct EnergizeStateHash {
  int operator()(const EnergizeState &c) const {
    auto [x, y, dx, dy] = c;
    return std::hash<int>()(x) ^ std::hash<int>()(y) ^ std::hash<int>()(dx) ^
           std::hash<int>()(dy);
  }
};

struct EnergizeStateEqual {
  bool operator()(const EnergizeState &a, const EnergizeState &b) const {
    return a == b;
  }
};

struct PointHash {
  int operator()(const Point &c) const {
    return std::hash<int>()(c.first) ^ std::hash<int>()(c.second);
  }
};

Tile convertToTile(char symbol) {
  switch (symbol) {
  case '.':
    return Tile::Dot;
  case '-':
    return Tile::Horiz;
  case '|':
    return Tile::Vert;
  case '/':
    return Tile::UpRight;
  case '\\':
    return Tile::UpLeft;
  default:
    std::cout << "RETARDED";
    exit(1);
  }
}

Grid parseGrid(std::string input) {
  std::vector<std::vector<Tile>> grid;
  std::istringstream iss(input);
  std::string line;

  while (std::getline(iss, line)) {
    std::vector<Tile> row;
    for (char ch : line) {
      row.push_back(convertToTile(ch));
    }
    grid.push_back(row);
  }

  return grid;
}

int energize(const Grid &grid, EnergizeState start) {
  std::deque<EnergizeState> queue = {start};
  std::unordered_set<EnergizeState, EnergizeStateHash, EnergizeStateEqual> set;

  while (!queue.empty()) {
    auto [x, y, dx, dy] = queue.front();
    queue.pop_front();

    x += dx;
    y += dy;
    if (x < 0 || x >= grid[0].size() || y < 0 || y >= grid.size()) {
      continue;
    }

    Tile tile = grid[y][x];

    if (tile == Tile::Dot || (tile == Tile::Vert && dy != 0) ||
        (tile == Tile::Horiz && dx != 0)) {
      if (set.insert({x, y, dx, dy}).second) {
        queue.push_back({x, y, dx, dy});
      }
    } else if (tile == Tile::UpRight) {
      int new_dx = -dy;
      int new_dy = -dx;
      if (set.insert({x, y, new_dx, new_dy}).second) {
        queue.push_back({x, y, new_dx, new_dy});
      }
    } else if (tile == Tile::UpLeft) {
      int new_dx = dy;
      int new_dy = dx;
      if (set.insert({x, y, new_dx, new_dy}).second) {
        queue.push_back({x, y, new_dx, new_dy});
      }
    } else {
      if (tile == Tile::Vert) {
        std::vector<std::pair<int, int>> directions = {{0, 1}, {0, -1}};
        for (auto [dir_x, dir_y] : directions) {
          if (set.insert({x, y, dir_x, dir_y}).second) {
            queue.push_back({x, y, dir_x, dir_y});
          }
        }
      } else {
        std::vector<std::pair<int, int>> directions = {{1, 0}, {-1, 0}};
        for (auto [dir_x, dir_y] : directions) {
          if (set.insert({x, y, dir_x, dir_y}).second) {
            queue.push_back({x, y, dir_x, dir_y});
          }
        }
      }
    }
  }

  std::unordered_set<Point, PointHash> uniqueCoords;

  for (auto elem : set) {
    auto [x, y, _dx, _dy] = elem;
    uniqueCoords.insert({x, y});
  }

  return uniqueCoords.size();
}

int findMaxEnergized(const Grid &grid) {
  int max_energized = 0;

  for (int y = 0; y < grid.size(); ++y) {
    int energized1 = energize(grid, {-1, y, 1, 0});
    int energized2 = energize(grid, {grid[0].size(), y, -1, 0});
    max_energized = std::max(max_energized, energized1);
    max_energized = std::max(max_energized, energized2);
  }

  for (int x = 0; x < grid[0].size(); ++x) {
    int energized1 = energize(grid, {x, -1, 0, 1});
    int energized2 = energize(grid, {x, grid.size(), 0, -1});
    max_energized = std::max(max_energized, energized1);
    max_energized = std::max(max_energized, energized2);
  }

  return max_energized;
}

std::string readFile(std::string filename) {
  std::ifstream file(filename);

  std::stringstream buffer;
  buffer << file.rdbuf();
  std::string content = buffer.str();

  file.close();
  return content;
}

int main() {
  std::string input = readFile("../../input.txt");
  std::string exampleInput = readFile("../../example_input.txt");
  Grid exampleGrid = parseGrid(exampleInput);
  Grid grid = parseGrid(input);

  EnergizeState res1State = EnergizeState(-1, 0, 1, 0);
  int res1Example = energize(exampleGrid, res1State);
  int res1 = energize(grid, res1State);
  printf("Result 1: example - %d, real - %d\n", res1Example, res1);

  int res2Example = findMaxEnergized(exampleGrid);
  int res2 = findMaxEnergized(grid);
  printf("Result 2: example - %d, real - %d\n", res2Example, res2);

  return 0;
}
