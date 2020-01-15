use std::{
    error,
    fs::{read_to_string, write},
    path::Path,
    result,
};

type TResult<T> = result::Result<T, TError>;
type TError = Box<dyn error::Error>;

// Read in the file as a String
fn read_file(p: &str) -> TResult<String> {
    read_to_string(p).map_err(|e| e.into())
}
// Convert String to a Vector of Numbers
fn split_numbers(s: &String) -> TResult<Vec<usize>> {
    s.split_whitespace()
        .map(|x| x.parse::<usize>().map_err(|e| e.into()))
        .collect()
}

// Sum all of the Numbers together
fn add_numbers(v: Vec<usize>) -> usize {
    v.iter().fold(0, |mut sum, &x| {
        sum = sum + x;
        sum
    })
}

// Write the Sum back into the file
fn write_numbers(n: usize, p: &str) -> TResult<()> {
    let path = Path::new(p);
    let res = read_file(&path.display().to_string())?;

    write(path, format!("{}\n{}", res, n))?;

    Ok(())
}

fn main() -> TResult<()> {
    let path = "data/numbers.txt";
    let res = read_file(&path);

    match res {
        Ok(s) => {
            let vec = split_numbers(&s)?;
            println!("vector: {:?}", &vec);
            let sum = add_numbers(vec);
            println!("Sum: {:?}", &sum);
            write_numbers(sum, path)?;
        }
        Err(_) => {}
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    fn setup_test_write() -> TResult<()> {
        write(Path::new("test_data/test_two.txt"), String::from("4\n6"))?;
        Ok(())
    }

    #[test]
    fn test_read_file() {
        let res = read_file("test_data/test_one.txt");
        assert!(res.is_ok());

        if let Ok(s) = res {
            assert_eq!(s, "3\n5");
        }
    }

    #[test]
    fn test_split_numbers() {
        let res = split_numbers(&String::from("5\n8"));

        assert!(res.is_ok());

        if let Ok(v) = res {
            assert_eq!(v, vec![5, 8]);
        }
    }

    #[test]
    fn test_add_numers() {
        let sum = add_numbers(vec![3, 6]);
        let sum_one = add_numbers(vec![1]);
        let sum_three = add_numbers(vec![4, 7, 2]);
        let sum_none = add_numbers(vec![]);

        assert!(sum == 9);
        assert!(sum_one == 1);
        assert_eq!(sum_three, 13);
        assert_eq!(sum_none, 0);

        assert_ne!(sum_none, sum_one);
    }

    #[test]
    fn test_write_numbers() {
        let res = setup_test_write();

        assert!(res.is_ok());

        let res = write_numbers(10, "test_data/test_two.txt");
        assert!(res.is_ok());

        let res = read_file("test_data/test_two.txt");

        if let Ok(s) = res {
            assert_eq!(s, "4\n6\n10");
        }
    }
}
