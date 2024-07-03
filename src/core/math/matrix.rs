/**
 * Implements some 2D matrix operations
 * (in particular, solving systems of linear equations).
 *
 * @author Martin Davis
 *
 */

pub struct Matrix {}

impl Matrix {
    pub fn swap_rows_2d(m: &mut Vec<Vec<f64>>, i: usize, j: usize) {
        if i == j {
            return;
        };
        for col in 0..m[0].len() {
            let temp = m[i][col];
            m[i][col] = m[j][col];
            m[j][col] = temp;
        }
    }

    pub fn swap_rows_1d(m: &mut Vec<f64>, i: usize, j: usize) {
        if i == j {
            return;
        }
        let temp = m[i];
        m[i] = m[j];
        m[j] = temp;
    }

    /**
     * Solves a system of equations using Gaussian Elimination.
     * In order to avoid overhead the algorithm runs in-place
     * on A - if A should not be modified the client must supply a copy.
     *
     * @param a an nxn matrix in row/column order )modified by this method)
     * @param b a vector of length n
     *
     * @return a vector containing the solution (if any)
     * or null if the system has no or no unique solution
     *
     * @throws IllegalArgumentException if the matrix is the wrong size
     */
    pub fn solve(a: &mut Vec<Vec<f64>>, b: &mut Vec<f64>) -> Option<Vec<f64>> {
        let n = b.len();
        if a.len() != n || a[0].len() != n {
            return None;
        }

        // Use Gaussian Elimination with partial pivoting.
        // Iterate over each row
        for i in 0..n {
            // Find the largest pivot in the rows below the current one.
            let mut max_element_row = i;
            let mut j = i + 1;
            while j < n {
                if f64::abs(a[j][i]) > f64::abs(a[max_element_row][i]) {
                    max_element_row = j;
                }
                j += 1;
            }

            if a[max_element_row][i] == 0.0 {
                return None;
            }

            // Exchange current row and maxElementRow in A and b.
            Matrix::swap_rows_2d(a, i, max_element_row);
            Matrix::swap_rows_1d(b, i, max_element_row);

            // Eliminate using row i
            let mut j = i + 1;
            while j < n {
                let row_factor = a[j][i] / a[i][i];
                let mut k = n - 1;
                while k >= i {
                    a[j][k] -= a[i][k] * row_factor;
                    k -= 1;
                }
                b[j] -= b[i] * row_factor;
                j += 1;
            }
        }

        // A is now (virtually) in upper-triangular form.
        // The solution vector is determined by back-substitution.
        let mut solution: Vec<f64> = vec![f64::NAN; n];
        let mut j = n - 1;
        #[allow(unused_comparisons)]
        while j >= 0 {
            let mut t = 0.0;
            let mut k = j + 1;
            while k < n {
                t += a[j][k] * solution[k];
                k += 1;
            }
            solution[j] = (b[j] - t) / a[j][j];
            j -= 1;
        }
        return Some(solution);
    }
}
