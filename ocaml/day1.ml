let lines_from_file ic =
  let rec lines_from_file_aux ic acc =
    match input_line ic with
    | line -> lines_from_file_aux ic (line :: acc)
    | exception End_of_file ->
        close_in ic;
        List.rev acc
  in
  List.rev (lines_from_file_aux ic [])

let rec pow10 exp = match exp with 0 -> 1 | 1 -> 10 | _ -> 10 * pow10 (exp - 1)

let ints_from_input_string s =
  let chars = List.init (String.length s) (fun i -> String.sub s i 1) in
  let ints = [] in
  let rec ints_from_input_string_aux chars ints =
    match chars with
    | [] -> ints
    | c :: cs -> (
        match int_of_string c with
        | i -> ints_from_input_string_aux cs (i :: ints)
        | exception _ -> ints_from_input_string_aux cs ints)
  in
  List.rev (ints_from_input_string_aux chars ints)

let print_ints l =
  List.iter
    (fun x ->
      print_int x;
      print_string " ")
    l;
  print_newline ()

let head_and_tail (l : int list) =
  print_string "List: ";
  print_ints l;
  let hd, tl =
    match l with
    | [] -> failwith "Empty list"
    | _ -> (List.hd l, List.hd (List.rev l))
  in
  print_int hd;
  print_string " ";
  print_int tl;
  print_newline ();
  (hd, tl)

let calibration_value (x, y) =
  let value = (x * 10) + y in
  print_string "Calibration value: ";
  print_int value;
  print_newline ();
  print_newline ();
  value

let part_1 filename =
  let ic = open_in filename in
  let lines = lines_from_file ic in
  let int_lists = List.map ints_from_input_string lines in
  let heads_and_tails = List.map head_and_tail int_lists in
  let calibration_values = List.map calibration_value heads_and_tails in
  List.fold_left ( + ) 0 calibration_values

type replacement = { to_replace : Str.regexp; replace_with : string }

let rec replace_all_in_string s =
  let regex =
    Str.regexp "\(one\|two\|three\|four\|five\|six\|seven\|eight\|nine\)"
  in
  match Str.search_forward regex s 0 with
  | exception Not_found ->
      print_string "Final: ";
      print_string s;
      print_newline ();
      s
  | _ -> (
      match Str.matched_string s with
      | "one" -> replace_all_in_string (Str.replace_first regex "1" s)
      | "two" -> replace_all_in_string (Str.replace_first regex "2" s)
      | "three" -> replace_all_in_string (Str.replace_first regex "3" s)
      | "four" -> replace_all_in_string (Str.replace_first regex "4" s)
      | "five" -> replace_all_in_string (Str.replace_first regex "5" s)
      | "six" -> replace_all_in_string (Str.replace_first regex "6" s)
      | "seven" -> replace_all_in_string (Str.replace_first regex "7" s)
      | "eight" -> replace_all_in_string (Str.replace_first regex "8" s)
      | "nine" -> replace_all_in_string (Str.replace_first regex "9" s)
      | _ -> failwith "Should not have matched")

let part_2 filename =
  let ic = open_in filename in
  let lines = lines_from_file ic in
  let final s =
    calibration_value
      (head_and_tail (ints_from_input_string (replace_all_in_string s)))
  in
  let calibration_values =
    List.map
      (fun x ->
        print_string "Original:";
        print_string x;
        print_newline ();
        final x)
      lines
  in
  List.fold_left ( + ) 0 calibration_values

let () =
  print_string "Part 1: ";
  print_endline (string_of_int (part_1 "./data/day1.data"));
  print_string "Part 2: ";
  print_endline (string_of_int (part_2 "./data/day1.data"))
