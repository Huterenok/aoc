defmodule Main do
  def i_do_not_want_to_disintegrate_these_blocks(input) do
    {bricks, supports, supported_by} = find_supporters_and_supported(input)

    0..(length(bricks) - 1)
    |> Enum.filter(fn i ->
      Enum.all?(supports[i], fn j -> MapSet.size(supported_by[j]) >= 2 end)
    end)
    |> Enum.count()
  end

  def i_want_to_disintegrate_these_blocks(input) do
    {bricks, supports, supported_by} = find_supporters_and_supported(input)

    Enum.reduce(0..(length(bricks) - 1), 0, fn i, acc ->
      q =
        Enum.filter(supports[i], fn j ->
          MapSet.size(supported_by[j]) == 1
        end)
        |> Enum.to_list()

      falling = MapSet.new(q) |> MapSet.put(i)

      {q, falling} = process_queue(q, falling, supports, supported_by)
      acc + MapSet.size(falling) - 1
    end)
  end

  defp process_queue([], falling, _supports, _supported_by), do: {[], falling}

  defp process_queue([head | q_tail], falling, supports, supported_by) do
    remaining_supports = MapSet.difference(supports[head], falling)

    {new_q, new_falling} =
      Enum.reduce(remaining_supports, {q_tail, falling}, fn k, {q_acc, falling_acc} ->
        if MapSet.subset?(MapSet.new(supported_by[k]), falling_acc) do
          {[k | q_acc], MapSet.put(falling_acc, k)}
        else
          {q_acc, falling_acc}
        end
      end)

    process_queue(new_q, new_falling, supports, supported_by)
  end

  defp find_supporters_and_supported(input) do
    grid = parse_grid(input)

    bricks =
      Enum.with_index(grid)
      |> Enum.reduce([], fn {brick, i}, acc ->
        max_z =
          Enum.slice(acc, 0, i)
          |> Enum.reduce(1, fn under_brick, acc_inner ->
            if does_this_brick_support_another_one(brick, under_brick) do
              max(acc_inner, Enum.at(under_brick, 5) + 1)
            else
              acc_inner
            end
          end)

        updated_brick =
          brick
          |> List.update_at(5, fn val -> val - Enum.at(brick, 2) + max_z end)
          |> List.update_at(2, fn _ -> max_z end)

        acc ++ [updated_brick]
      end)
      |> Enum.sort_by(&Enum.at(&1, 2))

    supports =
      Enum.map(0..(length(bricks) - 1), fn i -> {i, MapSet.new()} end) |> Enum.into(%{})

    supported_by =
      Enum.map(0..(length(bricks) - 1), fn i -> {i, MapSet.new()} end) |> Enum.into(%{})

    {supports, supported_by} =
      Enum.with_index(bricks)
      |> Enum.reduce({supports, supported_by}, fn {upper, j}, {supps, supd_by} ->
        lower_bricks =
          Enum.slice(bricks, 0, j)
          |> Enum.with_index()

        Enum.reduce(lower_bricks, {supps, supd_by}, fn {lower, i}, {acc_supps, acc_supd} ->
          if does_this_brick_support_another_one(lower, upper) &&
               Enum.at(upper, 2) == Enum.at(lower, 5) + 1 do
            updated_supps = Map.update!(acc_supps, i, fn set -> MapSet.put(set, j) end)
            updated_supd = Map.update!(acc_supd, j, fn set -> MapSet.put(set, i) end)
            {updated_supps, updated_supd}
          else
            {acc_supps, acc_supd}
          end
        end)
      end)

    {bricks, supports, supported_by}
  end

  defp does_this_brick_support_another_one(a, b) do
    max(Enum.at(a, 0), Enum.at(b, 0)) <= min(Enum.at(a, 3), Enum.at(b, 3)) &&
      max(Enum.at(a, 1), Enum.at(b, 1)) <= min(Enum.at(a, 4), Enum.at(b, 4))
  end

  def parse_grid(input) do
    String.replace(input, "~", ",")
    |> String.split("\n")
    |> Enum.map(fn line ->
      line
      |> String.split(",")
      |> Enum.map(&String.to_integer(&1, 10))
    end)
    |> Enum.sort_by(&Enum.at(&1, 2))
  end

  def main() do
    {_, example_input} = File.read("./example_input.txt")
    {_, input} = File.read("./input.txt")

    res1_example = i_do_not_want_to_disintegrate_these_blocks(example_input)
    res1 = i_do_not_want_to_disintegrate_these_blocks(input)
    IO.puts("Result 1: example - #{res1_example}, real - #{res1}")

    res2_example = i_want_to_disintegrate_these_blocks(example_input)
    res2 = i_want_to_disintegrate_these_blocks(input)
    IO.puts("Result 1: example - #{res2_example}, real - #{res2}")
  end
end

Main.main()
