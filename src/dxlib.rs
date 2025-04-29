use crate::dxlib_constants::*;
use crate::dxlib_types::*;
use dxlib_ffi_macro::dxlib_gen;
// =======================================================
// dxlib_gen! {
//  [libname],
//  [signature*]
// }
// CInt,CChar等その他C用のtype宣言、構造体はdxlib_typesに記述されている
// DxLib特有の定数は、dxlib_constantsに記述されている
// 基本的にDxLibの関数シグネチャ通りに渡すことが可能
// &strを指定した場合のみ、*const i8として変換されて渡される
// =======================================================
dxlib_gen! {
    // ライブラリ名
    "DxLib_x64",
    // ライブラリの初期化
    fn DxLib_Init() -> CInt,
    // ライブラリ使用の終了関数
    fn DxLib_End() -> CInt,
    // ウインドウズのメッセージを処理する
    fn ProcessMessage() -> CInt,
    // フリップ関数、画面の裏ページ(普段は表示されていない)の内容を表ページ(普段表示されている)に反映する
    fn ScreenFlip() -> CInt,
    // 描画先グラフィック領域の指定
    fn SetDrawScreen(draw_screen: CInt) -> CInt,
    fn ClearDrawScreen(clear_rect: *mut RECT) -> CInt,
    // ウインドウモード・フルスクリーンモードの変更を行う
    fn ChangeWindowMode(flag:CInt) -> CInt,
    // ウインドウのタイトルを変更する
    fn SetMainWindowText(window_text: &str) -> CInt,
    // キーの入力待ち
    fn WaitKey() -> CInt,
    // キーボードによる文字列の入力
    fn KeyInputString(
        x: CInt,
        y: CInt,
        char_max_length: CInt,
        str_buffer: *mut CChar,
        cancel_valid_flag: CInt,
    ) -> CInt,
    // 文字列の引数の文字コードを設定する
    fn SetUseCharCodeFormat(
        char_code_format: CInt,
    ) -> CInt,
    // 色コードを取得する
    fn GetColor(Red: CInt, Green: CInt, Blue: CInt) -> CInt,
    // 文字列を描画する
    fn DrawString(x: CInt, y: CInt, string: &str, Color: CInt) -> CInt,
}
