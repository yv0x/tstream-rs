(* This program is used to archive old conversations. 
 It scans the 'conversations' directory for files from the last month, 
 and moves them to a new directory named after the month. *)
open Sys
open Unix

let starts_with s prefix =
  let prefix_len = String.length prefix in
  let s_len = String.length s in
  s_len >= prefix_len && String.sub s 0 prefix_len = prefix

let get_last_month_prefix () =
  let current_time = Unix.time () in
  let tm = Unix.gmtime current_time in
  let year = tm.tm_year + 1900 in  (* tm_year is years since 1900 *)
  let month = tm.tm_mon + 1 in     (* tm_mon is in the range [0..11] *)
  if month = 1 then
    (string_of_int (year - 1), "12")
  else
    (string_of_int year, Printf.sprintf "%02d" (month - 1))

let move_files_from_last_month () =
  let dir = "../conversations" in
  let handle = opendir dir in
  
  let rec read_all_files dh acc =
    try
      let next_file = readdir dh in
      read_all_files dh (next_file :: acc)
    with End_of_file -> acc
  in
  
  let files = read_all_files handle [] in
  closedir handle;

  let (last_year, last_month) = get_last_month_prefix () in
  let prefix = last_year ^ "-" ^ last_month ^ "-" in

  let last_month_files = 
    List.filter (fun f -> starts_with f prefix) files 
  in

  (* Only create the new folder and move files if there are valid files *)
  if not (last_month_files = []) then
    let new_folder = Filename.concat dir last_month in
    if not (file_exists new_folder) then mkdir new_folder 0o777;
    
    List.iter 
      (fun f ->
        let old_path = Filename.concat dir f in
        let new_path = Filename.concat new_folder f in
        rename old_path new_path
      ) last_month_files

let () =
  move_files_from_last_month ()
