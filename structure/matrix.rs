type Weight = i64;
#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Vector {
    pub vect: Vec<Weight>,
}
#[allow(dead_code)]
impl Vector {
    pub fn new(v: Vec<Weight>) -> Vector {
        Vector { vect: v }
    }
    pub fn new_3d(x: Weight, y: Weight, z: Weight) -> Vector {
        Vector {
            vect: vec![x, y, z],
        }
    }
    pub fn zero(dim: usize) -> Vector {
        Vector {
            vect: vec![0 as Weight; dim],
        }
    }
    pub fn len(&self) -> usize {
        self.vect.len()
    }
    pub fn x(&self) -> Weight {
        self.vect[0]
    }
    pub fn y(&self) -> Weight {
        assert!(self.len() > 1);
        self.vect[1]
    }
    pub fn z(&self) -> Weight {
        assert!(self.len() > 2);
        self.vect[2]
    }
    pub fn w(&self) -> Weight {
        assert!(self.len() > 3);
        self.vect[3]
    }
    pub fn dot(&self, rhs: &Vector) -> Weight {
        assert!(self.len() == rhs.len());
        let mut ret = 0 as Weight;
        for i in 0..self.len() {
            ret += self.vect[i] * rhs.vect[i];
        }
        return ret;
    }
    pub fn cross(&self, rhs: &Vector) -> Vector {
        assert!(self.len() == 3);
        assert!(rhs.len() == 3);
        let mut ret = Vector::zero(3);
        ret.vect[0] = self.y() * rhs.z() - self.z() * rhs.y();
        ret.vect[1] = self.z() * rhs.x() - self.x() * rhs.z();
        ret.vect[2] = self.x() * rhs.y() - self.y() * rhs.x();
        return ret;
    }
    pub fn norm(&self) -> Weight {
        self.dot(self)
    }
    pub fn abs(&self) -> f64 {
        (self.norm() as f64).sqrt()
    }
    // Wegith‚ªf64‚Å‚È‚¢‚Æ“®‚©‚È‚¢
    // pub fn normalize(&self) -> &Vector {
    //     self /= self.abs();
    //     return self;
    // }
}
// #[allow(dead_code)]
// pub fn dot(lhs: &Vector, rhs: &Vector) -> Weight {
//     return lhs.dot(rhs);
// }
// #[allow(dead_code)]
// pub fn cross(lhs: &Vector, rhs: &Vector) -> Vector {
//     return lhs.cross(rhs);
// }

impl std::ops::Index<usize> for Vector {
    type Output = Weight;
    #[inline]
    fn index(&self, rhs: usize) -> &Weight {
        &self.vect[rhs]
    }
}
impl std::ops::IndexMut<usize> for Vector {
    #[inline]
    fn index_mut(&mut self, rhs: usize) -> &mut Weight {
        &mut self.vect[rhs]
    }
}
impl std::ops::Neg for Vector {
    type Output = Vector;
    #[inline]
    fn neg(self) -> Vector {
        return self * (-1 as Weight);
    }
}

macro_rules! vector_vector_ops {
    ( $trate:ident, $fname:ident, $op:tt) => {
impl<'a> std::ops::$trate<&'a Vector> for Vector {
    type Output = Vector;
    #[inline]
    fn $fname(self, rhs: &'a Vector) -> Vector {
        assert!(self.len() == rhs.len());
        let mut ret = self;
        for i in 0..ret.len() {
            ret[i] = ret[i] $op rhs[i];
        }
        return ret;
    }
        }
    };
}
macro_rules! vector_scalar_ops {
    ( $trate:ident, $fname:ident, $op:tt) => {
impl std::ops::$trate<Weight> for Vector {
    type Output = Vector;
    #[inline]
    fn $fname(self, rhs: Weight) -> Vector {
        let mut ret = self;
        for i in 0..ret.len() {
            ret[i] = ret[i] $op rhs;
        }
        return ret;
    }
}
    };
}
macro_rules! self_self_assign_ops {
    ( $type:ty, $trate:ident, $fname:ident, $op:tt) => {
        impl<'a> std::ops::$trate<&'a $type> for $type {
            #[inline]
            fn $fname(&mut self, rhs: &'a $type) {
                *self = self.clone() $op rhs;
            }
        }
    };
}
macro_rules! self_scalar_assign_ops {
    ( $type:ty, $trate:ident, $fname:ident, $op:tt) => {
        impl std::ops::$trate<Weight> for $type {
            #[inline]
            fn $fname(&mut self, rhs: Weight) {
                *self = self.clone() $op rhs;
            }
        }
    };
}
vector_vector_ops!(Add, add, +);
vector_vector_ops!(Sub, sub, -);
vector_scalar_ops!(Add, add, +);
vector_scalar_ops!(Sub, sub, -);
vector_scalar_ops!(Mul, mul, *);
vector_scalar_ops!(Div, div, /);
vector_scalar_ops!(Rem, rem, %);
self_self_assign_ops!(Vector, AddAssign, add_assign, +);
self_self_assign_ops!(Vector, SubAssign, sub_assign, -);
self_scalar_assign_ops!(Vector, AddAssign, add_assign, +);
self_scalar_assign_ops!(Vector, SubAssign, sub_assign, -);
self_scalar_assign_ops!(Vector, MulAssign, mul_assign, *);
self_scalar_assign_ops!(Vector, DivAssign, div_assign, /);
self_scalar_assign_ops!(Vector, RemAssign, rem_assign, %);

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Matrix {
    pub vects: Vec<Vector>,
}
impl Matrix {
    pub fn new(mat: Vec<Vec<Weight>>) -> Matrix {
        let vects: Vec<Vector> = mat.into_iter().map(|vect| Vector::new(vect)).collect();
        for i in 0..vects.len() {
            assert!(vects[0].len() == vects[i].len());
        }
        Matrix { vects: vects }
    }
    // (A B)
    // (C D)
    pub fn new_from_4mat(a: &Matrix, b: &Matrix, c: &Matrix, d: &Matrix) -> Matrix {
        assert!(a.width() == c.width());
        assert!(b.width() == d.width());
        assert!(a.height() == b.height());
        assert!(c.height() == d.height());
        let mut ret = Matrix::zero(a.height() + c.height(), a.width() + b.width());
        ret.set_matrix(0, 0, a);
        ret.set_matrix(0, a.width(), b);
        ret.set_matrix(a.height(), 0, c);
        ret.set_matrix(a.height(), a.width(), d);
        return ret;
    }
    pub fn zero(h: usize, w: usize) -> Matrix {
        Matrix {
            vects: vec![Vector::zero(w); h],
        }
    }
    pub fn identity(dim: usize) -> Matrix {
        let mut ret = Matrix::zero(dim, dim);
        for i in 0..dim {
            ret[i][i] = 1 as Weight;
        }
        return ret;
    }
    pub fn height(&self) -> usize {
        self.vects.len()
    }
    pub fn width(&self) -> usize {
        self[0].len()
    }
    pub fn set_matrix(&mut self, sy: usize, sx: usize, matrix: &Matrix) {
        assert!(sy + matrix.height() <= self.height());
        assert!(sx + matrix.width() <= self.width());
        for y in 0..matrix.height() {
            for x in 0..matrix.width() {
                self[sy + y][sx + x] = matrix[y][x];
            }
        }
    }
    pub fn get_matrix(&self, sy: usize, sx: usize, h: usize, w: usize) -> Matrix {
        assert!(sy + h <= self.height());
        assert!(sx + w <= self.width());
        let mut ret = Matrix::zero(h, w);
        for y in 0..h {
            for x in 0..w {
                ret[y][x] = self[sy + y][sx + x];
            }
        }
        return ret;
    }
    pub fn row(&self, y: usize) -> Vector {
        assert!(y < self.height());
        return self[y].clone();
    }
    pub fn column(&self, x: usize) -> Vector {
        assert!(x < self.width());
        let mut ret = Vector::zero(self.height());
        for y in 0..self.height() {
            ret[y] = self[y][x];
        }
        return ret;
    }
    pub fn trace(&self) -> Weight {
        assert!(self.height() == self.width());
        let mut ret = 0 as Weight;
        for i in 0..self.height() {
            ret += self[i][i];
        }
        return ret;
    }
    pub fn transpose(&self) -> Matrix {
        let mut ret = Matrix::zero(self.width(), self.height());
        for y in 0..self.height() {
            for x in 0..self.width() {
                ret[x][y] = self[y][x];
            }
        }
        return ret;
    }
    pub fn adjoint(&self) -> Matrix {
        assert!(self.height() == 3);
        assert!(self.width() == 3);
        let mut ret = Matrix::zero(3, 3);
        ret[0] = self[1].cross(&self[2]);
        ret[1] = self[2].cross(&self[0]);
        ret[2] = self[0].cross(&self[1]);
        return ret;
    }
    // Wegith‚ªf64‚Å‚È‚¢‚Æ“®‚©‚È‚¢
    // pub fn inverse(&self) -> Matrix {
    //     assert!(self.height() == 3);
    //     assert!(self.width() == 3);
    //     let a = self.adjoint();
    //     let d = a[0].dot(&self.vects[0]);
    //     if d == 0 {
    //         return Matrix::zero(3, 3);
    //     }
    //     return a.transpose() / d;
    // }
    pub fn det(&self) -> Weight {
        assert!(self.height() == 3);
        assert!(self.width() == 3);
        return self[0].dot(&self[1].cross(&self[2]));
    }
}
impl std::ops::Index<usize> for Matrix {
    type Output = Vector;
    #[inline]
    fn index(&self, rhs: usize) -> &Vector {
        &self.vects[rhs]
    }
}
impl std::ops::IndexMut<usize> for Matrix {
    #[inline]
    fn index_mut(&mut self, rhs: usize) -> &mut Vector {
        &mut self.vects[rhs]
    }
}
impl<'a> std::ops::Mul<&'a Matrix> for Matrix {
    type Output = Matrix;
    #[inline]
    fn mul(self, rhs: &'a Matrix) -> Matrix {
        assert!(self.width() == rhs.height());
        let mut ret = Matrix::zero(self.height(), rhs.width());
        for i in 0..self.height() {
            for k in 0..self.width() {
                for j in 0..rhs.width() {
                    ret[i][j] += self[i][k] * rhs[k][j];
                }
            }
        }
        return ret;
    }
}
impl<'a> std::ops::Mul<&'a Vector> for Matrix {
    type Output = Vector;
    #[inline]
    fn mul(self, rhs: &'a Vector) -> Vector {
        assert!(self.width() == rhs.len());
        let mut ret = Vector::zero(self.height());
        for i in 0..self.height() {
            for j in 0..self.width() {
                ret[i] += self[i][j] * rhs[j];
            }
        }
        return ret;
    }
}
impl std::ops::Neg for Matrix {
    type Output = Matrix;
    #[inline]
    fn neg(self) -> Matrix {
        return self * (-1 as Weight);
    }
}

macro_rules! matrix_matrix_ops {
    ( $trate:ident, $fname:ident, $op:tt) => {
impl<'a> std::ops::$trate<&'a Matrix> for Matrix {
    type Output = Matrix;
    #[inline]
    fn $fname(self, rhs: &'a Matrix) -> Matrix {
        assert!(self.height() == rhs.height());
        assert!(self.width() == rhs.width());
        let mut ret = self;
        for i in 0..ret.height() {
            for j in 0..ret.width() {
                ret[i][j] = ret[i][j] $op rhs[i][j];
            }
        }
        return ret;
    }
}
    };
}
macro_rules! matrix_scalar_ops {
    ( $trate:ident, $fname:ident, $op:tt) => {
impl std::ops::$trate<Weight> for Matrix {
    type Output = Matrix;
    #[inline]
    fn $fname(self, rhs: Weight) -> Matrix {
        let mut ret = self;
        for i in 0..ret.height() {
            for j in 0..ret.width() {
                ret[i][j] = ret[i][j] $op rhs;
            }
        }
        return ret;
    }
}
    };
}
matrix_matrix_ops!(Add, add, +);
matrix_matrix_ops!(Sub, sub, -);
matrix_scalar_ops!(Add, add, +);
matrix_scalar_ops!(Sub, sub, -);
matrix_scalar_ops!(Mul, mul, *);
matrix_scalar_ops!(Div, div, /);
matrix_scalar_ops!(Rem, rem, %);
self_self_assign_ops!(Matrix, AddAssign, add_assign, +);
self_self_assign_ops!(Matrix, SubAssign, sub_assign, -);
self_self_assign_ops!(Matrix, MulAssign, mul_assign, *);
self_scalar_assign_ops!(Matrix, AddAssign, add_assign, +);
self_scalar_assign_ops!(Matrix, SubAssign, sub_assign, -);
self_scalar_assign_ops!(Matrix, MulAssign, mul_assign, *);
self_scalar_assign_ops!(Matrix, DivAssign, div_assign, /);
self_scalar_assign_ops!(Matrix, RemAssign, rem_assign, %);

#[allow(dead_code)]
fn matrix_modmul(lhs: &Matrix, rhs: &Matrix, modulo: i64) -> Matrix {
    assert!(lhs.width() == rhs.height());
    let mut ret = Matrix::zero(lhs.height(), rhs.width());
    for i in 0..lhs.height() {
        for k in 0..lhs.width() {
            for j in 0..rhs.width() {
                ret[i][j] = (ret[i][j] + lhs[i][k] * rhs[k][j] % modulo) % modulo;
            }
        }
    }
    return ret;
}
#[allow(dead_code)]
fn matrix_powmod(base: Matrix, power: i64, modulo: i64) -> Matrix {
    assert!(base.height() == base.width());
    let mut base = base;
    let mut power = power;
    let mut ans = Matrix::identity(base.height());
    while power > 0 {
        if (power & 1) == 1 {
            ans = matrix_modmul(&ans, &base, modulo);
        }
        power >>= 1;
        base = matrix_modmul(&base, &base, modulo);
    }
    return ans;
}

#[test]
fn vector_test() {
    let mut v = Vector::new_3d(1, 2, 3);
    assert!(v == Vector::new_3d(1, 2, 3));
    assert!(v.x() == 1);
    assert!(v.y() == 2);
    assert!(v.z() == 3);
    assert!(v.dot(&v) == 14);
    assert!(v.norm() == 14);
    assert!(v.abs() == (14.0f64).sqrt());
    v.vect[0] = 10;
    v.vect[1] = 20;
    v.vect[2] = 30;
    assert!(v.x() == 10);
    assert!(v.y() == 20);
    assert!(v.z() == 30);
    assert!(Vector::new_3d(1, 0, 0).cross(&Vector::new_3d(0, 1, 0)) == Vector::new_3d(0, 0, 1));
}
#[test]
fn vector_ops_test() {
    let mut v1 = Vector::new_3d(1, 2, 3);
    let v2 = Vector::new_3d(1, 2, 3);
    let v3 = v1.clone() + &v2;
    assert!(v3.x() == 2);
    assert!(v3.y() == 4);
    assert!(v3.z() == 6);
    let v3 = v1.clone() + 3;
    assert!(v3.x() == 4);
    assert!(v3.y() == 5);
    assert!(v3.z() == 6);
    v1 += &v2;
    assert!(v1.x() == 2);
    assert!(v1.y() == 4);
    assert!(v1.z() == 6);

    let mut v1 = Vector::new_3d(1, 2, 3);
    let v3 = v1.clone() - &v2;
    assert!(v3.x() == 0);
    assert!(v3.y() == 0);
    assert!(v3.z() == 0);
    let v3 = v1.clone() - 1;
    assert!(v3.x() == 0);
    assert!(v3.y() == 1);
    assert!(v3.z() == 2);
    v1 -= &v2;
    assert!(v1.x() == 0);
    assert!(v1.y() == 0);
    assert!(v1.z() == 0);

    let mut v1 = Vector::new_3d(1, 2, 3);
    let v3 = v1.clone() * 3;
    assert!(v3.x() == 3);
    assert!(v3.y() == 6);
    assert!(v3.z() == 9);
    v1 *= 3;
    assert!(v1.x() == 3);
    assert!(v1.y() == 6);
    assert!(v1.z() == 9);

    let mut v1 = Vector::new_3d(3, 6, 9);
    let v3 = v1.clone() / 3;
    assert!(v3.x() == 1);
    assert!(v3.y() == 2);
    assert!(v3.z() == 3);
    v1 /= 3;
    assert!(v1.x() == 1);
    assert!(v1.y() == 2);
    assert!(v1.z() == 3);

    let mut v1 = Vector::new_3d(1, 2, 3);
    let v3 = v1.clone() % 3;
    assert!(v3.x() == 1);
    assert!(v3.y() == 2);
    assert!(v3.z() == 0);
    v1 %= 3;
    assert!(v1.x() == 1);
    assert!(v1.y() == 2);
    assert!(v1.z() == 0);

    let mut v1 = Vector::new_3d(1, 2, 3);
    v1[0] = 10;
    v1[1] = 20;
    v1[2] = 30;
    assert!(v1[0] == 10);
    assert!(v1[1] == 20);
    assert!(v1[2] == 30);
}

#[test]
fn matrix_test() {
    let mat1 = Matrix::new(vec![vec![1, 2], vec![3, 4]]);
    let v1 = mat1.row(1);
    assert!(v1[0] == 3);
    assert!(v1[1] == 4);

    let v1 = mat1.column(1);
    assert!(v1[0] == 2);
    assert!(v1[1] == 4);

    assert!(mat1.trace() == 5);

    let mat2 = mat1.transpose();
    assert!(mat2[0][0] == 1);
    assert!(mat2[0][1] == 3);
    assert!(mat2[1][0] == 2);
    assert!(mat2[1][1] == 4);
}

#[test]
fn matrix_ops_test() {
    let mat1 = Matrix::new(vec![vec![1, 2], vec![2, 3]]);
    let mat2 = Matrix::new(vec![vec![1, 1], vec![0, 1]]);
    let v1 = Vector::new(vec![1, 2]);
    let mat3 = mat1.clone() + &mat2;
    assert!(mat3[0][0] == 2);
    assert!(mat3[0][1] == 3);
    assert!(mat3[1][0] == 2);
    assert!(mat3[1][1] == 4);

    let mat3 = mat1.clone() - &mat2;
    assert!(mat3[0][0] == 0);
    assert!(mat3[0][1] == 1);
    assert!(mat3[1][0] == 2);
    assert!(mat3[1][1] == 2);

    let mat3 = mat1.clone() * 3;
    assert!(mat3[0][0] == 3);
    assert!(mat3[0][1] == 6);
    assert!(mat3[1][0] == 6);
    assert!(mat3[1][1] == 9);

    // 1 2   1 1
    // 2 3 * 0 1
    let mat3 = mat1.clone() * &mat2;
    assert!(mat3[0][0] == 1);
    assert!(mat3[0][1] == 3);
    assert!(mat3[1][0] == 2);
    assert!(mat3[1][1] == 5);

    let v2 = mat1.clone() * &v1;
    assert!(v2[0] == 5);
    assert!(v2[1] == 8);
}

#[test]
fn matrix_powmod_test() {
    let mut mat = Matrix::zero(3, 3);
    mat[0][0] = 1;
    mat[1][1] = 2;
    mat[2][2] = 3;
    let temp = matrix_powmod(mat, 4, 7);
    assert!(temp[0][0] == 1);
    assert!(temp[1][1] == 16 % 7);
    assert!(temp[2][2] == 81 % 7);
}
