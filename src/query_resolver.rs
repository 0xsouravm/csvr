use::csvr::{ CSVFile, FileDataUtils };

pub fn query_resolver(query: String, file: &mut CSVFile) {
    let query_elements: Vec<&str> = query.trim().split(' ').collect();
    let num_elements = query_elements.len();
    if num_elements == 0 { return }
    let action = query_elements[0].trim();
    match action {
        "display" | "-d" => {
            if num_elements < 2 {
                eprintln!("\x1b[31mcsvr: argument missing: need either 'row', 'col' or 'file'.\x1b[0m");
                return 
            }
            match query_elements[1].trim() {
                "row" => {
                    if num_elements < 3 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need row index.\x1b[0m");
                        return 
                    }
                    if num_elements > 3 {                         
                        eprintln!("\x1b[31mcsvr: extra arguments found: need only row index.\x1b[0m");
                        return
                    }
                    match query_elements[2].trim().parse::<usize>() {
                        Ok(row_ind) => {
                            let _ = file.display_row(row_ind)
                                        .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
                        }
                        Err(err) => {
                            eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                        }
                    }
                },
                "col" => {
                    if num_elements < 3 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need col index.\x1b[0m");
                        return 
                    }
                    if num_elements > 3 {                         
                        eprintln!("\x1b[31mcsvr: extra arguments found: need only col index.\x1b[0m");
                        return
                    }
                    match query_elements[2].trim().parse::<usize>() {
                        Ok(col_ind) => {
                            let _ = file.display_column(col_ind)
                                        .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
                        }
                        Err(err) => {
                            eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                        }
                    }
                },
                "file" => {
                    if num_elements < 3 {                         
                        let _ = file.display_file(None, None)
                        .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
                        return 
                    }
                    if num_elements < 4 {                         
                        eprintln!("\x1b[31mcsvr: argument missing: need both start and end index.\x1b[0m");
                        return 
                    }
                    if num_elements > 4 {                         
                        eprintln!("\x1b[31mcsvr: extra arguments found: need only start and end index.\x1b[0m");
                        return
                    }
                    match query_elements[2].trim().parse::<usize>() {
                        Ok(s_index) => {
                            match query_elements[3].trim().parse::<usize>() {
                                Ok(e_index) => {
                                    let _ = file.display_file(Some(s_index), Some(e_index))
                                                .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
                                }
                                Err(err) => {
                                    eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                        }
                    }
                },
                _ => eprintln!("\x1b[31mcsvr: incorrect argument: need either 'row', 'col' or 'file'.\x1b[0m")
            }
        }

        "delete" | "-r" => {
            if num_elements < 2 {
                eprintln!("\x1b[31mcsvr: argument missing: need either 'row', 'col' or 'item'.\x1b[0m");
                return 
            }
            match query_elements[1].trim() {
                "row" => {
                    if num_elements < 3 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need row index.\x1b[0m");
                        return 
                    }
                    if num_elements > 3 {                         
                        eprintln!("\x1b[31mcsvr: extra arguments found: need only row index.\x1b[0m");
                        return
                    }
                    match query_elements[2].trim().parse::<usize>() {
                        Ok(row_ind) => {
                            let _ = file.delete_row(row_ind)
                                        .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
                        }
                        Err(err) => {
                            eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                        }
                    }
                },
                "col" => {
                    if num_elements < 3 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need col index.\x1b[0m");
                        return
                    }
                    if num_elements > 3 {                         
                        eprintln!("\x1b[31mcsvr: extra arguments found: need only col index.\x1b[0m");
                        return
                    }
                    match query_elements[2].trim().parse::<usize>() {
                        Ok(col_ind) => {
                            let _ = file.delete_column(col_ind)
                                        .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
                        }
                        Err(err) => {
                            eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                        }
                    }
                },
                "item" => {
                    if num_elements < 4 {                         
                        eprintln!("\x1b[31mcsvr: argument missing: need both row and col index.\x1b[0m");
                        return 
                    }
                    if num_elements > 4 {                         
                        eprintln!("\x1b[31mcsvr: extra arguments found: need only row and col index.\x1b[0m");
                        return
                    }
                    match query_elements[2].trim().parse::<usize>() {
                        Ok(row_ind) => {
                            match query_elements[3].trim().parse::<usize>() {
                                Ok(col_ind) => {
                                    let _ = file.delete_entry(row_ind, col_ind)
                                                .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
                                }
                                Err(err) => {
                                    eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                                }
                            }
                        }
                        Err(err) => {
                            eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                        }
                    }
                },
                _ => eprintln!("\x1b[31mcsvr: incorrect argument: need either 'row', 'col' or 'item'.\x1b[0m")
            }
        }

        "modify" | "-m" => {
            if num_elements < 2 {
                eprintln!("\x1b[31mcsvr: argument missing: need either 'row', 'col' or 'item'.\x1b[0m");
                return 
            }
            match query_elements[1].trim() {
                "row" => {
                    if num_elements < 3 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need row index.\x1b[0m");
                        return 
                    }
                    if num_elements < 4 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need new row values.\x1b[0m");
                        return 
                    }
                    if num_elements > 4 { 
                        eprintln!("\x1b[31mcsvr: extra arguments: need only row index and new row values.\x1b[0m");
                        return 
                    }
                    match query_elements[2].trim().parse::<usize>() {
                        Ok(row_ind) => {
                            let _ = file.modify_row(row_ind, query_elements[3].trim().to_owned())
                                        .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
                        }
                        Err(err) => {
                            eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                        }
                    }
                },
                "col" => {
                    if num_elements < 3 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need col index.\x1b[0m");
                        return 
                    }
                    if num_elements < 4 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need new col values.\x1b[0m");
                        return 
                    }
                    if num_elements > 4 { 
                        eprintln!("\x1b[31mcsvr: extra arguments: need only col index and new col values.\x1b[0m");
                        return 
                    }
                    match query_elements[2].trim().parse::<usize>() {
                        Ok(col_ind) => {
                            let _ = file.modify_column(col_ind, query_elements[3].trim().to_owned())
                                        .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
                        }
                        Err(err) => {
                            eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                        }
                    }
                },
                "item" => {
                    if num_elements < 3 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need row index.\x1b[0m");
                        return 
                    }
                    if num_elements < 4 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need col index.\x1b[0m");
                        return 
                    }
                    if num_elements < 5 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need new item value.\x1b[0m");
                        return 
                    }
                    if num_elements > 5 { 
                        eprintln!("\x1b[31mcsvr: extra arguments: need only row index, col index and new item value.\x1b[0m");
                        return 
                    }
                    match query_elements[2].trim().parse::<usize>() {
                        Ok(row_ind) => {
                            match query_elements[3].trim().parse::<usize>() {
                                Ok(col_ind) => {
                                    let _ = file.update_entry(
                                        row_ind, 
                                        col_ind, 
                                        query_elements[4].trim().to_owned()
                                    )
                                    .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});

                                }
                                Err(err) => {
                                    eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                                }
                            }
                        }

                        Err(err) => {
                            eprintln!("\x1b[31m{}: Not a Valid Integer Index\x1b[0m", err);
                        }
                    }
                },

                _ => eprintln!("\x1b[31mcsvr: incorrect argument: need either 'row', 'col' or 'item'.\x1b[0m")
            }
        }

        "add" | "-a" => {
            if num_elements < 2 { 
                eprintln!("\x1b[31mcsvr: argument missing: need either 'row' or 'col'.\x1b[0m");
                return 
            }
            match query_elements[1].trim() {
                "row" => {
                    if num_elements < 3 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need row values.\x1b[0m");
                        return 
                    }
                    if num_elements > 3 { 
                        eprintln!("\x1b[31mcsvr: extra arguments: need only new row values.\x1b[0m");
                        return 
                    }
                    let _ = file.add_row(query_elements[2].trim().to_owned())
                                .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});

                },
                "col" => {
                    if num_elements < 3 { 
                        eprintln!("\x1b[31mcsvr: argument missing: need col values.\x1b[0m");
                        return 
                    }
                    if num_elements > 3 { 
                        eprintln!("\x1b[31mcsvr: extra arguments: need only new col values.\x1b[0m");
                        return 
                    }
                    let _ = file.add_column(query_elements[2].trim().to_owned())
                                .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
                },
                _ => eprintln!("\x1b[31mcsvr: incorrect argument: need either 'row' or 'col'.\x1b[0m")
            }
        }

        "merge" | "-M" => {
            if num_elements < 2 { 
                eprintln!("\x1b[31mcsvr: argument missing: need second file's name.\x1b[0m");
                return 
            }
            if num_elements > 2 { 
                eprintln!("\x1b[31mcsvr: extra arguments: need only second file's name.\x1b[0m");
                return 
            }
            let other = CSVFile::new(&query_elements[1].trim().to_owned());
            let _ = file.merge_files(&other)
                        .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
        }

        "sort" | "-s" => {
            if num_elements > 1 { 
                eprintln!("\x1b[31mcsvr: extra arguments: command doesn't take any arguments.\x1b[0m");
                return 
            }
            let _ = file.sorted_display()
                        .unwrap_or_else(|err| {eprintln!("\x1b[31m{}\x1b[0m", err);});
        }

        "write" | "-w" => {
            if num_elements > 2 { 
                eprintln!("\x1b[31mcsvr: extra arguments: need only new file's name.\x1b[0m");
                return 
            }
            if num_elements == 2 {
                let mut new_file = file.clone();
                new_file.file_path = query_elements[1].trim().to_owned();
                new_file.clone().write_to_file();
            }
            else {
                file.write_to_file();
            }
        }

        "help" | "-h" => {
            if num_elements > 2 { 
                eprintln!("\x1b[31mcsvr: extra arguments: command doesn't take any arguments.\x1b[0m");
                return 
            }
            display_help();
        }

        _ => eprintln!("\x1b[31mcsvr: command not found: use 'help' or '-h' for available commands.\x1b[0m")
    }
}

fn display_help() {
    let help_string = "
            1. \x1b[36mdisplay\x1b[0m | \x1b[36m-d\x1b[0m: used to display the file or parts of the file\n
                example: \x1b[36mdisplay\x1b[0m \x1b[33mrow\x1b[0m \x1b[35m1\x1b[0m -: displays the first row\n
                         \x1b[36mdisplay\x1b[0m \x1b[33mcol\x1b[0m \x1b[35m1\x1b[0m -: displays the first col\n
                         \x1b[36mdisplay\x1b[0m \x1b[33mfile\x1b[0m -: displays the whole file\n
                         \x1b[36mdisplay\x1b[0m \x1b[33mfile\x1b[0m \x1b[35m1\x1b[0m \x1b[35m5\x1b[0m -: displays from row 1 to 5\n


            2. \x1b[36mdelete\x1b[0m | \x1b[36m-r\x1b[0m: used to delete a row, col or item in the file\n
                example: \x1b[36mdelete\x1b[0m \x1b[33mrow\x1b[0m \x1b[35m1\x1b[0m -: deletes the first row\n
                         \x1b[36mdelete\x1b[0m \x1b[33mcol\x1b[0m \x1b[35m1\x1b[0m -: deletes the first col\n
                         \x1b[36mdelete\x1b[0m \x1b[33mitem\x1b[0m \x1b[35m1\x1b[0m \x1b[35m5\x1b[0m -: deletes the item in 1st row and 5th col\n


            3. \x1b[36mmodify\x1b[0m | \x1b[36m-m\x1b[0m: used to modify a row, col or item in the file\n
                example: \x1b[36mmodify\x1b[0m \x1b[33mrow\x1b[0m \x1b[35m1\x1b[0m \x1b[35mthese,are,new,row,values\x1b[0m -: updates the first row with the values supplied\n
                         \x1b[36mmodify\x1b[0m \x1b[33mcol\x1b[0m \x1b[35m1\x1b[0m \x1b[35mthese,are,new,col,values\x1b[0m -: updates the first col with the values supplied\n
                         \x1b[36mmodify\x1b[0m \x1b[33mitem\x1b[0m \x1b[35m1\x1b[0m \x1b[35m5\x1b[0m \x1b[35mnew\x1b[0m -: updates the item in 1st row and 5th col with the value supplied\n


            4. \x1b[36madd\x1b[0m | \x1b[36m-a\x1b[0m: used to modify a row or col in the file\n
                example: \x1b[36madd\x1b[0m \x1b[33mrow\x1b[0m \x1b[35mthese,are,new,row,values\x1b[0m -: adds a row with the values supplied\n
                         \x1b[36madd\x1b[0m \x1b[33mcol\x1b[0m \x1b[35mthese,are,new,col,values\x1b[0m -: adds a col with the values supplied\n


            5. \x1b[36mmerge\x1b[0m | \x1b[36m-M\x1b[0m: used to merge a file with the one currently loaded. header of the loaded file is the new header\n
                example: \x1b[36mmerge\x1b[0m \x1b[35mfile_name.csv\x1b[0m -: merges the file 'file_name.csv' with the loaded file\n         


            6. \x1b[36msort\x1b[0m | \x1b[36m-s\x1b[0m: used to display alphabetically sorted rows of the file\n
                example: \x1b[36msort\x1b[0m -: sorts and displays the rows of the loaded file\n


            7. \x1b[36mwrite\x1b[0m | \x1b[36m-w\x1b[0m: used to write the changes to a new file or the loaded file\n
                example: \x1b[36mwrite\x1b[0m \x1b[35mfile_name.csv\x1b[0m -: writes the changes to the file 'file_name.csv'\n         
                         \x1b[36mwrite\x1b[0m -: writes the changes to the loaded file\n         
    ";

    println!("{}", help_string);
}       