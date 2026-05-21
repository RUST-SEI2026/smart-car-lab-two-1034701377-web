use executor::{Executor, Pose};

mod fast_tests {
    use super::*;

    #[test]
    fn should_return_x_plus_2_given_status_is_fast_command_is_m_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("FM");

        let expected_pose = Pose::new(2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_2_given_status_is_reverse_and_fast_command_is_m_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("BFM");

        let expected_pose = Pose::new(-2, 0, 'E');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_y_plus_1_given_command_is_ffm_and_facing_is_n() {
        let original_pose = Pose::new(0, 0, 'N');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("FFM");

        let expected_pose = Pose::new(0, 1, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_and_n_given_status_is_fast_command_is_l_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("FL");

        let expected_pose = Pose::new(1, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_and_s_given_status_is_reverse_and_fast_command_is_l_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("BFL");

        let expected_pose = Pose::new(-1, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_plus_1_and_s_given_status_is_fast_command_is_r_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("FR");

        let expected_pose = Pose::new(1, 0, 'S');
        assert_eq!(expected_pose, executor.query());
    }

    #[test]
    fn should_return_x_minus_1_and_n_given_status_is_reverse_and_fast_command_is_r_and_facing_is_e() {
        let original_pose = Pose::new(0, 0, 'E');
        let mut executor = Executor::with_pose(original_pose);

        executor.execute("BFR");

        let expected_pose = Pose::new(-1, 0, 'N');
        assert_eq!(expected_pose, executor.query());
    }
}
