let ( <<< ) f g = fun x -> f (g x)

let ( <| ) f a = f a

let curry f x y = f (x, y)
let uncurry f (x, y) = f x y

exception BadInput

exception ToManyArguments

let open_input _ = open_in (match Array.length Sys.argv with
    | 1 -> "input"
    | 2 -> Sys.argv.(1)
    | _ -> raise ToManyArguments)


module Board = struct
    type board = (bool * int) list list

    let print b =
        Printf.printf "Board:\n";
        List.iter (fun line ->
            List.iter (function b, n -> Printf.printf "(%b, %02d), " b n) line;
            print_newline ()
        ) b

    let transpose l = match l with
        | [] -> []
        | (h :: t) -> List.fold_left (List.map2 (Fun.flip List.cons)) (List.map (fun x -> [x]) h) t


    let won board =
        let all_true = List.exists (fun line -> List.fold_left (fun st (b, _) -> st && b) true line) in
        all_true board || (all_true <<< transpose) board


    let update num = List.map (List.map (fun (b, n) -> (b || num == n, n)))

    let score n = (fun x -> n * x)
        <<< Seq.fold_left (+) 0
        <<< Seq.map snd
        <<< Seq.filter (not <<< fst)
        <<< Seq.flat_map List.to_seq
        <<< List.to_seq

    let rec read ic =
        match input_line ic with
        | "" -> []
        | l -> (List.map (fun x -> (false, x))
                <<< List.map int_of_string
                <<< List.filter (fun x -> String.length x > 0)
                <<< String.split_on_char ' ') l :: read ic
end

let print_boards boards =
    List.iter Board.print boards

let rec read_all_boards ic =
    try
        let b = Board.read ic in
        b :: read_all_boards ic
    with End_of_file ->
        close_in ic;
        []

let rec find_first_winner boards nums =
    match nums with
    | [] -> None
    | (h :: t) -> let updated_boards = List.map (Board.update h) boards in
        match List.find_opt Board.won updated_boards with
        | None -> find_first_winner updated_boards t
        | Some winner -> Some (winner, h)

let rec find_last_winner boards last_winner_found nums =
    match nums with
    | [] -> last_winner_found
    | (h :: t) -> let updated_boards = List.map (Board.update h) boards in
        let found = match List.find_opt Board.won updated_boards with
        | None -> last_winner_found
        | Some winner -> Some (winner, h) in
        let loosers = List.filter (not <<< Board.won) updated_boards in
        find_last_winner loosers found t


let () =
    let file = open_input () in
    let nums = input_line file
        |> String.split_on_char ','
        |> List.map int_of_string in
    let _ = input_line file in
    let all_boards = read_all_boards file in

    match find_first_winner all_boards nums with
    | None -> Printf.printf "no winner\n"
    | Some (winner, n) -> Printf.printf "first winner score: %d\n" (Board.score n winner);

    match find_last_winner all_boards None nums with
    | None -> Printf.printf "no winner\n"
    | Some (winner, n) -> Printf.printf "last winner score: %d\n" (Board.score n winner)
