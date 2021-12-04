exception BadInput

let forward amount (h, d) = (h + amount, d)
let down amount (h, d) = (h, d + amount)
let up amount (h, d) = (h, d - amount)

let forwardAim amount (h, d, a) = (h + amount, d + (amount * a), a)
let downAim amount (h, d, a) = (h, d, a + amount)
let upAim amount (h, d, a) = (h, d, a - amount)

let parse_line (forward, down, up) line =
    let f inst amount = match inst with
    | "forward" -> forward amount
    | "down" -> down amount
    | "up" -> up amount
    | _ -> raise BadInput in

    match String.split_on_char ' ' line with
    | [instruction; amount] -> f instruction (int_of_string amount)
    | _ -> raise BadInput

let rec read_lines parse_fn ic =
    let line = input_line ic in
    let fn = parse_fn line in
    fn :: try read_lines parse_fn ic with End_of_file -> []

exception ToManyArguments

let open_input _ = open_in (match Array.length Sys.argv with
    | 1 -> "input"
    | 2 -> Sys.argv.(1)
    | _ -> raise ToManyArguments)

let () =
    let file = open_input 0 in
    let fns = read_lines (parse_line (forward, down, up)) file in
    let (h, d) = List.fold_right (fun f pair -> f pair) fns (0, 0) in
    Printf.printf "height: %d, depth: %d => %d\n" h d (h * d)

let () =
    let file = open_input 0 in
    let fns = read_lines (parse_line (forwardAim, downAim, upAim)) file in
    let (h, d, a) = List.fold_left (fun pair f -> f pair) (0, 0, 0) fns in
    Printf.printf "height: %d, depth: %d, aim: %d => %d\n" h d a (h * d)
