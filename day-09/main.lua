---@param input string
---@return integer
function findExtrapolated(input)
  local nums = retrieveNums(input)
  local res = 0

  for i = 1, #nums do
    local extrapolatedVal = extrapolate(nums[i], false)
    res = res + extrapolatedVal
  end

  return res
end

---@param input string
---@return integer
function findBackwardsExtrapolated(input)
  local nums = retrieveNums(input)
  local res = 0

  for i = 1, #nums do
    local extrapolatedVal = extrapolate(nums[i], true)
    res = res + extrapolatedVal
  end

  return res
end

---@param nums integer[]
---@param backwards boolean
---@return integer
function extrapolate(nums, backwards)
  if isAllZeroes(nums) then
    return 0
  end
  local differences = findDifferences(nums)

  if backwards then
    return nums[1] - extrapolate(differences, backwards)
  else
    return nums[#nums] + extrapolate(differences, backwards)
  end
end

---@param nums integer[]
---@return integer[]
function findDifferences(nums)
  local res = {}
  for i = 1, #nums - 1 do
    table.insert(res, nums[i + 1] - nums[i])
  end
  return res
end

---@param nums integer[]
---@return boolean
function isAllZeroes(nums)
  for i = 1, #nums do
    if nums[i] ~= 0 then
      return false
    end
  end
  return true
end

---@param input string
---@return integer[][]
function retrieveNums(input)
  local res = {}
  for line in string.gmatch(input, "[^\n]+") do
      local nums = {}
      local splittedNums = myEbatoriaSplit(line, " ")
      for i = 1, #splittedNums do
        table.insert(nums, tonumber(splittedNums[i]))
      end
      table.insert(res, nums)
  end
  return res
end

---@param input string
---@param sep string
---@return string[]
function myEbatoriaSplit (input, sep)
  local t={}
  for str in string.gmatch(input, "([^"..sep.."]+)") do
    table.insert(t, str)
  end
  return t
end


function main()
  local input = io.open("example_input.txt", "r"):read("a")

  local res1 = findExtrapolated(input)
  local res2 = findBackwardsExtrapolated(input)

  print(res1)
  print(res2)
end

main()