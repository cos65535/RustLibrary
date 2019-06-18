const DIM: usize = 3;
type Weight = i64;
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Vector {
    pub vect: [Weight; DIM],
}
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
#[allow(dead_code)]
impl Vector {
    pub fn new(x: Weight, y: Weight, z: Weight) -> Vector {
        Vector { vect: [x, y, z] }
    }
    pub fn new_nd(v: [Weight; DIM]) -> Vector {
        Vector { vect: v }
    }
    pub fn zero() -> Vector {
        Vector { vect: [0; DIM] }
    }
    pub fn len(&self) -> usize {
        DIM
    }
    pub fn x(&self) -> Weight {
        self.vect[0]
    }
    pub fn y(&self) -> Weight {
        self.vect[1]
    }
    pub fn z(&self) -> Weight {
        self.vect[2]
    }
    pub fn dot(&self, rhs: Vector) -> Weight {
        assert!(self.len() == rhs.len());
        let mut ret = 0;
        for i in 0..self.len() {
            ret += self.vect[i] * rhs.vect[i];
        }
        return ret;
    }
    pub fn cross(&self, rhs: Vector) -> Vector {
        assert!(self.len() == 3);
        assert!(rhs.len() == 3);
        let mut ret = Vector::new(0, 0, 0);
        ret.vect[0] = self.y() * rhs.z() - self.z() * rhs.y();
        ret.vect[1] = self.z() * rhs.x() - self.x() * rhs.z();
        ret.vect[2] = self.x() * rhs.y() - self.y() * rhs.x();
        return ret;
    }
    pub fn norm(&self) -> Weight {
        self.dot(*self)
    }
    pub fn abs(&self) -> f64 {
        (self.norm() as f64).sqrt()
    }
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
#[test]
fn vector_inner_test() {
    let mut v = Vector::new(1, 2, 3);
    assert!(v.x() == 1);
    assert!(v.y() == 2);
    assert!(v.z() == 3);
    assert!(v.dot(v) == 14);
    assert!(v.norm() == 14);
    assert!(v.abs() == (14.0f64).sqrt());
    v.vect[0] = 10;
    v.vect[1] = 20;
    v.vect[2] = 30;
    assert!(v.x() == 10);
    assert!(v.y() == 20);
    assert!(v.z() == 30);
    assert!(Vector::new(1, 0, 0).cross(Vector::new(0, 1, 0)) == Vector::new(0, 0, 1));
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;
    #[inline]
    fn add(self, rhs: Vector) -> Vector {
        let mut ret = self.clone();
        for i in 0..self.len() {
            ret[i] += rhs[i];
        }
        return ret;
    }
}
impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: Vector) -> Vector {
        let mut ret = self.clone();
        for i in 0..self.len() {
            ret[i] -= rhs[i];
        }
        return ret;
    }
}
impl std::ops::Add<Weight> for Vector {
    type Output = Vector;
    #[inline]
    fn add(self, rhs: Weight) -> Vector {
        let mut ret = self.clone();
        for i in 0..self.len() {
            ret[i] += rhs;
        }
        return ret;
    }
}
impl std::ops::Sub<Weight> for Vector {
    type Output = Vector;
    #[inline]
    fn sub(self, rhs: Weight) -> Vector {
        let mut ret = self.clone();
        for i in 0..self.len() {
            ret[i] -= rhs;
        }
        return ret;
    }
}
impl std::ops::Mul<Weight> for Vector {
    type Output = Vector;
    #[inline]
    fn mul(self, rhs: Weight) -> Vector {
        let mut ret = self.clone();
        for i in 0..self.len() {
            ret[i] *= rhs;
        }
        return ret;
    }
}
impl std::ops::Div<Weight> for Vector {
    type Output = Vector;
    #[inline]
    fn div(self, rhs: Weight) -> Vector {
        let mut ret = self.clone();
        for i in 0..self.len() {
            ret[i] /= rhs;
        }
        return ret;
    }
}
impl std::ops::Rem<Weight> for Vector {
    type Output = Vector;
    #[inline]
    fn rem(self, rhs: Weight) -> Vector {
        let mut ret = self.clone();
        for i in 0..self.len() {
            ret[i] %= rhs;
        }
        return ret;
    }
}
impl std::ops::AddAssign<Vector> for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: Vector) {
        *self = *self + rhs;
    }
}
impl std::ops::SubAssign<Vector> for Vector {
    #[inline]
    fn sub_assign(&mut self, rhs: Vector) {
        *self = *self - rhs;
    }
}
impl std::ops::AddAssign<Weight> for Vector {
    #[inline]
    fn add_assign(&mut self, rhs: Weight) {
        *self = *self + rhs;
    }
}
impl std::ops::SubAssign<Weight> for Vector {
    #[inline]
    fn sub_assign(&mut self, rhs: Weight) {
        *self = *self - rhs;
    }
}
impl std::ops::MulAssign<Weight> for Vector {
    #[inline]
    fn mul_assign(&mut self, rhs: Weight) {
        *self = *self * rhs;
    }
}
impl std::ops::DivAssign<Weight> for Vector {
    #[inline]
    fn div_assign(&mut self, rhs: Weight) {
        *self = *self / rhs;
    }
}
impl std::ops::RemAssign<Weight> for Vector {
    #[inline]
    fn rem_assign(&mut self, rhs: Weight) {
        *self = *self % rhs;
    }
}
impl std::ops::Neg for Vector {
    type Output = Vector;
    #[inline]
    fn neg(self) -> Vector {
        return self * -1;
    }
}
#[test]
fn vect_ops_test() {
    let mut v1 = Vector::new(1, 2, 3);
    let v2 = Vector::new(1, 2, 3);
    let v3 = v1 + v2;
    assert!(v3.x() == 2);
    assert!(v3.y() == 4);
    assert!(v3.z() == 6);
    let v3 = v1 + 3;
    assert!(v3.x() == 4);
    assert!(v3.y() == 5);
    assert!(v3.z() == 6);
    v1 += v2;
    assert!(v1.x() == 2);
    assert!(v1.y() == 4);
    assert!(v1.z() == 6);

    let mut v1 = Vector::new(1, 2, 3);
    let v3 = v1 - v2;
    assert!(v3.x() == 0);
    assert!(v3.y() == 0);
    assert!(v3.z() == 0);
    let v3 = v1 - 1;
    assert!(v3.x() == 0);
    assert!(v3.y() == 1);
    assert!(v3.z() == 2);
    v1 -= v2;
    assert!(v1.x() == 0);
    assert!(v1.y() == 0);
    assert!(v1.z() == 0);

    let mut v1 = Vector::new(1, 2, 3);
    let v3 = v1 * 3;
    assert!(v3.x() == 3);
    assert!(v3.y() == 6);
    assert!(v3.z() == 9);
    v1 *= 3;
    assert!(v1.x() == 3);
    assert!(v1.y() == 6);
    assert!(v1.z() == 9);

    let mut v1 = Vector::new(3, 6, 9);
    let v3 = v1 / 3;
    assert!(v3.x() == 1);
    assert!(v3.y() == 2);
    assert!(v3.z() == 3);
    v1 /= 3;
    assert!(v1.x() == 1);
    assert!(v1.y() == 2);
    assert!(v1.z() == 3);

    let mut v1 = Vector::new(1, 2, 3);
    let v3 = v1 % 3;
    assert!(v3.x() == 1);
    assert!(v3.y() == 2);
    assert!(v3.z() == 0);
    v1 %= 3;
    assert!(v1.x() == 1);
    assert!(v1.y() == 2);
    assert!(v1.z() == 0);

    let mut v1 = Vector::new(1, 2, 3);
    v1[0] = 10;
    v1[1] = 20;
    v1[2] = 30;
    assert!(v1[0] == 10);
    assert!(v1[1] == 20);
    assert!(v1[2] == 30);
}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Matrix {
    pub vects: [Vector; DIM],
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
impl Matrix {
    pub fn zero() -> Matrix {
        Matrix {
            vects: [Vector::zero(); DIM],
        }
    }
    pub fn identity() -> Matrix {
        let mut ret = Matrix::zero();
        for i in 0..DIM {
            ret[i][i] = 1;
        }
        return ret;
    }
    pub fn height(&self) -> usize {
        DIM
    }
    pub fn width(&self) -> usize {
        DIM
    }
    pub fn row(&self, y: usize) -> Vector {
        assert!(y < self.height());
        return self[y];
    }
    pub fn column(&self, x: usize) -> Vector {
        assert!(x < self.width());
        let mut ret = Vector::zero();
        for y in 0..self.height() {
            ret[y] = self[y][x];
        }
        return ret;
    }
    pub fn trace(&self) -> Weight {
        let mut ret = 0;
        for i in 0..DIM {
            ret += self[i][i];
        }
        return ret;
    }
    pub fn transpose(&self) -> Matrix {
        let mut ret = Matrix::zero();
        for y in 0..self.height() {
            for x in 0..self.width() {
                ret[x][y] = self[y][x];
            }
        }
        return ret;
    }
    pub fn adjoint(&self) -> Matrix {
        assert!(DIM == 3);
        let mut ret = Matrix::zero();
        ret[0] = self[1].cross(self[2]);
        ret[1] = self[2].cross(self[0]);
        ret[2] = self[0].cross(self[1]);
        return ret;
    }
    // pub fn inverse(&self) -> Matrix {
    //     assert!(DIM == 3);
    //     let a = self.adjoint();
    //     let d = a[0].dot(self.vects[0]);
    //     if d == 0 {
    //         return Matrix::zero();
    //     }
    //     return a.transpose() / d;
    // }
    pub fn det(&self) -> Weight {
        assert!(DIM == 3);
        return self[0].dot(self[1].cross(self[2]));
    }
}
impl std::ops::Add<Matrix> for Matrix {
    type Output = Matrix;
    #[inline]
    fn add(self, rhs: Matrix) -> Matrix {
        let mut ret = self.clone();
        for i in 0..self.height() {
            ret[i] += rhs[i];
        }
        return ret;
    }
}
impl std::ops::Sub<Matrix> for Matrix {
    type Output = Matrix;
    #[inline]
    fn sub(self, rhs: Matrix) -> Matrix {
        let mut ret = self.clone();
        for i in 0..self.height() {
            ret[i] -= rhs[i];
        }
        return ret;
    }
}
impl std::ops::Mul<Matrix> for Matrix {
    type Output = Matrix;
    #[inline]
    fn mul(self, rhs: Matrix) -> Matrix {
        let mut ret = Matrix::zero();
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
impl std::ops::Mul<Vector> for Matrix {
    type Output = Vector;
    #[inline]
    fn mul(self, rhs: Vector) -> Vector {
        let mut ret = Vector::zero();
        for i in 0..self.height() {
            for j in 0..self.width() {
                ret[i] += self[i][j] * rhs[j];
            }
        }
        return ret;
    }
}
impl std::ops::Add<Weight> for Matrix {
    type Output = Matrix;
    #[inline]
    fn add(self, rhs: Weight) -> Matrix {
        let mut ret = self;
        for i in 0..self.height() {
            ret[i] += rhs;
        }
        return ret;
    }
}
impl std::ops::Sub<Weight> for Matrix {
    type Output = Matrix;
    #[inline]
    fn sub(self, rhs: Weight) -> Matrix {
        let mut ret = self;
        for i in 0..self.height() {
            ret[i] -= rhs;
        }
        return ret;
    }
}
impl std::ops::Mul<Weight> for Matrix {
    type Output = Matrix;
    #[inline]
    fn mul(self, rhs: Weight) -> Matrix {
        let mut ret = self;
        for i in 0..self.height() {
            ret[i] *= rhs;
        }
        return ret;
    }
}
impl std::ops::Div<Weight> for Matrix {
    type Output = Matrix;
    #[inline]
    fn div(self, rhs: Weight) -> Matrix {
        let mut ret = self;
        for i in 0..self.height() {
            ret[i] /= rhs;
        }
        return ret;
    }
}
impl std::ops::Rem<Weight> for Matrix {
    type Output = Matrix;
    #[inline]
    fn rem(self, rhs: Weight) -> Matrix {
        let mut ret = self;
        for i in 0..self.height() {
            ret[i] %= rhs;
        }
        return ret;
    }
}
impl std::ops::AddAssign<Matrix> for Matrix {
    #[inline]
    fn add_assign(&mut self, rhs: Matrix) {
        *self = *self + rhs;
    }
}
impl std::ops::SubAssign<Matrix> for Matrix {
    #[inline]
    fn sub_assign(&mut self, rhs: Matrix) {
        *self = *self - rhs;
    }
}
impl std::ops::MulAssign<Matrix> for Matrix {
    #[inline]
    fn mul_assign(&mut self, rhs: Matrix) {
        *self = *self * rhs;
    }
}
impl std::ops::AddAssign<Weight> for Matrix {
    #[inline]
    fn add_assign(&mut self, rhs: Weight) {
        *self = *self + rhs;
    }
}
impl std::ops::SubAssign<Weight> for Matrix {
    #[inline]
    fn sub_assign(&mut self, rhs: Weight) {
        *self = *self - rhs;
    }
}
impl std::ops::MulAssign<Weight> for Matrix {
    #[inline]
    fn mul_assign(&mut self, rhs: Weight) {
        *self = *self * rhs;
    }
}
impl std::ops::DivAssign<Weight> for Matrix {
    #[inline]
    fn div_assign(&mut self, rhs: Weight) {
        *self = *self / rhs;
    }
}
impl std::ops::RemAssign<Weight> for Matrix {
    #[inline]
    fn rem_assign(&mut self, rhs: Weight) {
        *self = *self % rhs;
    }
}
impl std::ops::Neg for Matrix {
    type Output = Matrix;
    #[inline]
    fn neg(self) -> Matrix {
        return self * -1;
    }
}
#[allow(dead_code)]
fn matrix_modmul(lhs: Matrix, rhs: Matrix, modulo: i64) -> Matrix {
    let mut ret = Matrix::zero();
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
    let mut base = base;
    let mut power = power;
    let mut ans = Matrix::identity();
    while power > 0 {
        if (power & 1) == 1 {
            ans = matrix_modmul(ans, base, modulo);
        }
        power >>= 1;
        base = matrix_modmul(base, base, modulo);
    }
    return ans;
}

#[test]
fn matrix_powmod_test() {
    let mut mat = Matrix::zero();
    mat[0][0] = 1;
    mat[1][1] = 2;
    mat[2][2] = 3;
    let temp = matrix_powmod(mat, 4, 7);
    assert!(temp[0][0] == 1);
    assert!(temp[1][1] == 16 % 7);
    assert!(temp[2][2] == 81 % 7);
}
