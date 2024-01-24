use std::{ fs, io, io::Write, process, fmt::{ Display, Formatter }, path::Path };
use std::error::Error;

pub trait FileDataUtils {
    // display file x y
    fn display_file(&self, _start_index: Option<usize>, _end_index: Option<usize>) -> Result<(), Box<dyn Error>> { Ok(()) }

    // delete row x
    fn delete_row(&mut self, _row_index: usize) -> Result<(), Box<dyn Error>> { Ok(()) }

    // delete column x
    fn delete_column(&mut self, _col_index: usize) -> Result<(), Box<dyn Error>> { Ok(()) }

    // delete entry x y
    fn delete_entry(&mut self, _row_index: usize, _col_index: usize) -> Result<(), Box<dyn Error>> { Ok(()) }

    // modifiy row x string
    fn modify_row(&mut self, _row_index: usize, _new_row_data: String) -> Result<(), Box<dyn Error>> { Ok(()) }

    // modify column x string
    fn modify_column(&mut self, _col_index: usize, _new_data: String) -> Result<(), Box<dyn Error>> { Ok(()) }

    // modify entry x y string
    fn update_entry(&mut self, _row_index: usize, _col_index: usize, _new_value: String) -> Result<(), Box<dyn Error>> { Ok(()) }

    // merge fileName
    fn merge_files(&mut self, _other: &Self) -> Result<(), Box<dyn Error>> { Ok(()) }

    // sort rows
    fn sorted_display(&self) -> Result<(), Box<dyn Error>> { Ok(()) }

    // add row string
    fn add_row(&mut self, _new_row_data: String) -> Result<(), Box<dyn Error>> { Ok(()) }

    // add column string
    fn add_column(&mut self, _new_col_data: String) -> Result<(), Box<dyn Error>> { Ok(()) }

    // display column x
    fn display_column(&self, _col_index: usize) -> Result<(), Box<dyn Error>> { Ok(()) }

    // display row x
    fn display_row(&self, _row_index: usize) -> Result<(), Box<dyn Error>> { Ok(()) }
}

#[derive(Default, Debug, Clone)]
pub struct CSVFile {
    pub file_data: Vec<String>,
    pub header: String,
    pub num_rows: usize,
    pub num_cols: usize,
    pub file_path: String,
    pub max_col_lengths: Vec<usize>
}

impl PartialEq for CSVFile {
    fn eq(&self, other: &Self) -> bool {
        self.file_data == other.file_data && 
        self.header == other.header && 
        self.num_rows == other.num_rows && 
        self.num_cols == other.num_cols && 
        self.file_path == other.file_path && 
        self.max_col_lengths == other.max_col_lengths
    }
}

impl From<CSVError> for String {
    fn from(value: CSVError) -> Self {
        match value {
            CSVError::IncorrectIndices => "Start index greater than end index".to_owned(),
            CSVError::InvalidRowIndex => "Row with entered index doesn't exist".to_owned(),
            CSVError::InvalidColIndex => "Col with entered index doesn't exist".to_owned(),
            CSVError::InvalidEntry => "No item exists at supplied position".to_owned(),
            CSVError::MoreEntriesThanRequired => "More entries supplied than required".to_owned(),
            CSVError::LessEntriesThanRequired => "Less entries supplied than required. Use '_' for lack of entry".to_owned(),
            CSVError::InvalidDimensions => "Dimensions of specified files do not match. Cannot merge".to_owned(),
        }
    }
}
#[derive(Debug, PartialEq)]
pub enum CSVError { 
    IncorrectIndices,
    InvalidRowIndex,
    InvalidColIndex,
    InvalidEntry,
    MoreEntriesThanRequired,
    LessEntriesThanRequired,
    InvalidDimensions
}

impl std::error::Error for CSVError {}
impl Display for CSVError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CSVError::IncorrectIndices => write!(f, "Start index greater than end index"),
            CSVError::InvalidRowIndex => write!(f, "Row with entered index doesn't exist"),
            CSVError::InvalidColIndex => write!(f, "Col with entered index doesn't exist"),
            CSVError::InvalidEntry => write!(f, "No item exists at supplied position"),
            CSVError::MoreEntriesThanRequired => write!(f, "More entries supplied than required"),
            CSVError::LessEntriesThanRequired => write!(f, "Less entries supplied than required. Use '_' for lack of entry"),
            CSVError::InvalidDimensions => write!(f, "Dimensions of specified files do not match. Cannot merge"),
        }
    }
}

impl FileDataUtils for CSVFile {
    /// Displays a portion of the CSV file, including headers and specified rows.
    ///
    /// # Arguments
    ///
    /// * `start_index` - The optional starting index for the displayed rows. Default is 1.
    /// * `end_index` - The optional ending index for the displayed rows. Default is the total number of rows.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided indices are invalid.
    /// 
    fn display_file(&self, start_index: Option<usize>, end_index: Option<usize>) -> Result<(), Box<dyn Error>> {
        let (s_index, e_index) = (start_index.unwrap_or_else(|| 1), end_index.unwrap_or_else(|| self.file_data.len()));
        if s_index > self.num_rows || e_index > self.num_rows || s_index == 0 || e_index == 0 { return Err(Box::new(CSVError::InvalidRowIndex)) }
        if s_index > e_index { return Err(Box::new(CSVError::IncorrectIndices)) }
        // Display first horizontal line
        self.display_horizontal_line();

        // Display headers
        print!("|");
        for (header, max_length) in self.header.split(",").collect::<Vec<&str>>().iter().zip(&self.max_col_lengths) {
            print!(" \x1b[36m{:<width$} \x1b[0m|", header, width = max_length);
        }
        println!();
        
        // Display second horizontal line
        self.display_horizontal_line();

        // Display rows
        for (index, line) in self.file_data[s_index-1..e_index].iter().enumerate() {
            let row_items: Vec<&str> = line.split(',').collect();
            print!("|");
            for (item, max_length) in row_items.iter().zip(&self.max_col_lengths) {
                print!(" {:<width$} |", item, width = max_length);
            }
            println!("  ({})", index + 1);

            // Display horizontal lines for rows
            self.display_horizontal_line();
        }
        return Ok(());
    }

    /// Deletes the specified row from the CSV file.
    ///
    /// # Arguments
    ///
    /// * `row_index` - The index of the row to be deleted.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided row index is invalid.
    /// 
    fn delete_row(&mut self, row_index: usize) -> Result<(), Box<dyn Error>> { 
        if row_index > self.num_rows || row_index == 0 { return Err(Box::new(CSVError::InvalidRowIndex)) }
        println!(">>>  \x1b[35mAre you sure you want to delete this row #{}\x1b[0m", row_index);
        self.display_file(Some(row_index), Some(row_index))?;
        print!(">>  \x1b[35m[y/n]:\x1b[0m ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        if input != "y\n".to_owned() { 
            println!(">>>  \x1b[33mDid Not Delete Row\x1b[0m");
            return Ok(()) 
        }

        let row = self.file_data[row_index - 1].clone();
        self.file_data.remove(row_index-1);
        self.num_rows -= 1;    
        println!(">>>  \x1b[32mSuccessfully Deleted Row: {}\x1b[0m", row);

        Ok(())
    }

    /// Deletes the specified column from the CSV file.
    ///
    /// # Arguments
    ///
    /// * `col_index` - The index of the column to be deleted.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided column index is invalid.
    /// 
    fn delete_column(&mut self, col_index: usize) -> Result<(), Box<dyn Error>> { 
        if col_index > self.num_cols || col_index == 0 { return Err(Box::new(CSVError::InvalidColIndex)) }

        println!(">>>  \x1b[35mAre you sure you want to delete the column:\x1b[0m");
        self.display_column(col_index)?;

        print!(">>  \x1b[35m[y/n]:\x1b[0m ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        if input != "y\n".to_owned() { 
            println!(">>>  \x1b[33mDid Not Delete Column\x1b[0m");
            return Ok(()) 
        }

        let mut header_vec: Vec<&str> = self.header.split(",").collect();
        header_vec.remove(col_index - 1);
        self.header = header_vec.join(",");

        for row_ind in 0..self.num_rows {
            let row = self.file_data[row_ind].clone();
            let mut row_vec: Vec<&str> = row.split(",").collect();
            row_vec.remove(col_index - 1);
            self.file_data[row_ind] = row_vec.join(",");
        }

        self.num_cols -= 1;
        println!(">>>  \x1b[32mSuccessfully Deleted Column\x1b[0m");
        Ok(())
    }

    /// Deletes the specified entry at the intersection of the provided row and column indices.
    ///
    /// # Arguments
    ///
    /// * `row_index` - The index of the row containing the entry to be deleted.
    /// * `col_index` - The index of the column containing the entry to be deleted.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided row or column index is invalid, or if the existing entry is NULL(_).
    /// 
    fn delete_entry(&mut self, row_index: usize, col_index: usize) -> Result<(), Box<dyn Error>> {
        if row_index > self.num_rows || row_index == 0 { return Err(Box::new(CSVError::InvalidRowIndex)) }
        if col_index > self.num_cols || col_index == 0 { return Err(Box::new(CSVError::InvalidColIndex)) }
        let row = self.file_data[row_index - 1].clone();
        let mut row_vec = row.split(",").collect::<Vec<&str>>();
        let entry = row_vec.remove(col_index - 1);
        if entry == "_" { return Err(Box::new(CSVError::InvalidEntry));}

        println!(">>>  \x1b[35mAre you sure you want to delete the entry: {}\x1b[0m", entry);
        print!(">>  \x1b[35m[y/n]:\x1b[0m ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        if input != "y\n".to_owned() { 
            println!(">>>  \x1b[33mDid Not Delete Entry\x1b[0m");
            return Ok(()) 
        }

        row_vec.insert(col_index - 1, "_");
        self.file_data[row_index - 1] = row_vec.join(",");     
        println!(">>>  \x1b[32mSuccessfully Deleted Entry: {}\x1b[0m", entry);

        Ok(())
    }

    /// Modifies the specified row with the provided row data.
    ///
    /// # Arguments
    ///
    /// * `row_index` - The index of the row to be modified.
    /// * `row_data` - A string containing the new data for the specified row, separated by commas.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided row index is invalid, or if the row data has an incorrect number of entries.
    /// 
    fn modify_row(&mut self, row_index: usize, row_data: String) -> Result<(), Box<dyn Error>> { 
        if row_index > self.num_rows || row_index == 0 { return Err(Box::new(CSVError::InvalidRowIndex)) }
        let row_entries_count = row_data.split(",").count();
        if row_entries_count > self.num_cols { return Err(Box::new(CSVError::MoreEntriesThanRequired)) }
        if row_entries_count < self.num_cols { return Err(Box::new(CSVError::LessEntriesThanRequired)) }

        println!(">>>  \x1b[35mAre you sure you want to modify this row: #{}\x1b[0m", row_index);
        self.display_file(Some(row_index), Some(row_index))?;
        print!(">>  \x1b[35m[y/n]:\x1b[0m ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        if input != "y\n".to_owned() { 
            println!(">>>  \x1b[33mDid Not Modify Row\x1b[0m");
            return Ok(()) 
        }
        self.file_data[row_index - 1] = row_data;      
        println!(">>>  \x1b[32mSuccessfully Modified Row #{} Into:\x1b[0m ", row_index);
        self.display_file(Some(row_index), Some(row_index))?;

        Ok(())
    }

    /// Modifies the specified column with the provided new entry values.
    ///
    /// # Arguments
    ///
    /// * `col_index` - The index of the column to be modified.
    /// * `new_entry` - A string containing the new values for the specified column, separated by commas.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided column index is invalid, or if the new entry has an incorrect number of values.
    /// 
    fn modify_column(&mut self, col_index: usize, new_entry: String) -> Result<(), Box<dyn Error>> { 
        if col_index > self.num_cols || col_index == 0 { return Err(Box::new(CSVError::InvalidColIndex)) }
        let new_col_values: Vec<&str> = new_entry.split(",").collect();
        let new_col_values_len = new_col_values.len();
        if new_col_values_len > self.num_rows { return Err(Box::new(CSVError::MoreEntriesThanRequired)) }
        if new_col_values_len < self.num_rows { return Err(Box::new(CSVError::LessEntriesThanRequired)) }

        println!(">>>  \x1b[35mAre you sure you want to modify the column #{}:\x1b[0m", col_index);
        self.display_column(col_index)?;

        print!(">>  \x1b[35m[y/n]:\x1b[0m ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        if input != "y\n".to_owned() { 
            println!(">>>  \x1b[33mDid Not Modify Column\x1b[0m");
            return Ok(()) 
        }

        for row_ind in 0..self.num_rows {
            let row = self.file_data[row_ind].clone();
            let mut row_vec: Vec<&str> = row.split(",").collect();
            row_vec[col_index - 1] = new_col_values[row_ind];
            self.file_data[row_ind] = row_vec.join(",");
        }
  
        println!(">>>  \x1b[32mSuccessfully Modified Column #{}:\x1b[0m", col_index);
        self.display_column(col_index)?;

        Ok(())
    }

    /// Updates the specified entry in the CSV file with a new value.
    ///
    /// # Arguments
    ///
    /// * `row_index` - The index of the row containing the entry to be updated.
    /// * `col_index` - The index of the column containing the entry to be updated.
    /// * `new_entry` - A string representing the new value for the specified entry.
    ///
    /// # Errors
    ///
    /// Returns an error if the provided row or column index is invalid.
    /// 
    fn update_entry(&mut self, row_index: usize, col_index: usize, new_entry: String) -> Result<(), Box<dyn Error>> { 
        if row_index > self.num_rows || row_index == 0 { return Err(Box::new(CSVError::InvalidRowIndex)) }
        if col_index > self.num_cols || col_index == 0 { return Err(Box::new(CSVError::InvalidColIndex)) }
        let row = self.file_data[row_index - 1].clone();
        let mut row_vec = row.split(",").collect::<Vec<&str>>();
        let entry = row_vec.remove(col_index - 1);
        
        println!(">>>  \x1b[35mAre you sure you want to update the entry: {}\x1b[0m", entry);
        print!(">>  \x1b[35m[y/n]:\x1b[0m ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        if input != "y\n".to_owned() { 
            println!(">>>  \x1b[33mDid Not Update Entry\x1b[0m");
            return Ok(()) 
        }

        row_vec.insert(col_index - 1, &new_entry);
        self.file_data[row_index - 1] = row_vec.join(",");
        println!(">>>  \x1b[32mSuccessfully Updated Entry: {}, With New Entry: {}\x1b[0m", entry, new_entry);

        Ok(())
    }

    /// Displays the CSV file with rows sorted in ascending order.
    ///
    /// This function creates a temporary CSV structure, sorts its rows, and displays the result.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue with displaying the sorted file.
    /// 
    fn sorted_display(&self) -> Result<(), Box<dyn Error>> { 
        let mut temp_csv_struct = self.clone();
        temp_csv_struct.file_data.sort();
        println!(">>>  \x1b[32mSorted Rows:\x1b[0m");
        temp_csv_struct.display_file(None, None)?;
        Ok(())
    }

    /// Merges the content of another CSV file into the current loaded CSV file.
    ///
    /// This function appends the rows from the specified CSV file (`other`) to the current CSV file.
    /// The two files must have the same number of columns for a successful merge.
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another CSV file to be merged into the current file.
    ///
    /// # Errors
    ///
    /// Returns an error if the two files have incompatible dimensions or if there is any issue with the merge.
    /// 
    fn merge_files(&mut self, other: &Self) -> Result<(), Box<dyn Error>> { 
        if self.num_cols != other.num_cols { return Err(Box::new(CSVError::InvalidDimensions)) }
        for row in other.file_data.iter() {
            self.file_data.push(row.clone());
        }
        self.num_rows += other.num_rows;
        println!(">>>  \x1b[32mSuccessfully Merged Files\x1b[0m");
        Ok(()) 
    }

    /// Adds a new row to the CSV file.
    ///
    /// This function appends a new row with the specified data to the end of the CSV file.
    ///
    /// # Arguments
    ///
    /// * `row_data` - A string containing the data for the new row, with values separated by commas.
    ///
    /// # Errors
    ///
    /// Returns an error if the number of entries in the provided row data is not equal to the total 
    /// number of columns in the CSV file.
    /// 
    fn add_row(&mut self, row_data: String) -> Result<(), Box<dyn Error>> {
        let row_entries_count = row_data.split(',').count();
        if row_entries_count > self.num_cols { return Err(Box::new(CSVError::MoreEntriesThanRequired)) }
        if row_entries_count < self.num_cols { return Err(Box::new(CSVError::LessEntriesThanRequired)) }

        println!(">>>  \x1b[35mAre you sure you want to add this row: {}\x1b[0m", row_data);
        print!(">>  \x1b[35m[y/n]:\x1b[0m ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        if input != "y\n".to_owned() { 
            println!(">>>  \x1b[33mDid Not Add Row\x1b[0m");
            return Ok(()) 
        }
        self.file_data.push(row_data.clone());
    
        println!(">>>  \x1b[32mSuccessfully Added Row: {}\x1b[0m", row_data);
        Ok(()) 
    }

    /// Adds a new column to the CSV file with the provided entries.
    ///
    /// # Arguments
    ///
    /// * `new_entry` - A string containing comma-separated values for the new column.
    ///
    /// # Errors
    ///
    /// Returns an error if the number of entries in `new_entry` is not equal to the
    /// (total number of rows + 1) in the CSV file.
    /// 
    fn add_column(&mut self, new_entry: String) -> Result<(), Box<dyn Error>> { 
        let new_col_values: Vec<&str> = new_entry.split(",").collect();
        let new_col_values_len = new_col_values.len();
        if new_col_values_len > self.num_rows + 1 { return Err(Box::new(CSVError::MoreEntriesThanRequired)) }
        if new_col_values_len < self.num_rows + 1{ return Err(Box::new(CSVError::LessEntriesThanRequired)) }

        println!(">>>  \x1b[35mAre you sure you want to add the column\x1b[0m");

        print!(">> \x1b[35m[y/n]:\x1b[0m ");
        let _ = io::stdout().flush();
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        if input != "y\n".to_owned() { 
            println!(">>>  \x1b[33mDid Not Add Column\x1b[0m");
            return Ok(()) 
        }

        self.header.push(',');
        self.header += new_col_values[0];
        for row_ind in 0..self.num_rows {
            self.file_data[row_ind].push(',');
            self.file_data[row_ind] += new_col_values[row_ind + 1];
        }

        println!(">>>  \x1b[32mSuccessfully Added Column: {}\x1b[0m", new_entry);
        self.num_cols += 1;
        Ok(())
    }

    /// Displays the specified column in the CSV file.
    ///
    /// # Arguments
    ///
    /// * `col_index` - The index of the column to be displayed.
    ///
    /// # Errors
    ///
    /// Returns an error if the column index is invalid (greater than the total number of columns
    /// or less than 1).
    /// 
    fn display_column(&self, col_index: usize) -> Result<(), Box<dyn Error>> {
        if col_index > self.num_cols || col_index == 0 { return Err(Box::new(CSVError::InvalidColIndex)) }

        let max_col_length = self.max_col_lengths[col_index - 1];
        print!("+");
        print!("{:-<width$}+", "", width = max_col_length + 2);
        println!();

        print!("|");
        let header = self.header.split(",").collect::<Vec<&str>>()[col_index - 1];
        print!(" \x1b[36m{:<width$} \x1b[0m|", header, width = max_col_length);
        println!();

        print!("+");
        print!("{:-<width$}+", "", width = max_col_length + 2);  // +2 for padding
        println!();

        for row_ind in 0..self.num_rows {
            let row_items: Vec<&str> = self.file_data[row_ind].split(',').collect();
            print!("|");
            print!(" {:<width$} |  ({})", row_items[col_index - 1], row_ind + 1, width = max_col_length);
            println!();

            print!("+");
            print!("{:-<width$}+", "", width = max_col_length + 2);  // +2 for padding
            println!();
        }
        Ok(())
    }

    /// Displays the specified row in the CSV file.
    ///
    /// # Arguments
    ///
    /// * `row_index` - The index of the row to be displayed.
    ///
    /// # Errors
    ///
    /// Returns an error if the row index is invalid (greater than the total number of rows
    /// or less than 1).
    ///
    fn display_row(&self, row_index: usize) -> Result<(), Box<dyn Error>> {
        self.display_file(Some(row_index), Some(row_index))?;
        Ok(())
    }
}

impl CSVFile {
    fn display_horizontal_line(&self) {
        print!("+");
        for max_length in &self.max_col_lengths {
            print!("{:-<width$}+", "", width = max_length + 2);
        }
        println!();
    }

    pub fn new(file_path: &String) -> Self {
        let path = Path::new(&file_path);
        if let Some(ext) = path.extension() {
            if ext != "csv" { 
                eprintln!("\x1b[31mcsvr: error loading file: supplied file is not a csv file\x1b[0m");
                process::exit(1);
            }
        }
        
        if !path.exists() {
            eprintln!("\x1b[31mcsvr: error loading file: file doesn't exist\x1b[0m");
            process::exit(1);
        }
        
        let file_read_result = fs::read_to_string(file_path);
        let file_data: Vec<String> = file_read_result
                            .unwrap()
                            .split("\n")
                            .into_iter()
                            .map(|row| row.to_owned())
                            .collect();

        if file_data[0] == "" { 
            eprintln!("\x1b[31mcsvr: error loading file: file is empty\x1b[0m");
            process::exit(1);
        }
        let header = file_data[0].clone();
        let num_rows = file_data.len() - 1;  // row - 1 because we are not including the header
        let num_cols = file_data[0].split(",").collect::<Vec<&str>>().len();
        let file_path = file_path.clone();

        let headers: Vec<&str> = header.split(',').collect();
        let max_col_lengths: Vec<usize> = (0..headers.len())
            .map(|col| {
                    file_data.iter()
                    .map(|row| row.split(',').nth(col).unwrap_or("").len())
                    .chain(std::iter::once(headers[col].len()))
                    .max()
                    .unwrap_or(0)
            })
            .collect();

        CSVFile {
            file_data: file_data[1..].to_vec(),
            header,
            num_rows,
            num_cols,
            file_path,
            max_col_lengths
        }
    }

    pub fn write_to_file(&self) {
        if self.num_cols == 0 { eprintln!("\x1b[31mcsvr: no data found to write to file\x1b[0m"); }
        let file_path_split: Vec<&str> = self.file_path.split('.').collect();
        if file_path_split[file_path_split.len() - 1] != "csv" { 
            eprintln!("\x1b[31mcsvr: invalid file extension: must be '.csv'\x1b[0m"); 
            return; 
        } 
        let mut new_file_content: String = String::new();
        new_file_content.push_str(&self.header);
        for row in self.file_data.iter() {
            let row_to_push = "\n".to_owned() + row;
            new_file_content.push_str(&row_to_push);
        }
        let _ = fs::write(&self.file_path, new_file_content);  
    }
}