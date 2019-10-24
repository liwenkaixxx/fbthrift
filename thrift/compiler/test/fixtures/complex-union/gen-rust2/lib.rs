// @generated by Thrift. This file is probably not the place you want to edit!

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

extern crate self as module;

pub use self::errors::*;
pub use self::types::*;

pub mod types {
    use fbthrift::{
        Deserialize, GetTType, ProtocolReader, ProtocolWriter, Serialize, TType,
    };

    pub type containerTypedef = std::collections::BTreeMap<i16, String>;

    #[derive(Clone, Debug, PartialEq)]
    pub struct ComplexUnion {
        pub intValue: i64,
        pub stringValue: String,
        pub intListValue: Vec<i64>,
        pub stringListValue: Vec<String>,
        pub typedefValue: std::collections::BTreeMap<i16, String>module::types::containerTypedef,
        pub stringRef: String,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ListUnion {
        pub intListValue: Vec<i64>,
        pub stringListValue: Vec<String>,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct DataUnion {
        pub binaryData: Vec<u8>,
        pub stringData: String,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct Val {
        pub strVal: String,
        pub intVal: i32,
        pub typedefValue: std::collections::BTreeMap<i16, String>module::types::containerTypedef,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct ValUnion {
        pub v1: module::types::Val,
        pub v2: module::types::Val,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct VirtualComplexUnion {
        pub thingOne: String,
        pub thingTwo: String,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct NonCopyableStruct {
        pub num: i64,
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct NonCopyableUnion {
        pub s: module::types::NonCopyableStruct,
    }

    impl Default for ComplexUnion {
        fn default() -> Self {
            Self {
                intValue: Default::default(),
                stringValue: Default::default(),
                intListValue: Default::default(),
                stringListValue: Default::default(),
                typedefValue: Default::default(),
                stringRef: Default::default(),
            }
        }
    }

    impl GetTType for ComplexUnion {
        const TTYPE: TType = TType::Struct;
    }

    impl<'a, P: ProtocolWriter> Serialize<P> for &'a ComplexUnion {
        fn write(self, p: &mut P) {
            p.write_struct_begin("ComplexUnion");
            p.write_field_begin("intValue", TType::I64, 1);
            Serialize::write(&self.intValue, p);
            p.write_field_end();
            p.write_field_begin("stringValue", TType::String, 5);
            Serialize::write(&self.stringValue, p);
            p.write_field_end();
            p.write_field_begin("intListValue", TType::List, 2);
            Serialize::write(&self.intListValue, p);
            p.write_field_end();
            p.write_field_begin("stringListValue", TType::List, 3);
            Serialize::write(&self.stringListValue, p);
            p.write_field_end();
            p.write_field_begin("typedefValue", TType::MapMap, 9);
            Serialize::write(&self.typedefValue, p);
            p.write_field_end();
            p.write_field_begin("stringRef", TType::String, 14);
            Serialize::write(&self.stringRef, p);
            p.write_field_end();
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P: ProtocolReader> Deserialize<P> for ComplexUnion {
        fn read(p: &mut P) -> failure::Fallible<Self> {
            let mut field_intValue = None;
            let mut field_stringValue = None;
            let mut field_intListValue = None;
            let mut field_stringListValue = None;
            let mut field_typedefValue = None;
            let mut field_stringRef = None;
            let _ = p.read_struct_begin(|_| ())?;
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| ())?;
                match (fty, fid as i32) {
                    (TType::Stop, _) => break,
                    (TType::I64, 1) => field_intValue = Some(Deserialize::read(p)?),
                    (TType::String, 5) => field_stringValue = Some(Deserialize::read(p)?),
                    (TType::List, 2) => field_intListValue = Some(Deserialize::read(p)?),
                    (TType::List, 3) => field_stringListValue = Some(Deserialize::read(p)?),
                    (TType::MapMap, 9) => field_typedefValue = Some(Deserialize::read(p)?),
                    (TType::String, 14) => field_stringRef = Some(Deserialize::read(p)?),
                    (fty, _) => p.skip(fty)?,
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            Ok(Self {
                intValue: field_intValue.unwrap_or_default(),
                stringValue: field_stringValue.unwrap_or_default(),
                intListValue: field_intListValue.unwrap_or_default(),
                stringListValue: field_stringListValue.unwrap_or_default(),
                typedefValue: field_typedefValue.unwrap_or_default(),
                stringRef: field_stringRef.unwrap_or_default(),
            })
        }
    }

    impl Default for ListUnion {
        fn default() -> Self {
            Self {
                intListValue: Default::default(),
                stringListValue: Default::default(),
            }
        }
    }

    impl GetTType for ListUnion {
        const TTYPE: TType = TType::Struct;
    }

    impl<'a, P: ProtocolWriter> Serialize<P> for &'a ListUnion {
        fn write(self, p: &mut P) {
            p.write_struct_begin("ListUnion");
            p.write_field_begin("intListValue", TType::List, 2);
            Serialize::write(&self.intListValue, p);
            p.write_field_end();
            p.write_field_begin("stringListValue", TType::List, 3);
            Serialize::write(&self.stringListValue, p);
            p.write_field_end();
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P: ProtocolReader> Deserialize<P> for ListUnion {
        fn read(p: &mut P) -> failure::Fallible<Self> {
            let mut field_intListValue = None;
            let mut field_stringListValue = None;
            let _ = p.read_struct_begin(|_| ())?;
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| ())?;
                match (fty, fid as i32) {
                    (TType::Stop, _) => break,
                    (TType::List, 2) => field_intListValue = Some(Deserialize::read(p)?),
                    (TType::List, 3) => field_stringListValue = Some(Deserialize::read(p)?),
                    (fty, _) => p.skip(fty)?,
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            Ok(Self {
                intListValue: field_intListValue.unwrap_or_default(),
                stringListValue: field_stringListValue.unwrap_or_default(),
            })
        }
    }

    impl Default for DataUnion {
        fn default() -> Self {
            Self {
                binaryData: Default::default(),
                stringData: Default::default(),
            }
        }
    }

    impl GetTType for DataUnion {
        const TTYPE: TType = TType::Struct;
    }

    impl<'a, P: ProtocolWriter> Serialize<P> for &'a DataUnion {
        fn write(self, p: &mut P) {
            p.write_struct_begin("DataUnion");
            p.write_field_begin("binaryData", TType::String, 1);
            Serialize::write(&self.binaryData, p);
            p.write_field_end();
            p.write_field_begin("stringData", TType::String, 2);
            Serialize::write(&self.stringData, p);
            p.write_field_end();
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P: ProtocolReader> Deserialize<P> for DataUnion {
        fn read(p: &mut P) -> failure::Fallible<Self> {
            let mut field_binaryData = None;
            let mut field_stringData = None;
            let _ = p.read_struct_begin(|_| ())?;
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| ())?;
                match (fty, fid as i32) {
                    (TType::Stop, _) => break,
                    (TType::String, 1) => field_binaryData = Some(Deserialize::read(p)?),
                    (TType::String, 2) => field_stringData = Some(Deserialize::read(p)?),
                    (fty, _) => p.skip(fty)?,
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            Ok(Self {
                binaryData: field_binaryData.unwrap_or_default(),
                stringData: field_stringData.unwrap_or_default(),
            })
        }
    }

    impl Default for Val {
        fn default() -> Self {
            Self {
                strVal: Default::default(),
                intVal: Default::default(),
                typedefValue: Default::default(),
            }
        }
    }

    impl GetTType for Val {
        const TTYPE: TType = TType::Struct;
    }

    impl<'a, P: ProtocolWriter> Serialize<P> for &'a Val {
        fn write(self, p: &mut P) {
            p.write_struct_begin("Val");
            p.write_field_begin("strVal", TType::String, 1);
            Serialize::write(&self.strVal, p);
            p.write_field_end();
            p.write_field_begin("intVal", TType::I32, 2);
            Serialize::write(&self.intVal, p);
            p.write_field_end();
            p.write_field_begin("typedefValue", TType::MapMap, 9);
            Serialize::write(&self.typedefValue, p);
            p.write_field_end();
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P: ProtocolReader> Deserialize<P> for Val {
        fn read(p: &mut P) -> failure::Fallible<Self> {
            let mut field_strVal = None;
            let mut field_intVal = None;
            let mut field_typedefValue = None;
            let _ = p.read_struct_begin(|_| ())?;
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| ())?;
                match (fty, fid as i32) {
                    (TType::Stop, _) => break,
                    (TType::String, 1) => field_strVal = Some(Deserialize::read(p)?),
                    (TType::I32, 2) => field_intVal = Some(Deserialize::read(p)?),
                    (TType::MapMap, 9) => field_typedefValue = Some(Deserialize::read(p)?),
                    (fty, _) => p.skip(fty)?,
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            Ok(Self {
                strVal: field_strVal.unwrap_or_default(),
                intVal: field_intVal.unwrap_or_default(),
                typedefValue: field_typedefValue.unwrap_or_default(),
            })
        }
    }

    impl Default for ValUnion {
        fn default() -> Self {
            Self {
                v1: Default::default(),
                v2: Default::default(),
            }
        }
    }

    impl GetTType for ValUnion {
        const TTYPE: TType = TType::Struct;
    }

    impl<'a, P: ProtocolWriter> Serialize<P> for &'a ValUnion {
        fn write(self, p: &mut P) {
            p.write_struct_begin("ValUnion");
            p.write_field_begin("v1", TType::Struct, 1);
            Serialize::write(&self.v1, p);
            p.write_field_end();
            p.write_field_begin("v2", TType::Struct, 2);
            Serialize::write(&self.v2, p);
            p.write_field_end();
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P: ProtocolReader> Deserialize<P> for ValUnion {
        fn read(p: &mut P) -> failure::Fallible<Self> {
            let mut field_v1 = None;
            let mut field_v2 = None;
            let _ = p.read_struct_begin(|_| ())?;
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| ())?;
                match (fty, fid as i32) {
                    (TType::Stop, _) => break,
                    (TType::Struct, 1) => field_v1 = Some(Deserialize::read(p)?),
                    (TType::Struct, 2) => field_v2 = Some(Deserialize::read(p)?),
                    (fty, _) => p.skip(fty)?,
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            Ok(Self {
                v1: field_v1.unwrap_or_default(),
                v2: field_v2.unwrap_or_default(),
            })
        }
    }

    impl Default for VirtualComplexUnion {
        fn default() -> Self {
            Self {
                thingOne: Default::default(),
                thingTwo: Default::default(),
            }
        }
    }

    impl GetTType for VirtualComplexUnion {
        const TTYPE: TType = TType::Struct;
    }

    impl<'a, P: ProtocolWriter> Serialize<P> for &'a VirtualComplexUnion {
        fn write(self, p: &mut P) {
            p.write_struct_begin("VirtualComplexUnion");
            p.write_field_begin("thingOne", TType::String, 1);
            Serialize::write(&self.thingOne, p);
            p.write_field_end();
            p.write_field_begin("thingTwo", TType::String, 2);
            Serialize::write(&self.thingTwo, p);
            p.write_field_end();
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P: ProtocolReader> Deserialize<P> for VirtualComplexUnion {
        fn read(p: &mut P) -> failure::Fallible<Self> {
            let mut field_thingOne = None;
            let mut field_thingTwo = None;
            let _ = p.read_struct_begin(|_| ())?;
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| ())?;
                match (fty, fid as i32) {
                    (TType::Stop, _) => break,
                    (TType::String, 1) => field_thingOne = Some(Deserialize::read(p)?),
                    (TType::String, 2) => field_thingTwo = Some(Deserialize::read(p)?),
                    (fty, _) => p.skip(fty)?,
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            Ok(Self {
                thingOne: field_thingOne.unwrap_or_default(),
                thingTwo: field_thingTwo.unwrap_or_default(),
            })
        }
    }

    impl Default for NonCopyableStruct {
        fn default() -> Self {
            Self {
                num: Default::default(),
            }
        }
    }

    impl GetTType for NonCopyableStruct {
        const TTYPE: TType = TType::Struct;
    }

    impl<'a, P: ProtocolWriter> Serialize<P> for &'a NonCopyableStruct {
        fn write(self, p: &mut P) {
            p.write_struct_begin("NonCopyableStruct");
            p.write_field_begin("num", TType::I64, 1);
            Serialize::write(&self.num, p);
            p.write_field_end();
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P: ProtocolReader> Deserialize<P> for NonCopyableStruct {
        fn read(p: &mut P) -> failure::Fallible<Self> {
            let mut field_num = None;
            let _ = p.read_struct_begin(|_| ())?;
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| ())?;
                match (fty, fid as i32) {
                    (TType::Stop, _) => break,
                    (TType::I64, 1) => field_num = Some(Deserialize::read(p)?),
                    (fty, _) => p.skip(fty)?,
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            Ok(Self {
                num: field_num.unwrap_or_default(),
            })
        }
    }

    impl Default for NonCopyableUnion {
        fn default() -> Self {
            Self {
                s: Default::default(),
            }
        }
    }

    impl GetTType for NonCopyableUnion {
        const TTYPE: TType = TType::Struct;
    }

    impl<'a, P: ProtocolWriter> Serialize<P> for &'a NonCopyableUnion {
        fn write(self, p: &mut P) {
            p.write_struct_begin("NonCopyableUnion");
            p.write_field_begin("s", TType::Struct, 1);
            Serialize::write(&self.s, p);
            p.write_field_end();
            p.write_field_stop();
            p.write_struct_end();
        }
    }

    impl<P: ProtocolReader> Deserialize<P> for NonCopyableUnion {
        fn read(p: &mut P) -> failure::Fallible<Self> {
            let mut field_s = None;
            let _ = p.read_struct_begin(|_| ())?;
            loop {
                let (_, fty, fid) = p.read_field_begin(|_| ())?;
                match (fty, fid as i32) {
                    (TType::Stop, _) => break,
                    (TType::Struct, 1) => field_s = Some(Deserialize::read(p)?),
                    (fty, _) => p.skip(fty)?,
                }
                p.read_field_end()?;
            }
            p.read_struct_end()?;
            Ok(Self {
                s: field_s.unwrap_or_default(),
            })
        }
    }
}

pub mod errors {
    use failure::Fail;
    use fbthrift::ApplicationException;

    #[derive(Debug, Fail)]
    pub enum ErrorKind {
        #[fail(display = "Application exception: {:?}", _0)]
        ApplicationException(ApplicationException),
    }

    impl From<ApplicationException> for ErrorKind {
        fn from(exn: ApplicationException) -> Self {
            ErrorKind::ApplicationException(exn).into()
        }
    }
}