use libquark::prelude::*;

#[cfg(test)]
mod quark_lib {
    use super::*;

    // #[test]
    // fn test_valid_initialization() {
    //     assert_html_exists();

    //     let config = QuarkConfig::new().frontend(BASIC_EXAMPLE);

    //     let result = Quark::new(config);
    //     assert!(result.is_ok());
    // }

    // #[test]
    // fn test_invalid_path() {
    //     let config = QuarkConfig::new().frontend("invalid");

    //     let result = Quark::new(config);
    //     assert!(matches!(result, Err(QuarkError::PathError)));
    // }

    // #[test]
    // fn test_binding() {
    //     assert_html_exists();

    //     let config = QuarkConfig::new().frontend(BASIC_EXAMPLE);

    //     let mut quark = Quark::new(config).expect("Failed to create an instance");
    //     quark.bind("test_function", |_, _| {});
    // }

    // #[test]
    // fn eval() {
    //     assert_html_exists();

    //     let config = QuarkConfig::new().frontend(BASIC_EXAMPLE);

    //     let mut quark = Quark::new(config).expect("Failed to create Quark instance");
    //     quark.eval("console.log('test');");
    // }

    #[test]
    fn custom_config() {
        let config = QuarkConfig::new()
            .title("QuarkTestWindowConfig")
            .width(1024)
            .height(768)
            .resizable(SizeHint::FIXED);

        let result = Quark::new(config);
        assert!(result.is_ok());
    }

    // #[test]
    // fn path() {
    //     assert_html_exists();

    //     let current_dir = std::env::current_dir().unwrap();
    //     let full_path = current_dir.join(BASIC_EXAMPLE).join("index.html");
    //     assert!(full_path.exists());
    // }

    #[test]
    fn errors() {
        let errors = [
            QuarkError::FrontendPathMissing,
            QuarkError::IncludeDirCouldntConvertToUTF8,
            QuarkError::ServerPortIsntAvailable,
            QuarkError::ServerError,
        ];

        for error in &errors {
            let debug_str = format!("{:?}", error);
            assert!(!debug_str.is_empty());
        }
    }

    // #[test]
    // fn test_multiple_bindings() {
    //     assert_html_exists();

    //     let config = QuarkConfig::new().frontend(BASIC_EXAMPLE);

    //     let mut quark = Quark::new(config).expect("Failed to create Quark instance");

    //     quark.bind("function1", |_, _| {});
    //     quark.bind("function2", |_, _| {});
    //     quark.bind("function3", |_, _| {});
    // }

    // #[test]
    // fn test_multiple_evals() {
    //     assert_html_exists();

    //     let config = QuarkConfig::new().frontend(BASIC_EXAMPLE);

    //     let mut quark = Quark::new(config).expect("Failed to create Quark instance");

    //     // Test that multiple evals don't panic
    //     quark.eval("console.log('test1');");
    //     quark.eval("console.log('test2');");
    //     quark.eval("console.log('test3');");
    // }
}
