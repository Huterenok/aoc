import Foundation

struct Path {
  var left: String
  var right: String
}

func findDaWay(_ input: String) -> Int {
  let (steps, paths) = retrieveStepsAndPaths(input)

  var res = 0
  var currentStep = "AAA"

  while currentStep != "ZZZ" {
    let strIndex = steps.index(steps.startIndex, offsetBy: res % steps.count)
    let char = steps[strIndex]

    res += 1
    currentStep = char == "R" ? paths[currentStep]!.right : paths[currentStep]!.left
  }
 
  return res
}

func findDaGhostWay(_ input: String) -> Int {
  let (steps, paths) = retrieveStepsAndPaths(input)
  let aPaths = paths.keys.filter({$0.last == "A"})

  let zPaths = aPaths.map({(path: String) -> Int in
    var step = 0
    var path = path;

    while path.last != "Z" {
        let strIndex = steps.index(steps.startIndex, offsetBy: step % steps.count)
        let char = steps[strIndex]

        path = char == "R" ? paths[path]!.right : paths[path]!.left
        step += 1
    }

    return step
  })

  return lcm(zPaths)
}

func retrieveStepsAndPaths(_ input: String) -> (String, [String : Path]) {
  let stepsMatcher = try! NSRegularExpression(pattern: "([LR]+)(?=\\n)")
  let pathMatcher = try! NSRegularExpression(pattern: "\\b(.{3})\\b = \\(\\b(.{3})\\b, \\b(.{3})\\b\\)")

  let stepsMatches = stepsMatcher.matches(in: input, range: NSRange(input.startIndex..., in: input))
  
  let steps = String(input[Range(stepsMatches.first!.range, in: input)!])
  var paths: [String : Path] = [:]
  
  for _ in input.split(separator: "\\n") {
    let pathMatches = pathMatcher.matches(in: input, range: NSRange(input.startIndex..., in: input))

    for match in pathMatches {
      let key = String(input[Range(match.range(at: 1), in: input)!])
      let left = String(input[Range(match.range(at: 2), in: input)!])
      let right = String(input[Range(match.range(at: 3), in: input)!])
      
      paths[key] = Path(left: left, right: right)
    }
  }

  return (steps, paths)
}

func gcd(_ a: Int, _ b: Int) -> Int {
  let r = a % b
  if r != 0 {
    return gcd(b, r)
  } else {
    return b
  }
}

func lcm(_ numbers: [Int]) -> Int {
  return numbers.reduce(1) { $0 * $1 / gcd($0, $1)  }
}

let fileUrl1 = Bundle.main.url(forResource: "example_input1", withExtension: "txt")!
let fileUrl2 = Bundle.main.url(forResource: "example_input2", withExtension: "txt")!

func main() {
  do {
    let input1 = try String(contentsOf: fileUrl1, encoding: .ascii)
    let input2 = try String(contentsOf: fileUrl2, encoding: .ascii)

    let res1 = findDaWay(input1)
    let res2 = findDaGhostWay(input2)

    print(res1);
    print(res2);
  } catch {
    print("RETARDED!!!")
  }
}

main()