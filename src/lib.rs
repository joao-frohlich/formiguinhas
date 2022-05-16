pub mod ant;
pub mod heterogeneous_ant;
pub mod board;
pub mod heterogeneous_board;
pub mod cell;
pub mod heterogeneous_cell;
pub mod params;
pub mod data_item;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
