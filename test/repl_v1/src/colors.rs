pub const RESET: &str = "\x1b[0m";

pub const BOLD: &str = "\x1b[1m";
/// Set an specific color using 2;r;g;b syntax (maybe)
pub const DIM: &str = "\x1b[2m";
/// Works with dim too
pub const BOLD_END: &str = "\x1b[22m";

pub const ITALIC    : &str = "\x1b[3, 23m";
pub const ITALIC_END: &str = "\x1b[3, 23m";

pub const UNDERLINE       : &str = "\x1b[4m";
pub const DOUBLE_UNDERLINE: &str = "\x1b[21m";
pub const UNDERLINE_END   : &str = "\x1b[24m";

pub const BLINK    : &str = "\x1b[5m";
pub const BLINK_END: &str = "\x1b[25m";

pub const INVERSE    : &str = "\x1b[7m";
pub const INVERSE_END: &str = "\x1b[27m";

pub const HIDDEN    : &str = "\x1b[8m";
pub const HIDDEN_END: &str = "\x1b[28m";

pub const STRIKE_THROUGH    : &str = "\x1b[9m";
pub const STRIKE_THROUGH_END: &str = "\x1b[29m";

pub const BLACK  : &str = "\x1b[30m";
pub const RED    : &str = "\x1b[31m";
pub const GREEN  : &str = "\x1b[32m";
pub const YELLOW : &str = "\x1b[33m";
pub const BLUE   : &str = "\x1b[34m";
pub const MAGENTA: &str = "\x1b[35m";
pub const CYAN   : &str = "\x1b[36m";
pub const WHITE  : &str = "\x1b[37m";
pub const GRAY   : &str = "\x1b[90m";
pub const RED_BRIGHT    : &str = "\x1b[91m";
pub const GREEN_BRIGHT  : &str = "\x1b[92m";
pub const YELLOW_BRIGHT : &str = "\x1b[93m";
pub const BLUE_BRIGHT   : &str = "\x1b[94m";
pub const MAGENTA_BRIGHT: &str = "\x1b[95m";
pub const CYAN_BRIGHT   : &str = "\x1b[96m";
pub const WHITE_BRIGHT  : &str = "\x1b[97m";
pub const COLOR_END: &str = "\x1b[39m";

pub const BG_BLACK  : &str = "\x1b[40m";
pub const BG_RED    : &str = "\x1b[41m";
pub const BG_GREEN  : &str = "\x1b[42m";
pub const BG_YELLOW : &str = "\x1b[43m";
pub const BG_BLUE   : &str = "\x1b[44m";
pub const BG_MAGENTA: &str = "\x1b[45m";
pub const BG_CYAN   : &str = "\x1b[46m";
pub const BG_WHITE  : &str = "\x1b[47m";
pub const BG_GRAY   : &str = "\x1b[100m";
pub const BG_RED_BRIGHT    : &str = "\x1b[101m";
pub const BG_GREEN_BRIGHT  : &str = "\x1b[102m";
pub const BG_YELLOW_BRIGHT : &str = "\x1b[103m";
pub const BG_BLUE_BRIGHT   : &str = "\x1b[104m";
pub const BG_MAGENTA_BRIGHT: &str = "\x1b[105m";
pub const BG_CYAN_BRIGHT   : &str = "\x1b[106m";
pub const BG_WHITE_BRIGHT  : &str = "\x1b[107m";
pub const BG_END: &str = "\x1b[49m";

pub const FRAMED    : &str = "\x1b[51, 54m";
pub const FRAMED_END: &str = "\x1b[51, 54m";

pub const OVERLINED    : &str = "\x1b[53, 55m";
pub const OVERLINED_END: &str = "\x1b[53, 55m";
