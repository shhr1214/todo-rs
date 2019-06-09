//! # TaskRepository の実装ライブラリ
//!
//! InMmeory なやつとか MySQL をバックエンドに持つやつとか実装する。

pub mod inmemory;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
