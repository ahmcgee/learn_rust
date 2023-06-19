use std::ops::Index;

pub trait Transpose {
    fn transpose(&self) -> Self;
}

#[derive(PartialEq, Eq, Clone, Default, Debug)]
pub struct DynamicHeightGrid<T> {
    pub width: usize,
    data_rows: Vec<Vec<T>>
}

impl<T> DynamicHeightGrid<T> {
    pub fn new(width: usize) -> Self {
        Self {
            width: width,
            data_rows: Default::default()
        }
    }

    pub fn push(&mut self, row: Vec<T>) -> Result<(), String> {
        if row.len() != self.width {
            Err(format!("This grid supports rows of length {}, but got a row of length {}", self.width, row.len()).to_string())
        } else {
            self.data_rows.push(row);
            Ok(())
        }
    }

    pub fn len(&self) -> usize {
        self.data_rows.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Vec<T>> {
        self.data_rows.iter()
    }
}

impl<T: Copy + Default> Transpose for DynamicHeightGrid<T> {
    fn transpose(&self) -> Self {
        let height = self.data_rows.len();
        let mut new_vec = vec![vec![Default::default(); height]; self.width];

        for (i, row) in self.data_rows.iter().enumerate() {
            for (j, &elem) in row.iter().enumerate() {
                new_vec[j][i] = elem.clone();
            }
        }

        Self {
            width: height,
            data_rows: new_vec
        }
 
    }
}

impl<T> Index<usize> for DynamicHeightGrid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        return &self.data_rows[index];
    }
}

#[cfg(test)]
mod tests {
    use super::DynamicHeightGrid;
    use super::Transpose;

    #[test]
    fn dynamic_height_grid_mismatched_push_is_illegal() {
        // arrange
        let mut grid = DynamicHeightGrid::<i32>::new(5);
        // act
        let result = grid.push(vec![1, 2]);
        // assert
        match result {
            Ok(_) => {
                panic!("Expected failure");
            },
            Err(e) => {
                assert_eq!("This grid supports rows of length 5, but got a row of length 2", e);
            }
        }
    }

    #[test]
    fn dynamic_height_grid_new_len_is_zero() {
        // arrange
        let grid = DynamicHeightGrid::<i32>::new(5);
        // act
        let l = grid.len();
        // assert
        assert_eq!(0, l);
        assert_eq!(5, grid.width);
    }

    #[test]
    fn dynamic_height_grid_push_adds_to_end() {
        // arrange
        let mut grid = DynamicHeightGrid::<i32>::new(3);
        // act
        let r1 = grid.push(vec![1, 3, -5]);
        let r2 = grid.push(vec![0, 7, 9]);
        // assert
        assert_eq!(3, grid.width);
        assert_eq!(2, grid.len());
        assert_eq!(Ok(()), r1);
        assert_eq!(Ok(()), r2);
        assert_eq!(vec![1, 3, -5], grid[0]);
        assert_eq!(vec![0, 7, 9], grid[1]);
    }

    #[test]
    fn dynamic_height_grid_transpose_swaps_along_diagonal() {
        // arrange
        let mut grid = DynamicHeightGrid::<i32>::new(3);
        grid.push(vec![1, 3, -5]).unwrap();
        grid.push(vec![0, 7, 9]).unwrap();
        // act
        let transposed_grid = grid.transpose();
        // assert
        assert_eq!(3, transposed_grid.len());
        assert_eq!(2, transposed_grid.width);
        assert_eq!(transposed_grid[0], vec![1, 0]);
        assert_eq!(transposed_grid[1], vec![3, 7]);
        assert_eq!(transposed_grid[2], vec![-5, 9]);
    }

    #[test]
    fn grid_can_be_iterated() {
        // arrange
        let mut grid = DynamicHeightGrid::<i32>::new(2);
        grid.push(vec![1, 2]).unwrap();
        grid.push(vec![3, 4]).unwrap();
        // act
        // We test the iterator by using it to sum over the rows
        let result: Vec<i32> = grid.iter().map(|r| r.iter().sum()).collect();
        // assert
        assert_eq!(result, vec![3, 7]);
    }
}