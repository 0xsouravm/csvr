#[cfg(test)]
mod tests {
    use csvr::{FileDataUtils, CSVError};

    #[test]
    fn test_display_file_incorrect_row_index() {
        let default_file: csvr::CSVFile = Default::default();
        let result_1 = default_file.display_file(Some(100), None);
        let result_2 = default_file.display_file(None, Some(100));
        if let (Err(error_1), Err(error_2)) = (result_1, result_2) {
            let err_1 = error_1.downcast_ref::<CSVError>().unwrap();
            let err_2 = error_2.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err_1, &CSVError::InvalidRowIndex);
            assert_eq!(err_2, &CSVError::InvalidRowIndex);
        }
    }

    #[test]
    fn test_display_file_incorrect_indices() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.file_data.push("1,2,3,4".to_owned());
        default_file.file_data.push("1,2,3,4".to_owned());
        default_file.num_rows = 2;
        let result = default_file.display_file(Some(2), Some(1));
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::IncorrectIndices);
        }
    }

    #[test]
    fn test_delete_row_invalid_row_index() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_rows = 2;
        let result = default_file.delete_row(20);
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::InvalidRowIndex);
        }
    }

    #[test]
    fn test_delete_col_invalid_col_index() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_cols = 2;
        let result = default_file.delete_column(20);
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::InvalidColIndex);
        }
    }

    #[test]
    fn test_delete_entry_invalid_row_index() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_rows = 2;
        let result = default_file.delete_entry(20, 0);
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::InvalidRowIndex);
        }
    }

    #[test]
    fn test_delete_entry_invalid_col_index() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_cols = 2;
        default_file.num_rows = 1;
        let result = default_file.delete_entry(1, 20);
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::InvalidColIndex);
        }
    }

    #[test]
    fn test_delete_entry_invalid_entry() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_cols = 1;
        default_file.num_rows = 1;
        default_file.file_data.push("_".to_owned());
        let result = default_file.delete_entry(1, 1);
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::InvalidEntry);
        }
    }

    #[test]
    fn test_modify_row_invalid_row_index() {
        let mut default_file: csvr::CSVFile = Default::default();
        let result = default_file.modify_row(20, "new".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::InvalidRowIndex);
        }
    }

    #[test]
    fn test_modify_row_more_entries() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_rows = 1;
        let result = default_file.modify_row(1, "new".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::MoreEntriesThanRequired);
        }
    }

    #[test]
    fn test_modify_row_less_entries() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_rows = 1;
        default_file.num_cols = 2;
        let result = default_file.modify_row(1, "new".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::LessEntriesThanRequired);
        }
    }

    #[test]
    fn test_modify_col_invalid_col_index() {
        let mut default_file: csvr::CSVFile = Default::default();
        let result = default_file.modify_column(20, "new".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::InvalidColIndex);
        }
    }

    #[test]
    fn test_modify_col_more_entries() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_cols = 1;
        let result = default_file.modify_column(1, "a".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::MoreEntriesThanRequired);
        }
    }

    #[test]
    fn test_modify_col_less_entries() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_rows = 2;
        default_file.num_cols = 1;
        let result = default_file.modify_column(1, "new".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::LessEntriesThanRequired);
        }
    }

    #[test]
    fn test_modify_entry_invalid_row_index() {
        let mut default_file: csvr::CSVFile = Default::default();
        let result = default_file.update_entry(1, 1, "new".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::InvalidRowIndex);
        }
    }

    #[test]
    fn test_modify_entry_invalid_col_index() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_rows = 1;
        let result = default_file.update_entry(1, 1, "new".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::InvalidColIndex);
        }
    }

    #[test]
    fn test_merge_files_invalid_dimensions() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_cols = 1;
        let other_file: csvr::CSVFile = Default::default();
        let result = default_file.merge_files(&other_file);
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::InvalidDimensions);
        }
    }

    #[test]
    fn test_add_row_less_entries() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_cols = 2;

        let result = default_file.add_row("a".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::LessEntriesThanRequired);
        }
    }

    #[test]
    fn test_add_row_more_entries() {
        let mut default_file: csvr::CSVFile = Default::default();
        let result = default_file.add_row("".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::MoreEntriesThanRequired);
        }
    }

    #[test]
    fn test_add_col_less_entries() {
        let mut default_file: csvr::CSVFile = Default::default();
        default_file.num_rows = 1;

        let result = default_file.add_column("".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::LessEntriesThanRequired);
        }
    }

    #[test]
    fn test_add_col_more_entries() {
        let mut default_file: csvr::CSVFile = Default::default();
        let result = default_file.add_column("a,b".to_owned());
        if let Err(error) = result {
            let err = error.downcast_ref::<CSVError>().unwrap();
            assert_eq!(err, &CSVError::MoreEntriesThanRequired);
        }
    }
}


// Red: \x1b[31m
// Bold: \x1b[1m
// Cyan: \x1b[36m
// Regular: \x1b[0m
// Green: \x1b[32m
// Yellow: \x1b[33m
// Blue: \x1b[34m
// Magenta: \x1b[35m