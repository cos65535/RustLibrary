#[allow(dead_code)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug, Hash)]
struct Dice<T: Clone + Copy> {
    face: [T; 6],
}

#[allow(dead_code)]
impl<T: Default + Copy> Dice<T> {
    fn new(face: &[T]) -> Dice<T> {
        assert!(face.len() == 6);
        let mut temp: [T; 6] = [T::default(); 6];
        for i in 0..6 {
            temp[i] = face[i];
        }
        Dice { face: temp }
    }

    fn top(&self) -> T {
        self.face[0]
    }
    fn bottom(&self) -> T {
        self.face[1]
    }
    fn right(&self) -> T {
        self.face[2]
    }
    fn left(&self) -> T {
        self.face[3]
    }
    fn front(&self) -> T {
        self.face[4]
    }
    fn back(&self) -> T {
        self.face[5]
    }

    fn rotate(&self, cnt: i32, dir: usize) -> Dice<T> {
        if cnt == 0 {
            return self.clone();
        }
        let cnt = if cnt >= 0 { cnt } else { cnt % 4 + 4 };
        let mut face: [T; 6] = [T::default(); 6];
        if dir == 0 {
            // +X rotate(Y axis rotate)
            face[0] = self.face[3];
            face[1] = self.face[2];
            face[2] = self.face[0];
            face[3] = self.face[1];
            face[4] = self.face[4];
            face[5] = self.face[5];
        } else if dir == 1 {
            // +Y rotate(X axis rotate)
            face[0] = self.face[4];
            face[1] = self.face[5];
            face[2] = self.face[2];
            face[3] = self.face[3];
            face[4] = self.face[1];
            face[5] = self.face[0];
        } else if dir == 2 {
            // Z rotate(Z axis rotate)
            face[0] = self.face[0];
            face[1] = self.face[1];
            face[2] = self.face[4];
            face[3] = self.face[5];
            face[4] = self.face[3];
            face[5] = self.face[2];
        } else {
            panic!("Wrong direction {}", dir);
        }
        return Dice::new(&face).rotate(cnt - 1, dir);
    }
    fn all_rotate(&self) -> Vec<Dice<T>> {
        let mut ret = vec![];
        let mut temp = *self;
        for i in 0..6 {
            for _j in 0..4 {
                ret.push(temp);
                temp = temp.rotate(1, 2);
            }
            temp = temp.rotate(1, i & 1);
        }
        ret
    }
}
