use std::collections::{HashSet, VecDeque};

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let initial_state = match start_bucket {
        Bucket::One => (capacity_1, 0),
        Bucket::Two => (0, capacity_2),
    };
    let opposite_state = match start_bucket {
        Bucket::One => (0, capacity_2),
        Bucket::Two => (capacity_1, 0),
    };
    queue.push_back((1, initial_state));
    while let Some((moves, state)) = queue.pop_front() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state);
        if state == opposite_state {
            continue;
        }
        let (bucket_1, bucket_2) = state;
        assert!(bucket_1 <= capacity_1);
        assert!(bucket_2 <= capacity_2);

        if bucket_1 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::One,
                other_bucket: bucket_2,
            });
        } else if bucket_2 == goal {
            return Some(BucketStats {
                moves,
                goal_bucket: Bucket::Two,
                other_bucket: bucket_1,
            });
        }

        let moves = moves + 1;

        // pour 1 into 2
        let d = bucket_1.min(capacity_2 - bucket_2);
        if d != 0 {
            queue.push_back((moves, (bucket_1 - d, bucket_2 + d)));
        }
        // pour 2 into 1
        let d = bucket_2.min(capacity_1 - bucket_1);
        if d != 0 {
            queue.push_back((moves, (bucket_1 + d, bucket_2 - d)));
        }
        // empty 1
        queue.push_back((moves, (0, bucket_2)));
        // empty 2
        queue.push_back((moves, (bucket_1, 0)));
        // filling 1
        queue.push_back((moves, (capacity_1, bucket_2)));
        // filling 2
        queue.push_back((moves, (bucket_1, capacity_2)));
    }
    None
}
