use std::collections::HashMap;

#[derive(Debug)]
struct Matrix {
    matrix: HashMap<usize, Vec<usize>>,
}

impl Matrix {
    pub fn from(input: &Vec<(usize, usize)>) -> Self {
        let mut matrix = HashMap::new();
        for &(a, b) in input.iter() {
            matrix.entry(a).or_insert(Vec::new()).push(b);
            matrix.get_mut(&a).unwrap().sort();
            matrix.entry(b).or_insert(Vec::new()).push(a);
            matrix.get_mut(&b).unwrap().sort();
        }

        Matrix { matrix: matrix }
    }

    pub fn sort_stones(&self, first: usize) -> Option<Vec<(usize, usize)>> {
        if self.is_empty() {
            return None;
        }

        for i in self.matrix.get(&first).unwrap().iter() {
            let cloned_matrix = &self.remove((first, *i));

            if cloned_matrix.is_empty() {
                return Some(vec![(first, *i)]);
            }

            let res = cloned_matrix.sort_stones(*i);
            match res {
                Some(mut x) => {
                    x.insert(0, (first, *i));
                    return Some(x);
                }
                None => {
                    continue;
                }
            };
        }

        return None;
    }

    fn remove(&self, stone: (usize, usize)) -> Self {
        let mut new_matrix = self.matrix.clone();

        let (a, b) = stone;
        {
            let mut row = new_matrix.get_mut(&a).unwrap();
            let index_to_remove = row.binary_search(&b).unwrap();
            row.remove(index_to_remove);
        }

        {
            let mut row = new_matrix.get_mut(&b).unwrap();
            let index_to_remove = row.binary_search(&a).unwrap();
            row.remove(index_to_remove);
        }

        Matrix { matrix: new_matrix }
    }

    fn is_empty(&self) -> bool {
        self.matrix.iter().all(|(_, set)| set.is_empty())
    }
}

pub fn chain(input: &Vec<(usize, usize)>) -> Option<Vec<(usize, usize)>> {
    if input.is_empty() {
        return Some(vec![]);
    }

    let matrix = Matrix::from(input);
    matrix.sort_stones(input.first().unwrap().0).and_then(|res| {
        if res.first().unwrap().0 == res.last().unwrap().1 {
            Some(res)
        } else {
            None
        }
    })
}

#[cfg(test)]
mod tests {
    use super::Matrix;
    // use super::remove_from_matrix;
    // use super::prova;
    use std::collections::HashMap;

    #[test]
    fn make_matrix_correctly() {
        let mut expected = HashMap::with_capacity(3);
        expected.insert(1, vec![2]);
        expected.insert(2, vec![1, 3]);
        expected.insert(3, vec![2]);

        assert_eq!(expected, (Matrix::from(&vec![(1, 2), (2, 3)])).matrix);
    }

    #[test]
    fn remove_from_matrix_also_remove_element_if_remaining_count_is_zero() {
        let mut expected = HashMap::with_capacity(3);
        expected.insert(1, vec![]);
        expected.insert(2, vec![3]);
        expected.insert(3, vec![2]);

        let matrix = Matrix::from(&vec![(1, 2), (2, 3)]);
        let new_matrix = matrix.remove((1, 2));
        assert_eq!(expected, new_matrix.matrix);
    }
}
