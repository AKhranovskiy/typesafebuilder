use typesafe::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Eq, TypedBuilder)]
struct Config {
    validate: bool,
    // path: String,
    // size: usize,
    // limit: Option<u32>,
}

//impl Config {
//    pub fn build() -> ConfigBuilder {
//        ConfigBuilder {
//            path: None,
//            size: None,
//            validate: None,
//            limit: None,
//        }
//    }
//}
//
//struct ConfigBuilder<
//    const PATH: bool = false,
//    const SIZE: bool = false,
//    const VALIDATE: bool = false,
//    const LIMIT: bool = false,
//> {
//    path: Option<String>,
//    size: Option<usize>,
//    validate: Option<bool>,
//    limit: Option<Option<u32>>,
//}
//
//impl<
//        /*const PATH: bool,*/ const SIZE: bool,
//        const VALIDATE: bool, /*const LIMIT: bool*/
//    > ConfigBuilder<false, SIZE, VALIDATE, false>
//{
//    pub fn path(self, value: String) -> ConfigBuilder<true, SIZE, VALIDATE, false> {
//        ConfigBuilder {
//            path: Some(value),
//            size: self.size,
//            validate: self.validate,
//            limit: self.limit,
//        }
//    }
//}
//
//impl ConfigBuilder<false, true, true, true> {
//    pub fn path(self, value: String) -> Config {
//        Config {
//            path: value,
//            size: self.size.unwrap(),
//            validate: self.validate.unwrap(),
//            limit: self.limit.unwrap_or_default(),
//        }
//    }
//}
//
//impl<const PATH: bool, /*const SIZE: bool,*/ const VALIDATE: bool, const LIMIT: bool>
//    ConfigBuilder<PATH, false, VALIDATE, LIMIT>
//{
//    pub fn size(self, value: usize) -> ConfigBuilder<PATH, true, VALIDATE, LIMIT> {
//        ConfigBuilder {
//            path: self.path,
//            size: value.into(),
//            validate: self.validate,
//            limit: self.limit,
//        }
//    }
//}
//
//impl<const PATH: bool, const SIZE: bool, /*const VALIDATE: bool,*/ const LIMIT: bool>
//    ConfigBuilder<PATH, SIZE, false, LIMIT>
//{
//    pub fn validate(self, value: bool) -> ConfigBuilder<PATH, SIZE, true, LIMIT> {
//        ConfigBuilder {
//            path: self.path,
//            size: self.size,
//            validate: value.into(),
//            limit: self.limit,
//        }
//    }
//}
//
//impl<const PATH: bool, const SIZE: bool, const VALIDATE: bool /*const LIMIT: bool*/>
//    ConfigBuilder<PATH, SIZE, VALIDATE, false>
//{
//    pub fn limit(self, value: u32) -> ConfigBuilder<PATH, SIZE, VALIDATE, true> {
//        ConfigBuilder {
//            path: self.path,
//            size: self.size,
//            validate: self.validate,
//            limit: Some(value).into(),
//        }
//    }
//}
//
//impl</*const PATH: bool, const SIZE: bool, const VALIDATE: bool,*/ const LIMIT: bool>
//    ConfigBuilder<true, true, true, LIMIT>
//{
//    pub fn build(self) -> Config {
//        Config {
//            path: self.path.unwrap(),
//            size: self.size.unwrap(),
//            validate: self.validate.unwrap(),
//            limit: self.limit.unwrap_or_default(),
//        }
//    }
//}

#[test]
fn with_limit() {
    let config = Config::build().validate(true);
    //     .size(25)
    //     .validate(false)
    //     .limit(100)
    //     .path("path".into());

    assert_eq!(
        config,
        Config {
            validate: true,
            //         path: "path".into(),
            //         size: 25,
            //         limit: 100.into()
        }
    );
}
