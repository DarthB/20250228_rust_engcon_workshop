#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
use serde::{Deserialize, Serialize};
use std::fs;
pub struct DistillationColumn {
    trays: i32,
    feed_place: i32,
    reflux_ratio: f32,
    #[serde(rename = "d2f")]
    distiliate_to_feed_ratio: f32,
}
#[automatically_derived]
impl ::core::fmt::Debug for DistillationColumn {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "DistillationColumn",
            "trays",
            &self.trays,
            "feed_place",
            &self.feed_place,
            "reflux_ratio",
            &self.reflux_ratio,
            "distiliate_to_feed_ratio",
            &&self.distiliate_to_feed_ratio,
        )
    }
}
#[automatically_derived]
impl ::core::clone::Clone for DistillationColumn {
    #[inline]
    fn clone(&self) -> DistillationColumn {
        let _: ::core::clone::AssertParamIsClone<i32>;
        let _: ::core::clone::AssertParamIsClone<f32>;
        *self
    }
}
#[automatically_derived]
impl ::core::marker::Copy for DistillationColumn {}
#[automatically_derived]
impl ::core::default::Default for DistillationColumn {
    #[inline]
    fn default() -> DistillationColumn {
        DistillationColumn {
            trays: ::core::default::Default::default(),
            feed_place: ::core::default::Default::default(),
            reflux_ratio: ::core::default::Default::default(),
            distiliate_to_feed_ratio: ::core::default::Default::default(),
        }
    }
}
#[automatically_derived]
impl ::core::marker::StructuralPartialEq for DistillationColumn {}
#[automatically_derived]
impl ::core::cmp::PartialEq for DistillationColumn {
    #[inline]
    fn eq(&self, other: &DistillationColumn) -> bool {
        self.trays == other.trays && self.feed_place == other.feed_place
            && self.reflux_ratio == other.reflux_ratio
            && self.distiliate_to_feed_ratio == other.distiliate_to_feed_ratio
    }
}
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl _serde::Serialize for DistillationColumn {
        fn serialize<__S>(
            &self,
            __serializer: __S,
        ) -> _serde::__private::Result<__S::Ok, __S::Error>
        where
            __S: _serde::Serializer,
        {
            let mut __serde_state = _serde::Serializer::serialize_struct(
                __serializer,
                "DistillationColumn",
                false as usize + 1 + 1 + 1 + 1,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "trays",
                &self.trays,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "feed_place",
                &self.feed_place,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "reflux_ratio",
                &self.reflux_ratio,
            )?;
            _serde::ser::SerializeStruct::serialize_field(
                &mut __serde_state,
                "d2f",
                &self.distiliate_to_feed_ratio,
            )?;
            _serde::ser::SerializeStruct::end(__serde_state)
        }
    }
};
#[doc(hidden)]
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _: () = {
    #[allow(unused_extern_crates, clippy::useless_attribute)]
    extern crate serde as _serde;
    #[automatically_derived]
    impl<'de> _serde::Deserialize<'de> for DistillationColumn {
        fn deserialize<__D>(
            __deserializer: __D,
        ) -> _serde::__private::Result<Self, __D::Error>
        where
            __D: _serde::Deserializer<'de>,
        {
            #[allow(non_camel_case_types)]
            #[doc(hidden)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __ignore,
            }
            #[doc(hidden)]
            struct __FieldVisitor;
            impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                type Value = __Field;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "field identifier",
                    )
                }
                fn visit_u64<__E>(
                    self,
                    __value: u64,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        0u64 => _serde::__private::Ok(__Field::__field0),
                        1u64 => _serde::__private::Ok(__Field::__field1),
                        2u64 => _serde::__private::Ok(__Field::__field2),
                        3u64 => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_str<__E>(
                    self,
                    __value: &str,
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        "trays" => _serde::__private::Ok(__Field::__field0),
                        "feed_place" => _serde::__private::Ok(__Field::__field1),
                        "reflux_ratio" => _serde::__private::Ok(__Field::__field2),
                        "d2f" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
                fn visit_bytes<__E>(
                    self,
                    __value: &[u8],
                ) -> _serde::__private::Result<Self::Value, __E>
                where
                    __E: _serde::de::Error,
                {
                    match __value {
                        b"trays" => _serde::__private::Ok(__Field::__field0),
                        b"feed_place" => _serde::__private::Ok(__Field::__field1),
                        b"reflux_ratio" => _serde::__private::Ok(__Field::__field2),
                        b"d2f" => _serde::__private::Ok(__Field::__field3),
                        _ => _serde::__private::Ok(__Field::__ignore),
                    }
                }
            }
            impl<'de> _serde::Deserialize<'de> for __Field {
                #[inline]
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    _serde::Deserializer::deserialize_identifier(
                        __deserializer,
                        __FieldVisitor,
                    )
                }
            }
            #[doc(hidden)]
            struct __Visitor<'de> {
                marker: _serde::__private::PhantomData<DistillationColumn>,
                lifetime: _serde::__private::PhantomData<&'de ()>,
            }
            impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                type Value = DistillationColumn;
                fn expecting(
                    &self,
                    __formatter: &mut _serde::__private::Formatter,
                ) -> _serde::__private::fmt::Result {
                    _serde::__private::Formatter::write_str(
                        __formatter,
                        "struct DistillationColumn",
                    )
                }
                #[inline]
                fn visit_seq<__A>(
                    self,
                    mut __seq: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::SeqAccess<'de>,
                {
                    let __field0 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    0usize,
                                    &"struct DistillationColumn with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field1 = match _serde::de::SeqAccess::next_element::<
                        i32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    1usize,
                                    &"struct DistillationColumn with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field2 = match _serde::de::SeqAccess::next_element::<
                        f32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    2usize,
                                    &"struct DistillationColumn with 4 elements",
                                ),
                            );
                        }
                    };
                    let __field3 = match _serde::de::SeqAccess::next_element::<
                        f32,
                    >(&mut __seq)? {
                        _serde::__private::Some(__value) => __value,
                        _serde::__private::None => {
                            return _serde::__private::Err(
                                _serde::de::Error::invalid_length(
                                    3usize,
                                    &"struct DistillationColumn with 4 elements",
                                ),
                            );
                        }
                    };
                    _serde::__private::Ok(DistillationColumn {
                        trays: __field0,
                        feed_place: __field1,
                        reflux_ratio: __field2,
                        distiliate_to_feed_ratio: __field3,
                    })
                }
                #[inline]
                fn visit_map<__A>(
                    self,
                    mut __map: __A,
                ) -> _serde::__private::Result<Self::Value, __A::Error>
                where
                    __A: _serde::de::MapAccess<'de>,
                {
                    let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                    let mut __field2: _serde::__private::Option<f32> = _serde::__private::None;
                    let mut __field3: _serde::__private::Option<f32> = _serde::__private::None;
                    while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                        __Field,
                    >(&mut __map)? {
                        match __key {
                            __Field::__field0 => {
                                if _serde::__private::Option::is_some(&__field0) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("trays"),
                                    );
                                }
                                __field0 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field1 => {
                                if _serde::__private::Option::is_some(&__field1) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "feed_place",
                                        ),
                                    );
                                }
                                __field1 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                );
                            }
                            __Field::__field2 => {
                                if _serde::__private::Option::is_some(&__field2) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field(
                                            "reflux_ratio",
                                        ),
                                    );
                                }
                                __field2 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                );
                            }
                            __Field::__field3 => {
                                if _serde::__private::Option::is_some(&__field3) {
                                    return _serde::__private::Err(
                                        <__A::Error as _serde::de::Error>::duplicate_field("d2f"),
                                    );
                                }
                                __field3 = _serde::__private::Some(
                                    _serde::de::MapAccess::next_value::<f32>(&mut __map)?,
                                );
                            }
                            _ => {
                                let _ = _serde::de::MapAccess::next_value::<
                                    _serde::de::IgnoredAny,
                                >(&mut __map)?;
                            }
                        }
                    }
                    let __field0 = match __field0 {
                        _serde::__private::Some(__field0) => __field0,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("trays")?
                        }
                    };
                    let __field1 = match __field1 {
                        _serde::__private::Some(__field1) => __field1,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("feed_place")?
                        }
                    };
                    let __field2 = match __field2 {
                        _serde::__private::Some(__field2) => __field2,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("reflux_ratio")?
                        }
                    };
                    let __field3 = match __field3 {
                        _serde::__private::Some(__field3) => __field3,
                        _serde::__private::None => {
                            _serde::__private::de::missing_field("d2f")?
                        }
                    };
                    _serde::__private::Ok(DistillationColumn {
                        trays: __field0,
                        feed_place: __field1,
                        reflux_ratio: __field2,
                        distiliate_to_feed_ratio: __field3,
                    })
                }
            }
            #[doc(hidden)]
            const FIELDS: &'static [&'static str] = &[
                "trays",
                "feed_place",
                "reflux_ratio",
                "d2f",
            ];
            _serde::Deserializer::deserialize_struct(
                __deserializer,
                "DistillationColumn",
                FIELDS,
                __Visitor {
                    marker: _serde::__private::PhantomData::<DistillationColumn>,
                    lifetime: _serde::__private::PhantomData,
                },
            )
        }
    }
};
fn export_json() -> Result<(), Box<dyn std::error::Error>> {
    let output_data = DistillationColumn::default();
    let json = serde_json::to_string_pretty(&output_data)?;
    let filename = "output.json";
    {
        ::std::io::_print(format_args!("JSON written to {0}:\n{1}\n", filename, json));
    };
    fs::write(filename, json)?;
    Ok(())
}
fn main() {
    export_json().unwrap();
}
