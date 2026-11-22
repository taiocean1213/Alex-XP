//! The unpacked editor extension is part of the project and is not built by Cargo.
//! These checks keep its manifest, grammar, and snippets aligned with the macros.

use std::path::PathBuf;

fn editor_file(name: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../editors/vscode")
        .join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("read {}: {err}", path.display()))
}

fn assert_json_balanced(text: &str) {
    let mut stack = Vec::new();
    let mut in_string = false;
    let mut escape = false;
    for c in text.chars() {
        if in_string {
            if escape {
                escape = false;
                continue;
            }
            match c {
                '\\' => escape = true,
                '"' => in_string = false,
                _ => {}
            }
            continue;
        }
        match c {
            '"' => in_string = true,
            '{' | '[' => stack.push(c),
            '}' => assert_eq!(stack.pop(), Some('{'), "unbalanced json object"),
            ']' => assert_eq!(stack.pop(), Some('['), "unbalanced json array"),
            _ => {}
        }
    }
    assert!(!in_string, "unterminated json string");
    assert!(stack.is_empty(), "unbalanced json");
}

#[test]
fn extension_manifest_points_at_real_files() {
    let manifest = editor_file("package.json");
    assert_json_balanced(&manifest);
    assert!(manifest.contains("alex-xp.injection.rust"));
    assert!(manifest.contains("./syntaxes/alex-xp.injection.json"));
    assert!(manifest.contains("./snippets/alex-xp.json"));
    assert!(manifest.contains("\"language\": \"rust\""));

    let grammar = editor_file("syntaxes/alex-xp.injection.json");
    let snippets = editor_file("snippets/alex-xp.json");
    assert_json_balanced(&grammar);
    assert_json_balanced(&snippets);
    assert!(grammar.contains("\"injectionSelector\": \"L:source.rust -comment -string\""));
    assert!(grammar.contains("no_mut_ptr|mut_ptr"));
}

#[test]
fn grammar_and_snippets_name_every_public_macro() {
    let grammar = editor_file("syntaxes/alex-xp.injection.json");
    let snippets = editor_file("snippets/alex-xp.json");

    let macros = [
        "interface",
        "class",
        "obj",
        "iface",
        "declare_immut_var",
        "declare_mut_var",
        "declare_const",
        "declare_var",
        "ownership_assign",
        "copy_assign",
        "unique_writable_lend",
        "read_only_lend",
        "deref_access",
        "deref_mutate",
        "scope_block",
        "reassign_ownership",
        "reassign_copy",
        "delete_var",
        "perform_op",
        "print_value",
        "drill_mut",
        "drill_read",
        "auto_let",
        "auto_let_mut",
        "auto_give",
        "auto_set",
        "auto_share",
        "auto_edit",
        "auto_own",
        "auto_box",
        "auto_fn",
        "val",
        "var",
        "set",
        "give",
        "add",
        "show",
        "def",
        "heap",
        "s",
    ];

    for name in macros {
        let mentioned = grammar.contains(&format!("({name}|"))
            || grammar.contains(&format!("|{name}|"))
            || grammar.contains(&format!("|{name})"));
        assert!(mentioned, "grammar does not mention {name}");
        let prefix = if name == "s" { "s!" } else { name };
        assert!(
            snippets.contains(&format!("\"prefix\": \"{prefix}\"")),
            "snippets have no prefix {prefix}"
        );
    }
}
