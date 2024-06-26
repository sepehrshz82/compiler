use strum::EnumString;
use strum_macros::AsRefStr;

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, PartialEq, Eq, AsRefStr, Hash, EnumString, Default)]
pub enum TokenType {
    #[default]
    T_Bool,
    T_Break,
    T_Char,
    T_Continue,
    T_Else,
    T_False,
    T_For,
    T_If,
    T_Int,
    T_Print,
    T_Return,
    T_True,
    T_AOp_PL,
    T_AOp_MN,
    T_AOp_ML,
    T_AOp_DV,
    T_AOp_RM,
    T_ROp_L,
    T_ROp_G,
    T_ROp_LE,
    T_ROp_GE,
    T_ROp_NE,
    T_ROp_E,
    T_LOp_AND,
    T_LOp_OR,
    T_LOp_NOT,
    T_Assign,
    T_LP,
    T_RP,
    T_LC,
    T_RC,
    T_LB,
    T_RB,
    T_Semicolon,
    T_Comma,
    T_Id,
    T_String,
    T_Decimal,
    T_Hexadecimal,
    T_Character,
    T_Whitespace,
    T_Comment,
    ILLEGAL,
    T_Newline,
    End,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Token {
    pub token: TokenType,
    pub line: usize,
    pub literal: String,
    pub column: usize,
}
