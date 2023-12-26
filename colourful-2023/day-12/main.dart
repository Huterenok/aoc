import 'dart:io';

class Solution {
  Map<SpringKey, int> _cache = {};

  int findSpringArrangments(List<String> input) {
    return input.fold(0, (previousValue, element) {
      var data = retrieveData(element);
      return previousValue + count(data.spring, data.nums);
    });
  }

  int findUnfoldedSpringArrangments(List<String> input) {
    return input.fold(0, (previousValue, element) {
      var data = retrieveData(element);
      var exactSpring = List.filled(5, data.spring).join('?');
      var exactNums = List.filled(5, data.nums).expand((x) => x).toList();

      return previousValue + count(exactSpring, exactNums);
    });
  }

  int count(String spring, List<int> nums) {
    if (spring == "") {
      return nums.isEmpty ? 1 : 0;
    }
    if (nums.isEmpty) {
      return !spring.contains('#') ? 1 : 0;
    }

    var key = SpringKey(spring, nums);
    if (_cache.containsKey(key)) {
      return _cache[key]!;
    }

    var res = 0;
    if ('.?'.contains(spring[0])) {
      res += count(spring.substring(1), nums);
    }

    if ('#?'.contains(spring[0])) {
      if (nums.isNotEmpty &&
          nums[0] <= spring.length &&
          !spring.substring(0, nums[0]).contains('.') &&
          (nums[0] == spring.length || spring[nums[0]] != '#')) {
        int add = nums[0] == spring.length ? 0 : 1;
        res += count(spring.substring(nums[0] + add), nums.sublist(1));
      }
    }

    _cache[key] = res;

    return res;
  }
}

class SpringData {
  String spring;
  List<int> nums;

  SpringData(this.spring, this.nums);
}

//OMG
class SpringKey {
  String spring;
  List<int> nums;

  SpringKey(this.spring, this.nums);

  @override
  bool operator ==(Object other) =>
      other is SpringKey &&
      other.spring == spring &&
      _listEquals(other.nums, nums);

  @override
  int get hashCode => spring.hashCode ^ _hashCodeForList(nums);

  static bool _listEquals<T>(List<T>? a, List<T>? b) {
    if (a == null) return b == null;
    if (b == null || a.length != b.length) return false;
    for (int i = 0; i < a.length; i++) {
      if (a[i] != b[i]) return false;
    }
    return true;
  }

  static int _hashCodeForList(List<Object> list) {
    return list.fold(
        0, (previousValue, element) => previousValue ^ element.hashCode);
  }
}

SpringData retrieveData(String input) {
  var data = input.split(" ");
  return SpringData(
      data[0], data[1].split(",").map((e) => int.parse(e)).toList());
}

void main() async {
  var example_input =
      await File("example_input.txt").readAsLines().then((value) => value);
  var input = await File("input.txt").readAsLines().then((value) => value);

  var sol = Solution();

  int res1_example = sol.findSpringArrangments(example_input);
  int res2_example = sol.findUnfoldedSpringArrangments(example_input);

  int res1 = sol.findSpringArrangments(input);
  int res2 = sol.findUnfoldedSpringArrangments(input);

  if (res1_example != 21) print("RETARDED res1_example!!!");
  if (res2_example != 525152) print("RETARDED res2_example!!!");

  if (res1 != 7195) print("RETARDED res1!!!");
  if (res2 != 33992866292225) print("RETARDED res2!!!");
}
