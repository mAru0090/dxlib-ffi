pub mod dxlib;
pub mod dxlib_constants;
pub mod dxlib_types;

pub use dxlib_ffi_macro::dxlib_gen;

mod tests {
    use crate::dxlib::*;
    use crate::dxlib_constants::*;
    use crate::dxlib_types::*;
    use anyhow::Result as R;
    const MAX_LENGTH: usize = 1024;
    #[test]
    fn test_dxlib_1() -> R<()> {
        let mut buffer: Vec<CChar> = vec![0; MAX_LENGTH]; // 最大長のバッファを確保

        // 呼び出し先関数に渡す前に、ポインタとして扱う
        let buffer_ptr = buffer.as_mut_ptr() as *mut CChar;
        SetUseCharCodeFormat(DX_CHARCODEFORMAT_UTF8);
        SetMainWindowText("DxLib and Rust draw Window! DxLibとRustでウィンドウ表示!");
        ChangeWindowMode(TRUE);
        DxLib_Init();
        SetDrawScreen(DX_SCREEN_BACK);
        KeyInputString(100, 100, MAX_LENGTH as i32 - 1, buffer_ptr, 0);

        // i8をu8に変換
        let u8_buffer: Vec<u8> = buffer.into_iter().map(|x| x as u8).collect();
        println!("{}", String::from_utf8(u8_buffer)?);
        let mut rect = RECT {
            left: -1,
            right: -1,
            top: -1,
            bottom: -1,
        };
        let mut x = 0;
        let mut y = 10;
        while ScreenFlip() == 0 && ClearDrawScreen(&mut rect) == 0 && ProcessMessage() == 0 {
            DrawString(x, y, "hello world! こんにちは 世界!", GetColor(255, 255, 255));
            x += 10;
        }
        DxLib_End();
        Ok(())
    }
}
