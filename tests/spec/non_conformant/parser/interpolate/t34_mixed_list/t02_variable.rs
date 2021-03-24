//! Tests auto-converted from "sass-spec/spec/non_conformant/parser/interpolate/34_mixed_list/02_variable.hrx"

#[test]
fn test() {
    assert_eq!(
        crate::rsass(
            "$input: \"[\"\',foo   ,   \'\"]\"    \"bar\";\
            \n.result {\
            \n  output: $input;\
            \n  output: #{$input};\
            \n  output: \"[#{$input}]\";\
            \n  output: \"#{$input}\";\
            \n  output: \'#{$input}\';\
            \n  output: \"[\'#{$input}\']\";\
            \n}\
            \n"
        )
        .unwrap(),
        ".result {\
        \n  output: \"[\" \",foo   ,   \" \"]\" \"bar\";\
        \n  output: [ ,foo   ,    ] bar;\
        \n  output: \"[[ ,foo   ,    ] bar]\";\
        \n  output: \"[ ,foo   ,    ] bar\";\
        \n  output: \"[ ,foo   ,    ] bar\";\
        \n  output: \"[\'[ ,foo   ,    ] bar\']\";\
        \n}\
        \n"
    );
}