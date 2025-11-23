pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let mut row_max_vector: Vec<(usize, usize, u64)> = Vec::new();
    let mut column_min_vector: Vec<(usize, usize, u64)> = Vec::new();
    let mut row_counter: usize = 0;
    let mut column_counter: usize = 0;
    while row_counter < input.len() {
        let mut row_max = <u64>::min_value();
        while column_counter < input[row_counter].len() {
            if row_max < input[row_counter][column_counter] {
                row_max = input[row_counter][column_counter];
            };
            column_counter += 1;
        }
        row_max_vector.push((row_counter, column_counter, row_max));
        column_counter = 0;
        row_counter += 1;
    }
    row_counter = 0;
    column_counter = 0;
    while column_counter < input[row_counter].len() {
        let mut column_min = <u64>::max_value();
        while row_counter < input.len() {
            if column_min > input[row_counter][column_counter] {
                column_min = input[row_counter][column_counter];
            };
            row_counter += 1;
        }
        column_min_vector.push((row_counter, column_counter, column_min));
        row_counter = 0;
        column_counter += 1;
    }
    if row_max_vector.len() > column_min_vector.len() {
        for i in row_max_vector.clone() {
            for j in column_min_vector.clone() {
                if i.2 == j.2 {
                    result.push((i.0, j.1));
                }
            }
        }
    } else {
        for i in column_min_vector.clone() {
            for j in row_max_vector.clone() {
                if i.2 == j.2 {
                    result.push((j.0, i.1));
                }
            }
        }
    }

    result
}
