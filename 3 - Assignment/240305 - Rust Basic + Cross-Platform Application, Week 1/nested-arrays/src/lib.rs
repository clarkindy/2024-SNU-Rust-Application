pub fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let m = matrix;
    [
        [m[0][0], m[1][0], m[2][0]],
        [m[0][1], m[1][1], m[2][1]],
        [m[0][2], m[1][2], m[2][2]],
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn transpose_3x3_unit() {
        #[rustfmt::skip]
        let matrix = [
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ];

        let ret = transpose(matrix);
        #[rustfmt::skip]
        assert_eq!(ret, [
            [1, 0, 0],
            [0, 1, 0],
            [0, 0, 1],
        ]);
    }

    #[test]
    fn transpose_3x3_random() {
        #[rustfmt::skip]
        let matrix = [
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 9],
        ];

        let ret = transpose(matrix);
        #[rustfmt::skip]
        assert_eq!(ret, [
            [1, 4, 7],
            [2, 5, 8],
            [3, 6, 9],
        ]);
    }
}
