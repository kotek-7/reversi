#[derive(Debug, Clone)]
pub struct Game {
    board: crate::board::Board,
    next_disk_to_place: crate::disk::Disk,
}

impl Game {
    pub fn init(first_move: crate::disk::Disk) -> Game {
        Game {
            board: crate::board::Board::new_init(),
            next_disk_to_place: first_move
        }
    }

    pub fn set_disk_and_progress(&mut self, pos_x: usize, pos_y: usize) -> Result<(), crate::board::SetDiskError> {
        self.board = self.board.set_disk(pos_x, pos_y, self.next_disk_to_place)?;
        if self.board.check_skip(self.next_disk_to_place) {
            self.next_disk_to_place = self.next_disk_to_place;
        } else {
            self.next_disk_to_place = self.next_disk_to_place.rev();
        }
        Ok(())
    }

    pub fn get_board(&self) -> crate::board::Board {
        self.board
    }

    pub fn get_next_disk(&self) -> crate::disk::Disk {
        self.next_disk_to_place
    }
}