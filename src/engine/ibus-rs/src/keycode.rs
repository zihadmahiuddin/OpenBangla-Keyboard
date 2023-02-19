use std::collections::HashMap;

use ibus::keys::*;
use lazy_static::lazy_static;
use riti::keycodes::*;

lazy_static! {
    static ref KEYS: HashMap<i32, u16> = {
        let mut k = HashMap::new();

        // Begin Alphanumeric Zone
        k.insert(GRAVE, VC_GRAVE);
        k.insert(DEAD_GRAVE, VC_GRAVE);
        k.insert(ASCII_TILDE, VC_TILDE);

        k.insert(NUM_0, VC_0);
        k.insert(NUM_1, VC_1);
        k.insert(NUM_2, VC_2);
        k.insert(NUM_3, VC_3);
        k.insert(NUM_4, VC_4);
        k.insert(NUM_5, VC_5);
        k.insert(NUM_6, VC_6);
        k.insert(NUM_7, VC_7);
        k.insert(NUM_8, VC_8);
        k.insert(NUM_9, VC_9);

        k.insert(PAREN_RIGHT, VC_PAREN_RIGHT);
        k.insert(EXCLAM, VC_EXCLAIM);
        k.insert(AT, VC_AT);
        k.insert(NUMBER_SIGN, VC_HASH);
        k.insert(DOLLAR, VC_DOLLAR);
        k.insert(PERCENT, VC_PERCENT);
        k.insert(ASCII_CIRCUM, VC_CIRCUM);
        k.insert(AMPERSAND, VC_AMPERSAND);
        k.insert(ASTERISK, VC_ASTERISK);
        k.insert(PAREN_LEFT, VC_PAREN_LEFT);

        k.insert(MINUS, VC_MINUS);
        k.insert(UNDERSCORE, VC_UNDERSCORE);

        k.insert(EQUAL, VC_EQUALS);
        k.insert(PLUS, VC_PLUS);

        k.insert(UPPERCASE_A, VC_A_SHIFT);
        k.insert(UPPERCASE_B, VC_B_SHIFT);
        k.insert(UPPERCASE_C, VC_C_SHIFT);
        k.insert(UPPERCASE_D, VC_D_SHIFT);
        k.insert(UPPERCASE_E, VC_E_SHIFT);
        k.insert(UPPERCASE_F, VC_F_SHIFT);
        k.insert(UPPERCASE_G, VC_G_SHIFT);
        k.insert(UPPERCASE_H, VC_H_SHIFT);
        k.insert(UPPERCASE_I, VC_I_SHIFT);
        k.insert(UPPERCASE_J, VC_J_SHIFT);
        k.insert(UPPERCASE_K, VC_K_SHIFT);
        k.insert(UPPERCASE_L, VC_L_SHIFT);
        k.insert(UPPERCASE_M, VC_M_SHIFT);
        k.insert(UPPERCASE_N, VC_N_SHIFT);
        k.insert(UPPERCASE_O, VC_O_SHIFT);
        k.insert(UPPERCASE_P, VC_P_SHIFT);
        k.insert(UPPERCASE_Q, VC_Q_SHIFT);
        k.insert(UPPERCASE_R, VC_R_SHIFT);
        k.insert(UPPERCASE_S, VC_S_SHIFT);
        k.insert(UPPERCASE_T, VC_T_SHIFT);
        k.insert(UPPERCASE_U, VC_U_SHIFT);
        k.insert(UPPERCASE_V, VC_V_SHIFT);
        k.insert(UPPERCASE_W, VC_W_SHIFT);
        k.insert(UPPERCASE_X, VC_X_SHIFT);
        k.insert(UPPERCASE_Y, VC_Y_SHIFT);
        k.insert(UPPERCASE_Z, VC_Z_SHIFT);

        k.insert(LOWERCASE_A, VC_A);
        k.insert(LOWERCASE_B, VC_B);
        k.insert(LOWERCASE_C, VC_C);
        k.insert(LOWERCASE_D, VC_D);
        k.insert(LOWERCASE_E, VC_E);
        k.insert(LOWERCASE_F, VC_F);
        k.insert(LOWERCASE_G, VC_G);
        k.insert(LOWERCASE_H, VC_H);
        k.insert(LOWERCASE_I, VC_I);
        k.insert(LOWERCASE_J, VC_J);
        k.insert(LOWERCASE_K, VC_K);
        k.insert(LOWERCASE_L, VC_L);
        k.insert(LOWERCASE_M, VC_M);
        k.insert(LOWERCASE_N, VC_N);
        k.insert(LOWERCASE_O, VC_O);
        k.insert(LOWERCASE_P, VC_P);
        k.insert(LOWERCASE_Q, VC_Q);
        k.insert(LOWERCASE_R, VC_R);
        k.insert(LOWERCASE_S, VC_S);
        k.insert(LOWERCASE_T, VC_T);
        k.insert(LOWERCASE_U, VC_U);
        k.insert(LOWERCASE_V, VC_V);
        k.insert(LOWERCASE_W, VC_W);
        k.insert(LOWERCASE_X, VC_X);
        k.insert(LOWERCASE_Y, VC_Y);
        k.insert(LOWERCASE_Z, VC_Z);

        k.insert(BRACKET_LEFT, VC_BRACKET_LEFT);
        k.insert(BRACE_LEFT, VC_BRACE_LEFT);

        k.insert(BRACKET_RIGHT, VC_BRACKET_RIGHT);
        k.insert(BRACE_RIGHT, VC_BRACE_RIGHT);

        k.insert(BACKSLASH, VC_BACK_SLASH);
        k.insert(BAR, VC_BAR);

        k.insert(SLASH, VC_SLASH);
        k.insert(QUESTION, VC_QUESTION);

        k.insert(SEMICOLON, VC_SEMICOLON);
        k.insert(COLON, VC_COLON);

        k.insert(COMMA, VC_COMMA);
        k.insert(LESS, VC_LESS);

        k.insert(PERIOD, VC_PERIOD);
        k.insert(GREATER, VC_GREATER);

        k.insert(APOSTROPHE, VC_APOSTROPHE);
        k.insert(QUOTE_DBL, VC_QUOTE);

        // Begin Numeric Zone
        k.insert(KP_DIVIDE, VC_KP_DIVIDE);
        k.insert(KP_MULTIPLY, VC_KP_MULTIPLY);
        k.insert(KP_SUBTRACT, VC_KP_SUBTRACT);
        k.insert(KP_EQUAL, VC_KP_EQUALS);
        k.insert(KP_ADD, VC_KP_ADD);
        k.insert(KP_ENTER, VC_KP_ENTER);
        k.insert(KP_DECIMAL, VC_KP_DECIMAL);

        k.insert(KP_1, VC_KP_1);
        k.insert(KP_2, VC_KP_2);
        k.insert(KP_3, VC_KP_3);
        k.insert(KP_4, VC_KP_4);
        k.insert(KP_5, VC_KP_5);
        k.insert(KP_6, VC_KP_6);
        k.insert(KP_7, VC_KP_7);
        k.insert(KP_8, VC_KP_8);
        k.insert(KP_9, VC_KP_9);
        k.insert(KP_0, VC_KP_0);
        // End Numeric Zone

        k
    };
}

pub fn ibus_to_riti_keycode(ibus_keycode: i32) -> Option<u16> {
    KEYS.get(&ibus_keycode).copied()
}
