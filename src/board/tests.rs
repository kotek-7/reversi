#[test]
fn test_set_disk() {
    use crate::disk::Disk::*;
    let mut board = super::Board {
        board: [
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, W, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ],
    };
    board = board.set_disk(3, 2, crate::disk::Disk::B).unwrap();
    let expected_board = super::Board {
        board: [
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, B, E, E, E, E],
            [E, E, E, B, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ],
    };
    assert_eq!(board, expected_board);
}

#[test]
fn test_set_disk_anyway() {
    use crate::disk::Disk::*;
    let mut board = super::Board {
        board: [
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, W, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ],
    };
    board = board.set_disk_anyway(3, 2, crate::disk::Disk::B).unwrap();
    let expected_board = super::Board {
        board: [
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, B, E, E, E, E],
            [E, E, E, W, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ],
    };
    assert_eq!(board, expected_board);
}

#[test]
fn test_set_disk_anyway_with_directional_offset() {
    use crate::disk::Disk::*;
    let mut board = super::Board {
        board: [
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, W, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ],
    };
    board = board.set_disk_anyway_with_directional_offset(3, 2, crate::disk::Disk::B, 2, crate::direction::Direction::L).unwrap();
    let expected_board = super::Board {
        board: [
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, B, E, E, E, E, E, E],
            [E, E, E, W, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ],
    };
    assert_eq!(board, expected_board);
}

#[test]
fn test_find_valid_disk_in_direction_1() {
    use crate::disk::Disk::*;
    let board = super::Board {
        board: [
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, W, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ],
    };
    let found_offset = board.find_valid_disk_in_direction(
        2,
        3,
        crate::disk::Disk::B,
        crate::direction::Direction::R,
    );
    assert_eq!(found_offset, Ok(Some(2)));

}

#[test]
fn test_find_valid_disk_in_direction_2() {
    use crate::disk::Disk::*;
    let board = super::Board {
        board: [
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, W, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ],
    };
    let found_offset = board.find_valid_disk_in_direction(
        2,
        3,
        crate::disk::Disk::B,
        crate::direction::Direction::R,
    );
    assert_eq!(found_offset, Ok(Some(2)));

}

#[test]
fn test_get_disk_with_directional_offset_1() {
    use crate::disk::Disk::*;
    let board = super::Board {
        board: [
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, W, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ],
    };
    let offset = board.get_disk_with_directional_offset(
        2,
        3,
        2,
        crate::direction::Direction::R,
    );
    assert_eq!(offset, Ok(crate::disk::Disk::B));
}

#[test]
fn test_get_disk_with_directional_offset_2() {
    use crate::disk::Disk::*;
    let board = super::Board {
        board: [
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, W, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ],
    };
    let offset = board.get_disk_with_directional_offset(
        0,
        0,
        2,
        crate::direction::Direction::L,
    );
    assert_eq!(offset, Err(crate::board::OutOfIndexError));
}

#[test]
fn test_get_disk() {
    use crate::disk::Disk::*;
    let board = super::Board {
        board: [
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, W, B, E, E, E],
            [E, E, E, B, W, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
            [E, E, E, E, E, E, E, E],
        ],
    };
    let offset = board.get_disk(
        3,
        3,
    );
    assert_eq!(offset, Ok(crate::disk::Disk::W));
}