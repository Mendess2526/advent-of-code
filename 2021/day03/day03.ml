exception BadInput

let ( <<< ) f g = fun x -> f (g x)

let ( <| ) f a = f a

let curry f x y = f (x, y)
let uncurry f (x, y) = f x y

let to_bin_array =
    List.of_seq
    <<< Seq.map (fun x -> if x == 0 then -1 else x)
    <<< Seq.map (int_of_string <<< Printf.sprintf "%c")
    <<< String.to_seq

let rec read_lines parse_fn ic =
    let e = input_line ic |> parse_fn in
    e :: try read_lines parse_fn ic with End_of_file -> []

exception ToManyArguments

let open_input () = open_in (match Array.length Sys.argv with
    | 1 -> "input"
    | 2 -> Sys.argv.(1)
    | _ -> raise ToManyArguments)

let coerce_to_bool x =
    if x < 0 then
        false
    else
        true

let int_of_bool b = if b then 1 else 0

let pow base exp =
    Float.pow (Float.of_int base) (Float.of_int exp)
    |> Int.of_float

let bin2dec =
    List.fold_left (+) 0
    <<< List.mapi (fun i b -> (int_of_bool b) * (pow 2 i))
    <<< List.rev

let numbers_folded l =
    let init = Array.to_list <| Array.make (List.length <| List.hd l) 0 in
    List.fold_right (fun state -> List.map2 (+) state) l init

let () =
    let final = open_input ()
        |> read_lines to_bin_array
        |> numbers_folded in
    let gamma = bin2dec <<< List.map coerce_to_bool <| final in
    let epsilon = bin2dec <<< List.map (not <<< coerce_to_bool) <| final in
    Printf.printf "gamma: %d\nepsilon: %d\npower consumption: %d\n" gamma epsilon (gamma * epsilon)


let rec sieve f l =
    match l with
    | [] -> []
    | [x] -> List.map coerce_to_bool x
    | _ -> (
        match List.map coerce_to_bool <| numbers_folded l with
        | [] -> []
        | (h :: _) -> (f h) :: (
            sieve f
                <<< List.map List.tl
                <<< List.filter (fun x -> coerce_to_bool (List.hd x) == (f h))
        ) l
    )


let () =
    let numbers = open_input ()
        |> read_lines to_bin_array in
    let o2 = bin2dec <| sieve Fun.id numbers in
    let co2 = bin2dec <| sieve not numbers in
    Printf.printf "o2: %d\nco2: %d\nlife support rating: %d\n" o2 co2 (o2 * co2)
