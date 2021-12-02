let rec read_lines ic =
    let num = int_of_string (input_line ic) in
    num :: try read_lines ic with End_of_file -> []


let rec count_increases l =
    match l with
    | [] | [_] -> 0
    | a :: b :: t -> let increased = (if a < b then 1 else 0) in
        increased + count_increases (b :: t)


let rec sliding_sums l =
    match l with
    | [] | [_] | [_; _] -> []
    | a :: b :: c :: t -> let sum = a + b + c in
        sum :: sliding_sums (b :: c :: t)


exception ToManyArguments

let open_input = open_in (match Array.length Sys.argv with
    | 1 -> "input"
    | 2 -> Sys.argv.(1)
    | _ -> raise ToManyArguments)

let () =
    let file = open_input in
    let numbers = read_lines file in
    print_endline (string_of_int (count_increases numbers));
    print_endline (string_of_int (count_increases (sliding_sums numbers)))
