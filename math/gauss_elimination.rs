const EPS: f64 = 1e-9;
#[allow(dead_code)]
fn gauss_elimination(mut matrix: Vec<Vec<f64>>, mut vect: Vec<f64>) -> Vec<f64> {
    let n = vect.len();
    let mut ret = vec![0.0; n];
    for x in 0..n {
        let mut pivot = x;
        for i in x + 1..n {
            if matrix[i][x].abs() - matrix[pivot][x].abs() > EPS {
                pivot = i;
            }
        }
        matrix.swap(x, pivot);
        vect.swap(x, pivot);
        if matrix[x][x].abs() < EPS {
            continue;
        }
        for y in x + 1..n {
            let ratio = -matrix[y][x] / matrix[x][x];
            // println!("{} {} {}", ratio, matrix[y][x], matrix[x][x]);
            matrix[y][x] = 0.0;
            for i in x + 1..n {
                matrix[y][i] += matrix[x][i] * ratio;
            }
            vect[y] += vect[x] * ratio;
        }
    }
    for x in (0..n).rev() {
        let mut sum = vect[x];
        for i in (x + 1..n).rev() {
            sum -= ret[i] * matrix[x][i];
        }
        ret[x] = sum / matrix[x][x];
    }
    ret
}

#[test]
fn test_gauss_elimination() {
    let matrix = vec![vec![1.0]];
    let vect = vec![2.0];
    let ans = gauss_elimination(matrix, vect);
    assert!((ans[0] - 2.0).abs() <= EPS);

    let mut matrix = vec![vec![1.0; 2]; 2];
    matrix[0][0] = 1.0;
    matrix[0][1] = 3.0;
    matrix[1][0] = -2.0;
    matrix[1][1] = -4.0;
    let mut vect = vec![1.0; 2];
    vect[0] = -3.0;
    vect[1] = 1.0;
    let ans = gauss_elimination(matrix, vect);
    assert!((ans[0] - 4.5).abs() <= EPS);
    assert!((ans[1] - -2.5).abs() <= EPS);

    let mut r = 1.0;
    let mut rnd = || {
        let a = 1237.23518;
        let b = 1349.18721;
        r = (a * r + b) % 100.0;
        r
    };
    for n in 2..30 {
        let mut matrix = vec![vec![1.0; n]; n];
        let mut vect = vec![1.0; n];
        for y in 0..n {
            for x in 0..n {
                matrix[y][x] = rnd();
            }
            vect[y] = rnd();
        }
        let ans = gauss_elimination(matrix.clone(), vect.clone());
        for y in 0..n {
            let mut sum = 0.0;
            for x in 0..n {
                sum += matrix[y][x] * ans[x];
            }
            assert!((sum - vect[y]).abs() <= EPS);
        }
    }
}
