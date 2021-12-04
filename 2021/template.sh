#!/bin/bash

set -e

[ "$#" -eq 1 ] || exit

day="$(printf "day%02d" $1)"
echo "DAY: $day"

mkdir "$day"
cd "$day"

eval $(opam env --switch=default)

dune init exe "$day"

cat <<EOF > "$day.ml"
let ( <<< ) f g = fun x -> f (g x)

let ( <| ) f a = f a

let curry f x y = f (x, y)
let uncurry f (x, y) = f x y

exception BadInput

let rec read_lines parse_fn ic =
    let line = input_line ic in
    let e = parse_fn line in
    e :: try read_lines parse_fn ic with End_of_file -> []

exception ToManyArguments

let open_input _ = open_in (match Array.length Sys.argv with
    | 1 -> "input"
    | 2 -> Sys.argv.(1)
    | _ -> raise ToManyArguments)

let () = Printf.printf "ola\n"
EOF

