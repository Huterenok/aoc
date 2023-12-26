open System.Collections.Generic

type Solution() =
    let mutable boxes: string list array = Array.init 256 (fun _ -> [])
    let mutable focals = Dictionary()

    member this.resolveBoxes(input: string) =
        input.Split(',')
        |> Array.iter (fun seq ->
            if seq.Contains("-") then
                let label = seq.[0 .. (seq.Length - 2)]
                let index = this.hash label

                if boxes.[index] |> List.contains label then
                    boxes.[index] <- List.filter (fun l -> l <> label) boxes.[index]
            else
                let parts = seq.Split('=')
                let label, length = parts.[0], int parts.[1]
                let index = this.hash label

                if not (boxes.[index] |> List.contains label) then
                    boxes.[index] <- boxes.[index] @ [ label ]

                focals.[label] <- length)

    member this.findTotalBoxes(input: string) : int =
        this.resolveBoxes (input) |> ignore

        boxes
        |> Array.mapi (fun boxNumber box ->
            box
            |> List.mapi (fun lensSlot label -> (boxNumber + 1) * (lensSlot + 1) * focals[label])
            |> List.sum)
        |> Array.sum

    member this.findTotalHash(input: string) : int =
        input.Split(',') |> Array.fold (fun sum curr -> sum + this.hash curr) 0

    member this.hash(seq: string) : int =
        seq.ToCharArray()
        |> Array.fold (fun sum curr -> ((sum + int curr) * 17) % 256) 0

let example_input = System.IO.File.ReadAllText("example_input.txt")
let input = System.IO.File.ReadAllText("input.txt")
let sol1 = Solution()
let sol2 = Solution()

let res1_example = sol1.findTotalHash example_input
let res1 = sol1.findTotalHash input
printfn "Result 1: example - %d, real - %d" res1_example res1

let res2_example = sol1.findTotalBoxes example_input
let res2 = sol2.findTotalBoxes input
printfn "Result 2: example - %d, real - %d" res2_example res2
