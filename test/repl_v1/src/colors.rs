const RESET: &str = "\x1b[0m";

const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
/// Works with dim too
const BOLD_END: &str = "\x1b[22m";

const ITALIC: &str = "\x1b[3, 23m";
const ITALIC_END: &str = "\x1b[3, 23m";

const UNDERLINE       : &str = "\x1b[4m";
const DOUBLE_UNDERLINE: &str = "\x1b[21m";
const UNDERLINE_END   : &str = "\x1b[24m";

const BLINK: &str = "\x1b[5m";
const BLINK_END: &str = "\x1b[25m";

const INVERSE: &str = "\x1b[7m";
const INVERSE_END: &str = "\x1b[27m";

const HIDDEN: &str = "\x1b[8m";
const HIDDEN_END: &str = "\x1b[28m";

const STRIKE_THROUGH: &str = "\x1b[9m";
const STRIKE_THROUGH_END: &str = "\x1b[29m";

const BLACK: &str = "\x1b[30, 39m";
const RED: &str = "\x1b[31, 39m";
const GREEN: &str = "\x1b[32, 39m";
const YELLOW: &str = "\x1b[33, 39m";
const BLUE: &str = "\x1b[34, 39m";
const MAGENTA: &str = "\x1b[35, 39m";
const CYAN: &str = "\x1b[36, 39m";
const WHITE: &str = "\x1b[37, 39m";
const GRAY: &str = "\x1b[90, 39m";
const RED_BRIGHT: &str = "\x1b[91, 39m";
const GREEN_BRIGHT: &str = "\x1b[92, 39m";
const YELLOW_BRIGHT: &str = "\x1b[93, 39m";
const BLUE_BRIGHT: &str = "\x1b[94, 39m";
const MAGENTA_BRIGHT: &str = "\x1b[95, 39m";
const CYAN_BRIGHT: &str = "\x1b[96, 39m";
const WHITE_BRIGHT: &str = "\x1b[97, 39m";

const BG_BLACK: &str = "\x1b[40, 49m";
const BG_RED: &str = "\x1b[41, 49m";
const BG_GREEN: &str = "\x1b[42, 49m";
const BG_YELLOW: &str = "\x1b[43, 49m";
const BG_BLUE: &str = "\x1b[44, 49m";
const BG_MAGENTA: &str = "\x1b[45, 49m";
const BG_CYAN: &str = "\x1b[46, 49m";
const BG_WHITE: &str = "\x1b[47, 49m";
const BG_GRAY: &str = "\x1b[100, 49m";
const BG_RED_BRIGHT: &str = "\x1b[101, 49m";
const BG_GREEN_BRIGHT: &str = "\x1b[102, 49m";
const BG_YELLOW_BRIGHT: &str = "\x1b[103, 49m";
const BG_BLUE_BRIGHT: &str = "\x1b[104, 49m";
const BG_MAGENTA_BRIGHT: &str = "\x1b[105, 49m";
const BG_CYAN_BRIGHT: &str = "\x1b[106, 49m";
const BG_WHITE_BRIGHT: &str = "\x1b[107, 49m";

const FRAMED: &str = "\x1b[51, 54m";
const FRAMED_END: &str = "\x1b[51, 54m";

const OVERLINED: &str = "\x1b[53, 55m";
const OVERLINED_END: &str = "\x1b[53, 55m";
