use super::*;
use taxicab_map::TaxicabMap;

#[derive(Clone, Debug)]
pub struct MazeState {
    pub width: usize,
    pub height: usize,
    pub walked: TaxicabMap<bool>,
    pub joints: Vec<Joint>,
    pub rng: SmallRng,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BfsWorker {
    walked: Vec<(isize, isize)>,
}

impl MazeState {
    pub fn nearby(&self, x: isize, y: isize) -> Vec<Joint> {
        self.walked.joints_nearby(x, y).iter().filter(|joint| !self.is_walked(joint)).cloned().collect()
    }
    #[inline]
    pub fn is_finished(&self) -> bool {
        self.walked.points_all().all(|(_, _, walked)| *walked)
    }
    #[inline]
    pub fn is_walked(&self, joint: &Joint) -> bool {
        let (x, y) = joint.target();
        self.walked.get_point(x as isize, y as isize).map(|s| *s).unwrap_or(true)
    }
}

impl BfsWorker {
    pub fn go_back(&mut self) {
        self.walked.pop();
    }
    pub fn go_walk(&mut self, nearby: &[Joint], state: &mut MazeState) {
        let index = state.rng.gen_range(0..nearby.len());
        let joint = &nearby[index];
        let (x, y) = (joint.target().0, joint.target().1);
        self.walked.push((x, y));
        state.walked.set_point(x as isize, y as isize, true);
        state.joints.push(*joint);
    }
}

impl Maze2DConfig {
    pub fn initial(&self) -> TaxicabMap<bool> {
        let mut walked = TaxicabMap::rectangle(self.width, self.height, &false);
        let (x, y) = self.get_entry();
        walked.set_point(x as isize, y as isize, true);
        for (x, y) in self.bad.iter() {
            walked.set_point(*x, *y, true);
        }
        walked
    }
    pub fn build_dfs(&self) -> impl Iterator<Item = Maze2D> {
        let config = self.clone();
        let entry = self.get_entry();
        let mut worker = BfsWorker { walked: vec![(entry.0 as isize, entry.1 as isize)] };
        let mut state = MazeState {
            width: self.width,
            height: self.height,
            walked: self.initial(),
            joints: Vec::with_capacity(self.width * self.height * 2),
            rng: self.get_rng(),
        };
        from_generator(move || {
            while !state.is_finished() {
                match worker.walked.last() {
                    Some(head) => {
                        let mut nearby = state.nearby(head.0, head.1);
                        if nearby.is_empty() {
                            worker.go_back()
                        }
                        else {
                            worker.go_walk(&mut nearby, &mut state);
                            yield Maze2D::new(&config, &state.joints, &[]);
                        }
                    }
                    None => {
                        todo!()
                    }
                }
            }
            yield Maze2D::new(&config, &state.joints, &[])
        })
    }
}
