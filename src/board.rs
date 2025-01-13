#[cfg(test)]
mod tests;

use crate::disk::Disk;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Board {
    board: [[Disk; 8]; 8],
}

impl Board {
    pub fn new_init() -> Board {
        use Disk::*;
        Board::new_with([
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, W, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ])
    }

    pub fn get(&self) -> &[[Disk; 8]; 8] {
        &self.board
    }

    pub fn set_disk(&self, pos_x: usize, pos_y: usize, value: Disk) -> Result<Board, SetDiskError> {
        if let Disk::E = value {
            return Err(SetDiskError::SetEmptyDiskError(SetEmptyDiskError));
        }
        match self.get_disk(pos_x, pos_y) {
            Ok(disk) => {
                if disk != Disk::E {
                    return Err(SetDiskError::InvalidPosError(InvalidPosError));
                }
            }
            Err(e) => {
                return Err(SetDiskError::OutOfIndexError(e));
            }
        }

        let mut result_board = self
            .set_disk_anyway(pos_x, pos_y, value)
            .map_err(|e| SetDiskError::OutOfIndexError(e))?;

        use crate::direction::Direction::*;
        let directions = [T, TR, R, BR, B, BL, L, TL];
        let mut reversed_at_least_one = false;
        for direction in directions {
            match self.find_valid_disk_in_direction(pos_x, pos_y, value, direction) {
                Ok(offset_option) => match offset_option {
                    Some(offset) => {
                        for i in 1..offset {
                            result_board = result_board
                                .set_disk_anyway_with_directional_offset(
                                    pos_x, pos_y, value, i, direction,
                                )
                                .unwrap_or(result_board);
                            reversed_at_least_one = true;
                        }
                    }
                    None => {
                        continue;
                    }
                },
                Err(e) => {
                    return Err(SetDiskError::OutOfIndexError(e));
                }
            }
        }
        if !reversed_at_least_one {
            return Err(SetDiskError::InvalidPosError(InvalidPosError));
        }
        Ok(result_board)
    }

    pub fn check_skip(&self, next_disk_to_place: Disk) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                if self.get_disk(x, y).unwrap_or(Disk::E) != Disk::E {
                    continue;
                }
                if let Ok(_) = self.set_disk(x, y, next_disk_to_place) {
                    return false;
                }
            }
        }
        return true;
    }

    fn new_with(board: [[Disk; 8]; 8]) -> Board {
        Board { board }
    }

    fn get_disk(&self, pos_x: usize, pos_y: usize) -> Result<Disk, OutOfIndexError> {
        match self.board.get(pos_y) {
            Some(row) => match row.get(pos_x) {
                Some(v) => Ok(*v),
                None => Err(OutOfIndexError),
            },
            None => Err(OutOfIndexError),
        }
    }

    fn get_disk_with_directional_offset(
        &self,
        start_pos_x: usize,
        start_pos_y: usize,
        offset: usize,
        direction: crate::direction::Direction,
    ) -> Result<Disk, OutOfIndexError> {
        use crate::direction::Direction::*;
        match direction {
            T => self.get_disk(
                start_pos_x,
                start_pos_y.checked_sub(offset).ok_or(OutOfIndexError)?,
            ),
            TR => self.get_disk(
                start_pos_x + offset,
                start_pos_y.checked_sub(offset).ok_or(OutOfIndexError)?,
            ),
            R => self.get_disk(start_pos_x + offset, start_pos_y),
            BR => self.get_disk(start_pos_x + offset, start_pos_y + offset),
            B => self.get_disk(start_pos_x, start_pos_y + offset),
            BL => self.get_disk(
                start_pos_x.checked_sub(offset).ok_or(OutOfIndexError)?,
                start_pos_y + offset,
            ),
            L => self.get_disk(
                start_pos_x.checked_sub(offset).ok_or(OutOfIndexError)?,
                start_pos_y,
            ),
            TL => self.get_disk(
                start_pos_x.checked_sub(offset).ok_or(OutOfIndexError)?,
                start_pos_y.checked_sub(offset).ok_or(OutOfIndexError)?,
            ),
        }
    }

    /// 相手の駒を与えられた駒と挟む形になっている自分の駒をdirection方向に探して、
    /// 最初に見つかった駒の位置のoffsetを返します。
    /// 見つからなかった場合はNoneが、探索開始位置がボードの外の場合はErrが返されます。
    /// ((start_pos_x, start_pos_y)の位置の駒は探索対象外です。)
    fn find_valid_disk_in_direction(
        &self,
        start_pos_x: usize,
        start_pos_y: usize,
        value: Disk,
        direction: crate::direction::Direction,
    ) -> Result<Option<usize>, OutOfIndexError> {
        match self.get_disk_with_directional_offset(start_pos_x, start_pos_y, 1, direction) {
            Ok(disk) => {
                if disk == value || disk == Disk::E {
                    return Ok(None);
                }
            }
            Err(_) => {
                return Ok(None);
            }
        }
        let mut i = 2;
        loop {
            match self.get_disk_with_directional_offset(start_pos_x, start_pos_y, i, direction) {
                Ok(disk) => {
                    if disk == value {
                        break Ok(Some(i));
                    }
                    if disk == Disk::E {
                        break Ok(None);
                    }
                }
                Err(_) => {
                    break Ok(None);
                }
            };
            i = i + 1;
        }
    }

    fn set_disk_anyway(
        &self,
        pos_x: usize,
        pos_y: usize,
        value: Disk,
    ) -> Result<Board, OutOfIndexError> {
        let prev_board = self.board;
        let mut new_board = prev_board;
        match new_board.get_mut(pos_y) {
            Some(row) => match row.get_mut(pos_x) {
                Some(v) => {
                    *v = value;
                }
                None => {
                    return Err(OutOfIndexError);
                }
            },
            None => {
                return Err(OutOfIndexError);
            }
        };
        Ok(Board::new_with(new_board))
    }

    fn set_disk_anyway_with_directional_offset(
        &self,
        start_pos_x: usize,
        start_pos_y: usize,
        value: Disk,
        offset: usize,
        direction: crate::direction::Direction,
    ) -> Result<Board, OutOfIndexError> {
        use crate::direction::Direction::*;
        match direction {
            T => self.set_disk_anyway(
                start_pos_x,
                start_pos_y.checked_sub(offset).ok_or(OutOfIndexError)?,
                value,
            ),
            TR => self.set_disk_anyway(
                start_pos_x + offset,
                start_pos_y.checked_sub(offset).ok_or(OutOfIndexError)?,
                value,
            ),
            R => self.set_disk_anyway(start_pos_x + offset, start_pos_y, value),
            BR => self.set_disk_anyway(start_pos_x + offset, start_pos_y + offset, value),
            B => self.set_disk_anyway(start_pos_x, start_pos_y + offset, value),
            BL => self.set_disk_anyway(
                start_pos_x.checked_sub(offset).ok_or(OutOfIndexError)?,
                start_pos_y + offset,
                value,
            ),
            L => self.set_disk_anyway(
                start_pos_x.checked_sub(offset).ok_or(OutOfIndexError)?,
                start_pos_y,
                value,
            ),
            TL => self.set_disk_anyway(
                start_pos_x.checked_sub(offset).ok_or(OutOfIndexError)?,
                start_pos_y.checked_sub(offset).ok_or(OutOfIndexError)?,
                value,
            ),
        }
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let board = self.get();
        let mut display = "".to_string();
        display.push_str("  0 1 2 3 4 5 6 7 \n");

        for y in 0..8 {
            display.push_str(&(y.to_string() + " "));
            for x in 0..8 {
                display.push_str(&format!("{} ", board[y][x]));
            }
            display.push_str("\n");
        }

        write!(f, "{}", display)
    }
}

#[derive(Debug, PartialEq)]
pub struct OutOfIndexError;

impl std::error::Error for OutOfIndexError {}

impl std::fmt::Display for OutOfIndexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "index is out of length")
    }
}

#[derive(Debug, PartialEq)]
pub struct SetEmptyDiskError;

impl std::error::Error for SetEmptyDiskError {}

impl std::fmt::Display for SetEmptyDiskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot set empty disk")
    }
}

#[derive(Debug, PartialEq)]
pub struct InvalidPosError;

impl std::error::Error for InvalidPosError {}

impl std::fmt::Display for InvalidPosError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the disk cannot placed in that place")
    }
}

#[derive(Debug, PartialEq)]
pub enum SetDiskError {
    OutOfIndexError(OutOfIndexError),
    SetEmptyDiskError(SetEmptyDiskError),
    InvalidPosError(InvalidPosError),
}

impl std::fmt::Display for SetDiskError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SetDiskError::OutOfIndexError(out_of_index_error) => out_of_index_error.fmt(f),
            SetDiskError::SetEmptyDiskError(set_empty_disk_error) => set_empty_disk_error.fmt(f),
            SetDiskError::InvalidPosError(invalid_pos_error) => invalid_pos_error.fmt(f),
        }
    }
}
