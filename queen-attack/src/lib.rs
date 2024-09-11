#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if !(0..8).contains(&rank) || !(0..8).contains(&file) {
            return None;
        }

        Some(Self { rank, file })
    }

    pub fn same_rank(&self, other: &ChessPosition) -> bool {
        self.rank == other.rank
    }

    pub fn same_file(&self, other: &ChessPosition) -> bool {
        self.file == other.file
    }

    pub fn same_diagonal(&self, other: &ChessPosition) -> bool {
        self.rank.abs_diff(other.rank) == self.file.abs_diff(other.file)
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Self { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.same_rank(&other.position)
            || self.position.same_file(&other.position)
            || self.position.same_diagonal(&other.position)
    }
}
