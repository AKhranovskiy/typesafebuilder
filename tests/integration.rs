// #![allow(incomplete_features)]
// #![feature(generic_const_exprs)]

// struct Boolean<const B: bool>();
// trait True {}
// impl True for Boolean<true> {}
// trait False {}
// impl False for Boolean<false> {}

// struct Config {
//     path: String,
//     size: usize,
//     validate: bool,
//     // limit: Option<u32>,
// }

// #[derive(Default)]
// struct ConfigBuilder<
//     const PATH: bool = false,
//     const SIZE: bool = false,
//     const VALIDATE: bool = false,
// > {
//     path: Option<String>,
//     size: Option<usize>,
//     validate: Option<bool>,
//     // limit: Option<Option<u32>>,
// }

// impl<const SIZE: bool, const VALIDATE: bool> ConfigBuilder<false, SIZE, VALIDATE>
// where
//     Boolean<{ SIZE & VALIDATE }>: False,
// {
//     fn path(self, value: String) -> ConfigBuilder<true, SIZE, VALIDATE> {
//         ConfigBuilder {
//             path: Some(value),
//             size: self.size,
//             validate: self.validate,
//         }
//     }
// }

// impl ConfigBuilder<false, true, true> {
//     fn path(self, value: String) -> Config {
//         Config {
//             path: value,
//             size: self.size.unwrap(),
//             validate: self.validate.unwrap(),
//         }
//     }
// }

// // impl ConfigBuilder<false, false> {
// //     fn size(self, value: usize) -> ConfigBuilder<false, true> {
// //         ConfigBuilder {
// //             path: self.path,
// //             size: Some(value),
// //             validate: self.validate,
// //         }
// //     }
// // }

// // impl ConfigBuilder<true, false> {
// //     fn size(self, value: usize) -> Config {
// //         Config {
// //             path: self.path.unwrap(),
// //             size: value,
// //             validate: self.validate.unwrap(),
// //         }
// //     }
// // }

// // impl<const PATH: bool, const SIZE: bool> ConfigBuilder<PATH, SIZE, false>
// // where
// //     crate::Boolean<{ PATH & SIZE }>: False,
// // {
// //     fn validate(self, value: bool) -> ConfigBuilder<PATH, SIZE, true> {
// //         ConfigBuilder {
// //             path: self.path,
// //             size: self.size,
// //             validate: Some(value),
// //         }
// //     }
// // }
// // impl ConfigBuilder<true, true, false> {
// //     fn validate(self, value: bool) -> Config {
// //         Config {
// //             path: self.path.unwrap(),
// //             size: self.size.unwrap(),
// //             validate: value,
// //         }
// //     }
// // }

// // #[test]
// // fn test() {
// //     let a = ConfigBuilder::<false, false>::default()
// //         .size(5)
// //         .path("test".to_string())
// //         .validate(true);
// // }
// // //
// // //impl<const PATH: bool, /*const SIZE: bool,*/ const VALIDATE: bool, const LIMIT: bool>
// // //    ConfigBuilder<PATH, false, VALIDATE, LIMIT>
// // //{
// // //    pub fn size(self, value: usize) -> ConfigBuilder<PATH, true, VALIDATE, LIMIT> {
// // //        ConfigBuilder {
// // //            path: self.path,
// // //            size: value.into(),
// // //            validate: self.validate,
// // //            limit: self.limit,
// // //        }
// // //    }
// // //}
// // //
// // //impl<const PATH: bool, const SIZE: bool, /*const VALIDATE: bool,*/ const LIMIT: bool>
// // //    ConfigBuilder<PATH, SIZE, false, LIMIT>
// // //{
// // //    pub fn validate(self, value: bool) -> ConfigBuilder<PATH, SIZE, true, LIMIT> {
// // //        ConfigBuilder {
// // //            path: self.path,
// // //            size: self.size,
// // //            validate: value.into(),
// // //            limit: self.limit,
// // //        }
// // //    }
// // //}
// // //
// // //impl<const PATH: bool, const SIZE: bool, const VALIDATE: bool /*const LIMIT: bool*/>
// // //    ConfigBuilder<PATH, SIZE, VALIDATE, false>
// // //{
// // //    pub fn limit(self, value: u32) -> ConfigBuilder<PATH, SIZE, VALIDATE, true> {
// // //        ConfigBuilder {
// // //            path: self.path,
// // //            size: self.size,
// // //            validate: self.validate,
// // //            limit: Some(value).into(),
// // //        }
// // //    }
// // //}
// // //
// // //impl</*const PATH: bool, const SIZE: bool, const VALIDATE: bool,*/ const LIMIT: bool>
// // //    ConfigBuilder<true, true, true, LIMIT>
// // //{
// // //    pub fn build(self) -> Config {
// // //        Config {
// // //            path: self.path.unwrap(),
// // //            size: self.size.unwrap(),
// // //            validate: self.validate.unwrap(),
// // //            limit: self.limit.unwrap_or_default(),
// // //        }
// // //    }
// // //}
