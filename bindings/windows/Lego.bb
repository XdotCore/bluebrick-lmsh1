functions
mod Lego

cdecl RunGame(cmd_line_arg_count: i32, cmd_line_args: ptr string) i32 = 0x185410

cdecl AddToCoins(coins: ptr u64, to_add: u64, mult: i32, round_to_10s: bool) void = 0x7E1070

cdecl CreateWindow(hinstance: HINSTANCE, param_2: i32, icon: string) i32 = 0x2e2210

// TODO: find which class this belongs to
thiscall CreateAdditionalWindow(this: ptr void, window_name: string, x: i32, y: i32, width: i32, height: i32) HWND = 0x2e0760
