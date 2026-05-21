use crate::pose::Pose;
use crate::state::State;

pub struct Executor {
    pose: Pose,
    state: State,
}

impl Executor {
    pub fn with_pose(pose: Pose) -> Self {
        Executor {
            pose,
            state: State::default(),
        }
    }

    pub fn execute(&mut self, cmds: &str) {
        for cmd in cmds.chars() {
            match cmd {
                'B' => self.state.be_reverse(),
                'M' => {
                    if self.state.is_reverse {
                        self.pose.forward(-1);
                    } else {
                        self.pose.forward(1);
                    }
                }
                'L' => {
                    if self.state.is_reverse {
                        self.pose.turn_right();
                    } else {
                        self.pose.turn_left();
                    }
                }
                'R' => {
                    if self.state.is_reverse {
                        self.pose.turn_left();
                    } else {
                        self.pose.turn_right();
                    }
                }
                _ => (),
            }
        }
    }

    pub fn query(&self) -> Pose {
        self.pose
    }
}
