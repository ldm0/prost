#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
#[rustfmt::skip]
pub mod wrappers_arc {
    pub struct Payload {
        #[prost(int32, repeated, packed = "false", tag = "1")]
        pub stuff: ::prost::alloc::vec::Vec<i32>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Payload {
        #[inline]
        fn clone(&self) -> Payload {
            Payload {
                stuff: ::core::clone::Clone::clone(&self.stuff),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Payload {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Payload {
        #[inline]
        fn eq(&self, other: &Payload) -> bool {
            self.stuff == other.stuff
        }
    }
    impl ::prost::Message for Payload {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            ::prost::encoding::int32::encode_repeated(1u32, &self.stuff, buf);
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Payload";
            match tag {
                1u32 => {
                    let mut value = &mut self.stuff;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "stuff");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::int32::encoded_len_repeated(1u32, &self.stuff)
        }
        fn clear(&mut self) {
            self.stuff.clear();
        }
    }
    impl ::core::default::Default for Payload {
        fn default() -> Self {
            Payload {
                stuff: ::prost::alloc::vec::Vec::new(),
            }
        }
    }
    impl ::core::fmt::Debug for Payload {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Payload");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.stuff)
                };
                builder.field("stuff", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct MyMessage {
        #[prost(int32, required, arc, tag = "20")]
        pub int: ::prost::alloc::sync::Arc<i32>,
        #[prost(int32, optional, arc, tag = "21")]
        pub optional_int: ::prost::alloc::sync::Arc<::core::option::Option<i32>>,
        #[prost(int32, repeated, packed = "false", arc, tag = "22")]
        pub repeated_int: ::prost::alloc::sync::Arc<::prost::alloc::vec::Vec<i32>>,
        #[prost(int32, repeated, arc, tag = "23")]
        pub packed_int: ::prost::alloc::sync::Arc<::prost::alloc::vec::Vec<i32>>,
        #[prost(string, required, arc, tag = "1")]
        pub str: ::prost::alloc::sync::Arc<::prost::alloc::string::String>,
        #[prost(string, optional, arc, tag = "2")]
        pub optional_str: ::prost::alloc::sync::Arc<
            ::core::option::Option<::prost::alloc::string::String>,
        >,
        #[prost(string, repeated, arc, tag = "16")]
        pub repeated_str: ::prost::alloc::sync::Arc<
            ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        >,
        #[prost(message, required, arc, tag = "3")]
        pub payload: ::prost::alloc::sync::Arc<Payload>,
        #[prost(message, optional, arc, tag = "4")]
        pub optional_payload: ::prost::alloc::sync::Arc<::core::option::Option<Payload>>,
        #[prost(message, repeated, arc, tag = "17")]
        pub repeated_payload: ::prost::alloc::sync::Arc<
            ::prost::alloc::vec::Vec<Payload>,
        >,
        #[prost(btree_map = "int32, message", arc, tag = "5")]
        pub map_payload: ::prost::alloc::sync::Arc<
            ::prost::alloc::collections::BTreeMap<i32, Payload>,
        >,
        #[prost(group, required, arc, tag = "6")]
        pub group: ::prost::alloc::sync::Arc<my_message::Group>,
        #[prost(group, optional, arc, tag = "8")]
        pub optional_group: ::prost::alloc::sync::Arc<
            ::core::option::Option<my_message::OptionalGroup>,
        >,
        #[prost(group, repeated, arc, tag = "18")]
        pub repeated_group: ::prost::alloc::sync::Arc<
            ::prost::alloc::vec::Vec<my_message::RepeatedGroup>,
        >,
        #[prost(enumeration = "MyEnum", required, arc, tag = "12")]
        pub my_enum: ::prost::alloc::sync::Arc<i32>,
        #[prost(enumeration = "MyEnum", optional, arc, tag = "13")]
        pub optional_my_enum: ::prost::alloc::sync::Arc<::core::option::Option<i32>>,
        #[prost(enumeration = "MyEnum", repeated, packed = "false", arc, tag = "14")]
        pub repeated_my_enum: ::prost::alloc::sync::Arc<::prost::alloc::vec::Vec<i32>>,
        #[prost(enumeration = "MyEnum", repeated, arc, tag = "15")]
        pub packed_my_enum: ::prost::alloc::sync::Arc<::prost::alloc::vec::Vec<i32>>,
        /// default tests:
        #[prost(int32, optional, arc, tag = "24", default = "42")]
        pub default_int: ::prost::alloc::sync::Arc<::core::option::Option<i32>>,
        #[prost(float, optional, arc, tag = "25", default = "1")]
        pub default_float: ::prost::alloc::sync::Arc<::core::option::Option<f32>>,
        #[prost(string, optional, arc, tag = "26", default = "foobar")]
        pub default_string: ::prost::alloc::sync::Arc<
            ::core::option::Option<::prost::alloc::string::String>,
        >,
        #[prost(oneof = "my_message::OneofField", arc, tags = "10, 11")]
        pub oneof_field: ::prost::alloc::sync::Arc<
            ::core::option::Option<my_message::OneofField>,
        >,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MyMessage {
        #[inline]
        fn clone(&self) -> MyMessage {
            MyMessage {
                int: ::core::clone::Clone::clone(&self.int),
                optional_int: ::core::clone::Clone::clone(&self.optional_int),
                repeated_int: ::core::clone::Clone::clone(&self.repeated_int),
                packed_int: ::core::clone::Clone::clone(&self.packed_int),
                str: ::core::clone::Clone::clone(&self.str),
                optional_str: ::core::clone::Clone::clone(&self.optional_str),
                repeated_str: ::core::clone::Clone::clone(&self.repeated_str),
                payload: ::core::clone::Clone::clone(&self.payload),
                optional_payload: ::core::clone::Clone::clone(&self.optional_payload),
                repeated_payload: ::core::clone::Clone::clone(&self.repeated_payload),
                map_payload: ::core::clone::Clone::clone(&self.map_payload),
                group: ::core::clone::Clone::clone(&self.group),
                optional_group: ::core::clone::Clone::clone(&self.optional_group),
                repeated_group: ::core::clone::Clone::clone(&self.repeated_group),
                my_enum: ::core::clone::Clone::clone(&self.my_enum),
                optional_my_enum: ::core::clone::Clone::clone(&self.optional_my_enum),
                repeated_my_enum: ::core::clone::Clone::clone(&self.repeated_my_enum),
                packed_my_enum: ::core::clone::Clone::clone(&self.packed_my_enum),
                default_int: ::core::clone::Clone::clone(&self.default_int),
                default_float: ::core::clone::Clone::clone(&self.default_float),
                default_string: ::core::clone::Clone::clone(&self.default_string),
                oneof_field: ::core::clone::Clone::clone(&self.oneof_field),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MyMessage {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MyMessage {
        #[inline]
        fn eq(&self, other: &MyMessage) -> bool {
            self.int == other.int && self.optional_int == other.optional_int
                && self.repeated_int == other.repeated_int
                && self.packed_int == other.packed_int && self.str == other.str
                && self.optional_str == other.optional_str
                && self.repeated_str == other.repeated_str
                && self.payload == other.payload
                && self.optional_payload == other.optional_payload
                && self.repeated_payload == other.repeated_payload
                && self.map_payload == other.map_payload && self.group == other.group
                && self.optional_group == other.optional_group
                && self.repeated_group == other.repeated_group
                && self.my_enum == other.my_enum
                && self.optional_my_enum == other.optional_my_enum
                && self.repeated_my_enum == other.repeated_my_enum
                && self.packed_my_enum == other.packed_my_enum
                && self.default_int == other.default_int
                && self.default_float == other.default_float
                && self.default_string == other.default_string
                && self.oneof_field == other.oneof_field
        }
    }
    impl ::prost::Message for MyMessage {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            ::prost::encoding::string::encode(1u32, &*self.str, buf);
            if let ::core::option::Option::Some(value) = &*self.optional_str {
                ::prost::encoding::string::encode(2u32, value, buf);
            }
            ::prost::encoding::message::encode(3u32, &*self.payload, buf);
            if let Some(msg) = &*self.optional_payload {
                ::prost::encoding::message::encode(4u32, msg, buf);
            }
            ::prost::encoding::btree_map::encode(
                ::prost::encoding::int32::encode,
                ::prost::encoding::int32::encoded_len,
                ::prost::encoding::message::encode,
                ::prost::encoding::message::encoded_len,
                5u32,
                &self.map_payload,
                buf,
            );
            ::prost::encoding::group::encode(6u32, &*self.group, buf);
            if let Some(msg) = &*self.optional_group {
                ::prost::encoding::group::encode(8u32, msg, buf);
            }
            if let Some(oneof) = &*self.oneof_field {
                oneof.encode(buf)
            }
            ::prost::encoding::int32::encode(12u32, &*self.my_enum, buf);
            if let ::core::option::Option::Some(value) = &*self.optional_my_enum {
                ::prost::encoding::int32::encode(13u32, value, buf);
            }
            ::prost::encoding::int32::encode_repeated(
                14u32,
                &*self.repeated_my_enum,
                buf,
            );
            ::prost::encoding::int32::encode_packed(15u32, &*self.packed_my_enum, buf);
            ::prost::encoding::string::encode_repeated(16u32, &*self.repeated_str, buf);
            for msg in &*self.repeated_payload {
                ::prost::encoding::message::encode(17u32, msg, buf);
            }
            for msg in &*self.repeated_group {
                ::prost::encoding::group::encode(18u32, msg, buf);
            }
            ::prost::encoding::int32::encode(20u32, &*self.int, buf);
            if let ::core::option::Option::Some(value) = &*self.optional_int {
                ::prost::encoding::int32::encode(21u32, value, buf);
            }
            ::prost::encoding::int32::encode_repeated(22u32, &*self.repeated_int, buf);
            ::prost::encoding::int32::encode_packed(23u32, &*self.packed_int, buf);
            if let ::core::option::Option::Some(value) = &*self.default_int {
                ::prost::encoding::int32::encode(24u32, value, buf);
            }
            if let ::core::option::Option::Some(value) = &*self.default_float {
                ::prost::encoding::float::encode(25u32, value, buf);
            }
            if let ::core::option::Option::Some(value) = &*self.default_string {
                ::prost::encoding::string::encode(26u32, value, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "MyMessage";
            match tag {
                1u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(&mut self.str)
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "str");
                            error
                        })
                }
                2u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.optional_str,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::string::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "optional_str");
                            error
                        })
                }
                3u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(&mut self.payload)
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::message::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "payload");
                            error
                        })
                }
                4u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.optional_payload,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "optional_payload");
                            error
                        })
                }
                5u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.map_payload,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::btree_map::merge(
                            ::prost::encoding::int32::merge,
                            ::prost::encoding::message::merge,
                            &mut value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "map_payload");
                            error
                        })
                }
                6u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(&mut self.group)
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::group::merge(tag, wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "group");
                            error
                        })
                }
                8u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.optional_group,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::group::merge(
                            tag,
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "optional_group");
                            error
                        })
                }
                10u32 | 11u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.oneof_field,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    my_message::OneofField::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "oneof_field");
                            error
                        })
                }
                12u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(&mut self.my_enum)
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "my_enum");
                            error
                        })
                }
                13u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.optional_my_enum,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::int32::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "optional_my_enum");
                            error
                        })
                }
                14u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.repeated_my_enum,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "repeated_my_enum");
                            error
                        })
                }
                15u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.packed_my_enum,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "packed_my_enum");
                            error
                        })
                }
                16u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.repeated_str,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::string::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "repeated_str");
                            error
                        })
                }
                17u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.repeated_payload,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "repeated_payload");
                            error
                        })
                }
                18u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.repeated_group,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::group::merge_repeated(
                            tag,
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "repeated_group");
                            error
                        })
                }
                20u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(&mut self.int)
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "int");
                            error
                        })
                }
                21u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.optional_int,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::int32::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "optional_int");
                            error
                        })
                }
                22u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.repeated_int,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "repeated_int");
                            error
                        })
                }
                23u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.packed_int,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "packed_int");
                            error
                        })
                }
                24u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.default_int,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::int32::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "default_int");
                            error
                        })
                }
                25u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.default_float,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::float::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "default_float");
                            error
                        })
                }
                26u32 => {
                    let mut value = ::prost::alloc::sync::Arc::get_mut(
                            &mut self.default_string,
                        )
                        .ok_or_else(|| {
                            ::prost::DecodeError::new(
                                "Cannot get mutable reference to a Arc field",
                            )
                        })?;
                    ::prost::encoding::string::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "default_string");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::string::encoded_len(1u32, &*self.str)
                + (&*self.optional_str)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::string::encoded_len(2u32, value),
                    ) + ::prost::encoding::message::encoded_len(3u32, &*self.payload)
                + (&*self.optional_payload)
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(4u32, msg))
                + ::prost::encoding::btree_map::encoded_len(
                    ::prost::encoding::int32::encoded_len,
                    ::prost::encoding::message::encoded_len,
                    5u32,
                    &self.map_payload,
                ) + ::prost::encoding::group::encoded_len(6u32, &*self.group)
                + (&*self.optional_group)
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::group::encoded_len(8u32, msg))
                + (&*self.oneof_field)
                    .as_ref()
                    .map_or(0, my_message::OneofField::encoded_len)
                + ::prost::encoding::int32::encoded_len(12u32, &*self.my_enum)
                + (&*self.optional_my_enum)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::int32::encoded_len(13u32, value),
                    )
                + ::prost::encoding::int32::encoded_len_repeated(
                    14u32,
                    &*self.repeated_my_enum,
                )
                + ::prost::encoding::int32::encoded_len_packed(
                    15u32,
                    &*self.packed_my_enum,
                )
                + ::prost::encoding::string::encoded_len_repeated(
                    16u32,
                    &*self.repeated_str,
                )
                + ::prost::encoding::message::encoded_len_repeated(
                    17u32,
                    &*self.repeated_payload,
                )
                + ::prost::encoding::group::encoded_len_repeated(
                    18u32,
                    &*self.repeated_group,
                ) + ::prost::encoding::int32::encoded_len(20u32, &*self.int)
                + (&*self.optional_int)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::int32::encoded_len(21u32, value),
                    )
                + ::prost::encoding::int32::encoded_len_repeated(
                    22u32,
                    &*self.repeated_int,
                )
                + ::prost::encoding::int32::encoded_len_packed(23u32, &*self.packed_int)
                + (&*self.default_int)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::int32::encoded_len(24u32, value),
                    )
                + (&*self.default_float)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::float::encoded_len(25u32, value),
                    )
                + (&*self.default_string)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::string::encoded_len(26u32, value),
                    )
        }
        fn clear(&mut self) {
            self.str = ::prost::alloc::sync::Arc::new(
                ::prost::alloc::string::String::new(),
            );
            self.optional_str = ::prost::alloc::sync::Arc::new(
                ::core::option::Option::None,
            );
            self.payload = ::prost::alloc::sync::Arc::new(
                ::core::default::Default::default(),
            );
            self.optional_payload = ::prost::alloc::sync::Arc::new(
                ::core::default::Default::default(),
            );
            self.map_payload = ::prost::alloc::sync::Arc::new(
                ::core::default::Default::default(),
            );
            self.group = ::prost::alloc::sync::Arc::new(
                ::core::default::Default::default(),
            );
            self.optional_group = ::prost::alloc::sync::Arc::new(
                ::core::default::Default::default(),
            );
            self.oneof_field = ::prost::alloc::sync::Arc::new(
                ::core::default::Default::default(),
            );
            self.my_enum = ::prost::alloc::sync::Arc::new(MyEnum::default() as i32);
            self.optional_my_enum = ::prost::alloc::sync::Arc::new(
                ::core::option::Option::None,
            );
            self.repeated_my_enum = ::prost::alloc::sync::Arc::new(
                ::prost::alloc::vec::Vec::new(),
            );
            self.packed_my_enum = ::prost::alloc::sync::Arc::new(
                ::prost::alloc::vec::Vec::new(),
            );
            self.repeated_str = ::prost::alloc::sync::Arc::new(
                ::prost::alloc::vec::Vec::new(),
            );
            self.repeated_payload = ::prost::alloc::sync::Arc::new(
                ::core::default::Default::default(),
            );
            self.repeated_group = ::prost::alloc::sync::Arc::new(
                ::core::default::Default::default(),
            );
            self.int = ::prost::alloc::sync::Arc::new(0i32);
            self.optional_int = ::prost::alloc::sync::Arc::new(
                ::core::option::Option::None,
            );
            self.repeated_int = ::prost::alloc::sync::Arc::new(
                ::prost::alloc::vec::Vec::new(),
            );
            self.packed_int = ::prost::alloc::sync::Arc::new(
                ::prost::alloc::vec::Vec::new(),
            );
            self.default_int = ::prost::alloc::sync::Arc::new(
                ::core::option::Option::None,
            );
            self.default_float = ::prost::alloc::sync::Arc::new(
                ::core::option::Option::None,
            );
            self.default_string = ::prost::alloc::sync::Arc::new(
                ::core::option::Option::None,
            );
        }
    }
    impl ::core::default::Default for MyMessage {
        fn default() -> Self {
            MyMessage {
                str: ::prost::alloc::sync::Arc::new(
                    ::prost::alloc::string::String::new(),
                ),
                optional_str: ::prost::alloc::sync::Arc::new(
                    ::core::option::Option::None,
                ),
                payload: ::prost::alloc::sync::Arc::new(
                    ::core::default::Default::default(),
                ),
                optional_payload: ::prost::alloc::sync::Arc::new(
                    ::core::default::Default::default(),
                ),
                map_payload: ::prost::alloc::sync::Arc::new(
                    ::core::default::Default::default(),
                ),
                group: ::prost::alloc::sync::Arc::new(
                    ::core::default::Default::default(),
                ),
                optional_group: ::prost::alloc::sync::Arc::new(
                    ::core::default::Default::default(),
                ),
                oneof_field: ::prost::alloc::sync::Arc::new(
                    ::core::default::Default::default(),
                ),
                my_enum: ::prost::alloc::sync::Arc::new(MyEnum::default() as i32),
                optional_my_enum: ::prost::alloc::sync::Arc::new(
                    ::core::option::Option::None,
                ),
                repeated_my_enum: ::prost::alloc::sync::Arc::new(
                    ::prost::alloc::vec::Vec::new(),
                ),
                packed_my_enum: ::prost::alloc::sync::Arc::new(
                    ::prost::alloc::vec::Vec::new(),
                ),
                repeated_str: ::prost::alloc::sync::Arc::new(
                    ::prost::alloc::vec::Vec::new(),
                ),
                repeated_payload: ::prost::alloc::sync::Arc::new(
                    ::core::default::Default::default(),
                ),
                repeated_group: ::prost::alloc::sync::Arc::new(
                    ::core::default::Default::default(),
                ),
                int: ::prost::alloc::sync::Arc::new(0i32),
                optional_int: ::prost::alloc::sync::Arc::new(
                    ::core::option::Option::None,
                ),
                repeated_int: ::prost::alloc::sync::Arc::new(
                    ::prost::alloc::vec::Vec::new(),
                ),
                packed_int: ::prost::alloc::sync::Arc::new(
                    ::prost::alloc::vec::Vec::new(),
                ),
                default_int: ::prost::alloc::sync::Arc::new(
                    ::core::option::Option::None,
                ),
                default_float: ::prost::alloc::sync::Arc::new(
                    ::core::option::Option::None,
                ),
                default_string: ::prost::alloc::sync::Arc::new(
                    ::core::option::Option::None,
                ),
            }
        }
    }
    impl ::core::fmt::Debug for MyMessage {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("MyMessage");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.int)
                };
                builder.field("int", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.optional_int)
                };
                builder.field("optional_int", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.repeated_int)
                };
                builder.field("repeated_int", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.packed_int)
                };
                builder.field("packed_int", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.str)
                };
                builder.field("str", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::core::option::Option<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.optional_str)
                };
                builder.field("optional_str", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.repeated_str)
                };
                builder.field("repeated_str", &wrapper)
            };
            let builder = {
                let wrapper = &self.payload;
                builder.field("payload", &wrapper)
            };
            let builder = {
                let wrapper = &self.optional_payload;
                builder.field("optional_payload", &wrapper)
            };
            let builder = {
                let wrapper = &self.repeated_payload;
                builder.field("repeated_payload", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct MapWrapper<'a, V: 'a>(
                        &'a ::prost::alloc::collections::BTreeMap<i32, V>,
                    );
                    impl<'a, V> ::core::fmt::Debug for MapWrapper<'a, V>
                    where
                        V: ::core::fmt::Debug + 'a,
                    {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn KeyWrapper<T>(v: T) -> T {
                                v
                            }
                            fn ValueWrapper<T>(v: T) -> T {
                                v
                            }
                            let mut builder = f.debug_map();
                            for (k, v) in self.0 {
                                builder.entry(&KeyWrapper(k), &ValueWrapper(v));
                            }
                            builder.finish()
                        }
                    }
                    MapWrapper(&self.map_payload)
                };
                builder.field("map_payload", &wrapper)
            };
            let builder = {
                let wrapper = &self.group;
                builder.field("group", &wrapper)
            };
            let builder = {
                let wrapper = &self.optional_group;
                builder.field("optional_group", &wrapper)
            };
            let builder = {
                let wrapper = &self.repeated_group;
                builder.field("repeated_group", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.my_enum)
                };
                builder.field("my_enum", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            struct Inner<'a>(&'a i32);
                            impl<'a> ::core::fmt::Debug for Inner<'a> {
                                fn fmt(
                                    &self,
                                    f: &mut ::core::fmt::Formatter,
                                ) -> ::core::fmt::Result {
                                    let res: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                                        *self.0,
                                    );
                                    match res {
                                        Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                        Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                                    }
                                }
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.optional_my_enum)
                };
                builder.field("optional_my_enum", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                struct Inner<'a>(&'a i32);
                                impl<'a> ::core::fmt::Debug for Inner<'a> {
                                    fn fmt(
                                        &self,
                                        f: &mut ::core::fmt::Formatter,
                                    ) -> ::core::fmt::Result {
                                        let res: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                                            *self.0,
                                        );
                                        match res {
                                            Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                            Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                                        }
                                    }
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.repeated_my_enum)
                };
                builder.field("repeated_my_enum", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                struct Inner<'a>(&'a i32);
                                impl<'a> ::core::fmt::Debug for Inner<'a> {
                                    fn fmt(
                                        &self,
                                        f: &mut ::core::fmt::Formatter,
                                    ) -> ::core::fmt::Result {
                                        let res: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                                            *self.0,
                                        );
                                        match res {
                                            Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                            Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                                        }
                                    }
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.packed_my_enum)
                };
                builder.field("packed_my_enum", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.default_int)
                };
                builder.field("default_int", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<f32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.default_float)
                };
                builder.field("default_float", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::core::option::Option<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.default_string)
                };
                builder.field("default_string", &wrapper)
            };
            let builder = {
                let wrapper = &self.oneof_field;
                builder.field("oneof_field", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl MyMessage {
        ///Returns the value of `optional_str`, or the default value if `optional_str` is unset.
        pub fn optional_str(&self) -> &str {
            match (&*self.optional_str).as_ref() {
                ::core::option::Option::Some(val) => &val[..],
                ::core::option::Option::None => "",
            }
        }
        ///Returns the enum value of `my_enum`, or the default if the field is set to an invalid enum value.
        pub fn my_enum(&self) -> MyEnum {
            ::core::convert::TryFrom::try_from(*&*self.my_enum)
                .unwrap_or(MyEnum::default())
        }
        ///Sets `my_enum` to the provided enum value.
        pub fn set_my_enum(&mut self, value: MyEnum) {
            self.my_enum = ::prost::alloc::sync::Arc::new(value as i32);
        }
        ///Returns the enum value of `optional_my_enum`, or the default if the field is unset or set to an invalid enum value.
        pub fn optional_my_enum(&self) -> MyEnum {
            self.optional_my_enum
                .and_then(|x| {
                    let result: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                        x,
                    );
                    result.ok()
                })
                .unwrap_or(MyEnum::default())
        }
        ///Sets `optional_my_enum` to the provided enum value.
        pub fn set_optional_my_enum(&mut self, value: MyEnum) {
            self.optional_my_enum = ::prost::alloc::sync::Arc::new(
                ::core::option::Option::Some(value as i32),
            );
        }
        ///Returns an iterator which yields the valid enum values contained in `repeated_my_enum`.
        pub fn repeated_my_enum(
            &self,
        ) -> ::core::iter::FilterMap<
            ::core::iter::Cloned<::core::slice::Iter<i32>>,
            fn(i32) -> ::core::option::Option<MyEnum>,
        > {
            self.repeated_my_enum
                .iter()
                .cloned()
                .filter_map(|x| {
                    let result: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                        x,
                    );
                    result.ok()
                })
        }
        ///Appends the provided enum value to `repeated_my_enum`.
        pub fn push_repeated_my_enum(&mut self, value: MyEnum) {
            let mut pushed = (*&*self.repeated_my_enum).clone();
            pushed.push(value as i32);
            self.repeated_my_enum = ::prost::alloc::sync::Arc::new(pushed);
        }
        ///Returns an iterator which yields the valid enum values contained in `packed_my_enum`.
        pub fn packed_my_enum(
            &self,
        ) -> ::core::iter::FilterMap<
            ::core::iter::Cloned<::core::slice::Iter<i32>>,
            fn(i32) -> ::core::option::Option<MyEnum>,
        > {
            self.packed_my_enum
                .iter()
                .cloned()
                .filter_map(|x| {
                    let result: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                        x,
                    );
                    result.ok()
                })
        }
        ///Appends the provided enum value to `packed_my_enum`.
        pub fn push_packed_my_enum(&mut self, value: MyEnum) {
            let mut pushed = (*&*self.packed_my_enum).clone();
            pushed.push(value as i32);
            self.packed_my_enum = ::prost::alloc::sync::Arc::new(pushed);
        }
        ///Returns the value of `optional_int`, or the default value if `optional_int` is unset.
        pub fn optional_int(&self) -> i32 {
            match (&*self.optional_int).as_ref() {
                ::core::option::Option::Some(val) => *val,
                ::core::option::Option::None => 0i32,
            }
        }
        ///Returns the value of `default_int`, or the default value if `default_int` is unset.
        pub fn default_int(&self) -> i32 {
            match (&*self.default_int).as_ref() {
                ::core::option::Option::Some(val) => *val,
                ::core::option::Option::None => 42i32,
            }
        }
        ///Returns the value of `default_float`, or the default value if `default_float` is unset.
        pub fn default_float(&self) -> f32 {
            match (&*self.default_float).as_ref() {
                ::core::option::Option::Some(val) => *val,
                ::core::option::Option::None => 1f32,
            }
        }
        ///Returns the value of `default_string`, or the default value if `default_string` is unset.
        pub fn default_string(&self) -> &str {
            match (&*self.default_string).as_ref() {
                ::core::option::Option::Some(val) => &val[..],
                ::core::option::Option::None => "foobar",
            }
        }
    }
    /// Nested message and enum types in `MyMessage`.
    pub mod my_message {
        pub struct Group {
            #[prost(int32, optional, tag = "7")]
            pub i2: ::core::option::Option<i32>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Group {
            #[inline]
            fn clone(&self) -> Group {
                let _: ::core::clone::AssertParamIsClone<::core::option::Option<i32>>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Group {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Group {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Group {
            #[inline]
            fn eq(&self, other: &Group) -> bool {
                self.i2 == other.i2
            }
        }
        impl ::prost::Message for Group {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let ::core::option::Option::Some(value) = &self.i2 {
                    ::prost::encoding::int32::encode(7u32, value, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "Group";
                match tag {
                    7u32 => {
                        let mut value = &mut self.i2;
                        ::prost::encoding::int32::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "i2");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + (&self.i2)
                        .as_ref()
                        .map_or(
                            0,
                            |value| ::prost::encoding::int32::encoded_len(7u32, value),
                        )
            }
            fn clear(&mut self) {
                self.i2.take();
            }
        }
        impl ::core::default::Default for Group {
            fn default() -> Self {
                Group {
                    i2: ::core::option::Option::None,
                }
            }
        }
        impl ::core::fmt::Debug for Group {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Group");
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                            }
                        }
                        ScalarWrapper(&self.i2)
                    };
                    builder.field("i2", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(dead_code)]
        impl Group {
            ///Returns the value of `i2`, or the default value if `i2` is unset.
            pub fn i2(&self) -> i32 {
                match (&self.i2).as_ref() {
                    ::core::option::Option::Some(val) => *val,
                    ::core::option::Option::None => 0i32,
                }
            }
        }
        pub struct OptionalGroup {
            #[prost(int32, optional, tag = "9")]
            pub i2: ::core::option::Option<i32>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for OptionalGroup {
            #[inline]
            fn clone(&self) -> OptionalGroup {
                let _: ::core::clone::AssertParamIsClone<::core::option::Option<i32>>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for OptionalGroup {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OptionalGroup {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for OptionalGroup {
            #[inline]
            fn eq(&self, other: &OptionalGroup) -> bool {
                self.i2 == other.i2
            }
        }
        impl ::prost::Message for OptionalGroup {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let ::core::option::Option::Some(value) = &self.i2 {
                    ::prost::encoding::int32::encode(9u32, value, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "OptionalGroup";
                match tag {
                    9u32 => {
                        let mut value = &mut self.i2;
                        ::prost::encoding::int32::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "i2");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + (&self.i2)
                        .as_ref()
                        .map_or(
                            0,
                            |value| ::prost::encoding::int32::encoded_len(9u32, value),
                        )
            }
            fn clear(&mut self) {
                self.i2.take();
            }
        }
        impl ::core::default::Default for OptionalGroup {
            fn default() -> Self {
                OptionalGroup {
                    i2: ::core::option::Option::None,
                }
            }
        }
        impl ::core::fmt::Debug for OptionalGroup {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("OptionalGroup");
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                            }
                        }
                        ScalarWrapper(&self.i2)
                    };
                    builder.field("i2", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(dead_code)]
        impl OptionalGroup {
            ///Returns the value of `i2`, or the default value if `i2` is unset.
            pub fn i2(&self) -> i32 {
                match (&self.i2).as_ref() {
                    ::core::option::Option::Some(val) => *val,
                    ::core::option::Option::None => 0i32,
                }
            }
        }
        pub struct RepeatedGroup {
            #[prost(int32, optional, tag = "19")]
            pub i2: ::core::option::Option<i32>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RepeatedGroup {
            #[inline]
            fn clone(&self) -> RepeatedGroup {
                let _: ::core::clone::AssertParamIsClone<::core::option::Option<i32>>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for RepeatedGroup {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for RepeatedGroup {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for RepeatedGroup {
            #[inline]
            fn eq(&self, other: &RepeatedGroup) -> bool {
                self.i2 == other.i2
            }
        }
        impl ::prost::Message for RepeatedGroup {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let ::core::option::Option::Some(value) = &self.i2 {
                    ::prost::encoding::int32::encode(19u32, value, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "RepeatedGroup";
                match tag {
                    19u32 => {
                        let mut value = &mut self.i2;
                        ::prost::encoding::int32::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "i2");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + (&self.i2)
                        .as_ref()
                        .map_or(
                            0,
                            |value| ::prost::encoding::int32::encoded_len(19u32, value),
                        )
            }
            fn clear(&mut self) {
                self.i2.take();
            }
        }
        impl ::core::default::Default for RepeatedGroup {
            fn default() -> Self {
                RepeatedGroup {
                    i2: ::core::option::Option::None,
                }
            }
        }
        impl ::core::fmt::Debug for RepeatedGroup {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("RepeatedGroup");
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                            }
                        }
                        ScalarWrapper(&self.i2)
                    };
                    builder.field("i2", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(dead_code)]
        impl RepeatedGroup {
            ///Returns the value of `i2`, or the default value if `i2` is unset.
            pub fn i2(&self) -> i32 {
                match (&self.i2).as_ref() {
                    ::core::option::Option::Some(val) => *val,
                    ::core::option::Option::None => 0i32,
                }
            }
        }
        pub enum OneofField {
            #[prost(string, arc, tag = "10")]
            A(::prost::alloc::string::String),
            #[prost(bytes, arc, tag = "11")]
            B(::prost::alloc::vec::Vec<u8>),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for OneofField {
            #[inline]
            fn clone(&self) -> OneofField {
                match self {
                    OneofField::A(__self_0) => {
                        OneofField::A(::core::clone::Clone::clone(__self_0))
                    }
                    OneofField::B(__self_0) => {
                        OneofField::B(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OneofField {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for OneofField {
            #[inline]
            fn eq(&self, other: &OneofField) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (OneofField::A(__self_0), OneofField::A(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (OneofField::B(__self_0), OneofField::B(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl OneofField {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    OneofField::A(ref value) => {
                        ::prost::encoding::string::encode(10u32, &**value, buf);
                    }
                    OneofField::B(ref value) => {
                        ::prost::encoding::bytes::encode(11u32, &**value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<OneofField>,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    10u32 => {
                        match field {
                            ::core::option::Option::Some(
                                OneofField::A(ref mut value),
                            ) => {
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::prost::alloc::sync::Arc::new(
                                    ::prost::alloc::string::String::new(),
                                );
                                let value = &mut owned_value;
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            OneofField::A(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    11u32 => {
                        match field {
                            ::core::option::Option::Some(
                                OneofField::B(ref mut value),
                            ) => {
                                ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::prost::alloc::sync::Arc::new(
                                    ::core::default::Default::default(),
                                );
                                let value = &mut owned_value;
                                ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            OneofField::B(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid OneofField tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    OneofField::A(ref value) => {
                        ::prost::encoding::string::encoded_len(10u32, &**value)
                    }
                    OneofField::B(ref value) => {
                        ::prost::encoding::bytes::encoded_len(11u32, &**value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for OneofField {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    OneofField::A(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("A").field(&wrapper).finish()
                    }
                    OneofField::B(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("B").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    #[repr(i32)]
    pub enum MyEnum {
        Bar = 1,
        Foo = 2,
        Baz = 3,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MyEnum {
        #[inline]
        fn clone(&self) -> MyEnum {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MyEnum {}
    #[automatically_derived]
    impl ::core::fmt::Debug for MyEnum {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    MyEnum::Bar => "Bar",
                    MyEnum::Foo => "Foo",
                    MyEnum::Baz => "Baz",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MyEnum {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MyEnum {
        #[inline]
        fn eq(&self, other: &MyEnum) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for MyEnum {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for MyEnum {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for MyEnum {
        #[inline]
        fn partial_cmp(
            &self,
            other: &MyEnum,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for MyEnum {
        #[inline]
        fn cmp(&self, other: &MyEnum) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl MyEnum {
        ///Returns `true` if `value` is a variant of `MyEnum`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                1 => true,
                2 => true,
                3 => true,
                _ => false,
            }
        }
        #[deprecated = "Use the TryFrom<i32> implementation instead"]
        ///Converts an `i32` to a `MyEnum`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<MyEnum> {
            match value {
                1 => ::core::option::Option::Some(MyEnum::Bar),
                2 => ::core::option::Option::Some(MyEnum::Foo),
                3 => ::core::option::Option::Some(MyEnum::Baz),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for MyEnum {
        fn default() -> MyEnum {
            MyEnum::Bar
        }
    }
    impl ::core::convert::From<MyEnum> for i32 {
        fn from(value: MyEnum) -> i32 {
            value as i32
        }
    }
    impl ::core::convert::TryFrom<i32> for MyEnum {
        type Error = ::prost::UnknownEnumValue;
        fn try_from(
            value: i32,
        ) -> ::core::result::Result<MyEnum, ::prost::UnknownEnumValue> {
            match value {
                1 => ::core::result::Result::Ok(MyEnum::Bar),
                2 => ::core::result::Result::Ok(MyEnum::Foo),
                3 => ::core::result::Result::Ok(MyEnum::Baz),
                _ => ::core::result::Result::Err(::prost::UnknownEnumValue(value)),
            }
        }
    }
    impl MyEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Bar => "Bar",
                Self::Foo => "Foo",
                Self::Baz => "Baz",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Bar" => Some(Self::Bar),
                "Foo" => Some(Self::Foo),
                "Baz" => Some(Self::Baz),
                _ => None,
            }
        }
    }
}
#[rustfmt::skip]
pub mod wrappers_box {
    pub struct Payload {
        #[prost(int32, repeated, packed = "false", tag = "1")]
        pub stuff: ::prost::alloc::vec::Vec<i32>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for Payload {
        #[inline]
        fn clone(&self) -> Payload {
            Payload {
                stuff: ::core::clone::Clone::clone(&self.stuff),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for Payload {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for Payload {
        #[inline]
        fn eq(&self, other: &Payload) -> bool {
            self.stuff == other.stuff
        }
    }
    impl ::prost::Message for Payload {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            ::prost::encoding::int32::encode_repeated(1u32, &self.stuff, buf);
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "Payload";
            match tag {
                1u32 => {
                    let mut value = &mut self.stuff;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "stuff");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::int32::encoded_len_repeated(1u32, &self.stuff)
        }
        fn clear(&mut self) {
            self.stuff.clear();
        }
    }
    impl ::core::default::Default for Payload {
        fn default() -> Self {
            Payload {
                stuff: ::prost::alloc::vec::Vec::new(),
            }
        }
    }
    impl ::core::fmt::Debug for Payload {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("Payload");
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.stuff)
                };
                builder.field("stuff", &wrapper)
            };
            builder.finish()
        }
    }
    pub struct MyMessage {
        #[prost(int32, required, boxy, tag = "20")]
        pub int: ::prost::alloc::boxed::Box<i32>,
        #[prost(int32, optional, boxy, tag = "21")]
        pub optional_int: ::prost::alloc::boxed::Box<::core::option::Option<i32>>,
        #[prost(int32, repeated, packed = "false", boxy, tag = "22")]
        pub repeated_int: ::prost::alloc::boxed::Box<::prost::alloc::vec::Vec<i32>>,
        #[prost(int32, repeated, boxy, tag = "23")]
        pub packed_int: ::prost::alloc::boxed::Box<::prost::alloc::vec::Vec<i32>>,
        #[prost(string, required, boxy, tag = "1")]
        pub str: ::prost::alloc::boxed::Box<::prost::alloc::string::String>,
        #[prost(string, optional, boxy, tag = "2")]
        pub optional_str: ::prost::alloc::boxed::Box<
            ::core::option::Option<::prost::alloc::string::String>,
        >,
        #[prost(string, repeated, boxy, tag = "16")]
        pub repeated_str: ::prost::alloc::boxed::Box<
            ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
        >,
        #[prost(message, required, boxy, tag = "3")]
        pub payload: ::prost::alloc::boxed::Box<Payload>,
        #[prost(message, optional, boxy, tag = "4")]
        pub optional_payload: ::prost::alloc::boxed::Box<
            ::core::option::Option<Payload>,
        >,
        #[prost(message, repeated, boxy, tag = "17")]
        pub repeated_payload: ::prost::alloc::boxed::Box<
            ::prost::alloc::vec::Vec<Payload>,
        >,
        #[prost(btree_map = "int32, message", boxy, tag = "5")]
        pub map_payload: ::prost::alloc::boxed::Box<
            ::prost::alloc::collections::BTreeMap<i32, Payload>,
        >,
        #[prost(group, required, boxy, tag = "6")]
        pub group: ::prost::alloc::boxed::Box<my_message::Group>,
        #[prost(group, optional, boxy, tag = "8")]
        pub optional_group: ::prost::alloc::boxed::Box<
            ::core::option::Option<my_message::OptionalGroup>,
        >,
        #[prost(group, repeated, boxy, tag = "18")]
        pub repeated_group: ::prost::alloc::boxed::Box<
            ::prost::alloc::vec::Vec<my_message::RepeatedGroup>,
        >,
        #[prost(enumeration = "MyEnum", required, boxy, tag = "12")]
        pub my_enum: ::prost::alloc::boxed::Box<i32>,
        #[prost(enumeration = "MyEnum", optional, boxy, tag = "13")]
        pub optional_my_enum: ::prost::alloc::boxed::Box<::core::option::Option<i32>>,
        #[prost(enumeration = "MyEnum", repeated, packed = "false", boxy, tag = "14")]
        pub repeated_my_enum: ::prost::alloc::boxed::Box<::prost::alloc::vec::Vec<i32>>,
        #[prost(enumeration = "MyEnum", repeated, boxy, tag = "15")]
        pub packed_my_enum: ::prost::alloc::boxed::Box<::prost::alloc::vec::Vec<i32>>,
        /// default tests:
        #[prost(int32, optional, boxy, tag = "24", default = "42")]
        pub default_int: ::prost::alloc::boxed::Box<::core::option::Option<i32>>,
        #[prost(float, optional, boxy, tag = "25", default = "1")]
        pub default_float: ::prost::alloc::boxed::Box<::core::option::Option<f32>>,
        #[prost(string, optional, boxy, tag = "26", default = "foobar")]
        pub default_string: ::prost::alloc::boxed::Box<
            ::core::option::Option<::prost::alloc::string::String>,
        >,
        #[prost(oneof = "my_message::OneofField", boxy, tags = "10, 11")]
        pub oneof_field: ::prost::alloc::boxed::Box<
            ::core::option::Option<my_message::OneofField>,
        >,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MyMessage {
        #[inline]
        fn clone(&self) -> MyMessage {
            MyMessage {
                int: ::core::clone::Clone::clone(&self.int),
                optional_int: ::core::clone::Clone::clone(&self.optional_int),
                repeated_int: ::core::clone::Clone::clone(&self.repeated_int),
                packed_int: ::core::clone::Clone::clone(&self.packed_int),
                str: ::core::clone::Clone::clone(&self.str),
                optional_str: ::core::clone::Clone::clone(&self.optional_str),
                repeated_str: ::core::clone::Clone::clone(&self.repeated_str),
                payload: ::core::clone::Clone::clone(&self.payload),
                optional_payload: ::core::clone::Clone::clone(&self.optional_payload),
                repeated_payload: ::core::clone::Clone::clone(&self.repeated_payload),
                map_payload: ::core::clone::Clone::clone(&self.map_payload),
                group: ::core::clone::Clone::clone(&self.group),
                optional_group: ::core::clone::Clone::clone(&self.optional_group),
                repeated_group: ::core::clone::Clone::clone(&self.repeated_group),
                my_enum: ::core::clone::Clone::clone(&self.my_enum),
                optional_my_enum: ::core::clone::Clone::clone(&self.optional_my_enum),
                repeated_my_enum: ::core::clone::Clone::clone(&self.repeated_my_enum),
                packed_my_enum: ::core::clone::Clone::clone(&self.packed_my_enum),
                default_int: ::core::clone::Clone::clone(&self.default_int),
                default_float: ::core::clone::Clone::clone(&self.default_float),
                default_string: ::core::clone::Clone::clone(&self.default_string),
                oneof_field: ::core::clone::Clone::clone(&self.oneof_field),
            }
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MyMessage {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MyMessage {
        #[inline]
        fn eq(&self, other: &MyMessage) -> bool {
            self.int == other.int && self.optional_int == other.optional_int
                && self.repeated_int == other.repeated_int
                && self.packed_int == other.packed_int && self.str == other.str
                && self.optional_str == other.optional_str
                && self.repeated_str == other.repeated_str
                && self.payload == other.payload
                && self.optional_payload == other.optional_payload
                && self.repeated_payload == other.repeated_payload
                && self.map_payload == other.map_payload && self.group == other.group
                && self.optional_group == other.optional_group
                && self.repeated_group == other.repeated_group
                && self.my_enum == other.my_enum
                && self.optional_my_enum == other.optional_my_enum
                && self.repeated_my_enum == other.repeated_my_enum
                && self.packed_my_enum == other.packed_my_enum
                && self.default_int == other.default_int
                && self.default_float == other.default_float
                && self.default_string == other.default_string
                && self.oneof_field == other.oneof_field
        }
    }
    impl ::prost::Message for MyMessage {
        #[allow(unused_variables)]
        fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
            ::prost::encoding::string::encode(1u32, &*self.str, buf);
            if let ::core::option::Option::Some(value) = &*self.optional_str {
                ::prost::encoding::string::encode(2u32, value, buf);
            }
            ::prost::encoding::message::encode(3u32, &*self.payload, buf);
            if let Some(msg) = &*self.optional_payload {
                ::prost::encoding::message::encode(4u32, msg, buf);
            }
            ::prost::encoding::btree_map::encode(
                ::prost::encoding::int32::encode,
                ::prost::encoding::int32::encoded_len,
                ::prost::encoding::message::encode,
                ::prost::encoding::message::encoded_len,
                5u32,
                &self.map_payload,
                buf,
            );
            ::prost::encoding::group::encode(6u32, &*self.group, buf);
            if let Some(msg) = &*self.optional_group {
                ::prost::encoding::group::encode(8u32, msg, buf);
            }
            if let Some(oneof) = &*self.oneof_field {
                oneof.encode(buf)
            }
            ::prost::encoding::int32::encode(12u32, &*self.my_enum, buf);
            if let ::core::option::Option::Some(value) = &*self.optional_my_enum {
                ::prost::encoding::int32::encode(13u32, value, buf);
            }
            ::prost::encoding::int32::encode_repeated(
                14u32,
                &*self.repeated_my_enum,
                buf,
            );
            ::prost::encoding::int32::encode_packed(15u32, &*self.packed_my_enum, buf);
            ::prost::encoding::string::encode_repeated(16u32, &*self.repeated_str, buf);
            for msg in &*self.repeated_payload {
                ::prost::encoding::message::encode(17u32, msg, buf);
            }
            for msg in &*self.repeated_group {
                ::prost::encoding::group::encode(18u32, msg, buf);
            }
            ::prost::encoding::int32::encode(20u32, &*self.int, buf);
            if let ::core::option::Option::Some(value) = &*self.optional_int {
                ::prost::encoding::int32::encode(21u32, value, buf);
            }
            ::prost::encoding::int32::encode_repeated(22u32, &*self.repeated_int, buf);
            ::prost::encoding::int32::encode_packed(23u32, &*self.packed_int, buf);
            if let ::core::option::Option::Some(value) = &*self.default_int {
                ::prost::encoding::int32::encode(24u32, value, buf);
            }
            if let ::core::option::Option::Some(value) = &*self.default_float {
                ::prost::encoding::float::encode(25u32, value, buf);
            }
            if let ::core::option::Option::Some(value) = &*self.default_string {
                ::prost::encoding::string::encode(26u32, value, buf);
            }
        }
        #[allow(unused_variables)]
        fn merge_field(
            &mut self,
            tag: u32,
            wire_type: ::prost::encoding::wire_type::WireType,
            buf: &mut impl ::prost::bytes::Buf,
            ctx: ::prost::encoding::DecodeContext,
        ) -> ::core::result::Result<(), ::prost::DecodeError> {
            const STRUCT_NAME: &'static str = "MyMessage";
            match tag {
                1u32 => {
                    let mut value = &mut *self.str;
                    ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "str");
                            error
                        })
                }
                2u32 => {
                    let mut value = &mut *self.optional_str;
                    ::prost::encoding::string::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "optional_str");
                            error
                        })
                }
                3u32 => {
                    let mut value = &mut *self.payload;
                    ::prost::encoding::message::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "payload");
                            error
                        })
                }
                4u32 => {
                    let mut value = &mut *self.optional_payload;
                    ::prost::encoding::message::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "optional_payload");
                            error
                        })
                }
                5u32 => {
                    let mut value = &mut *self.map_payload;
                    ::prost::encoding::btree_map::merge(
                            ::prost::encoding::int32::merge,
                            ::prost::encoding::message::merge,
                            &mut value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "map_payload");
                            error
                        })
                }
                6u32 => {
                    let mut value = &mut *self.group;
                    ::prost::encoding::group::merge(tag, wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "group");
                            error
                        })
                }
                8u32 => {
                    let mut value = &mut *self.optional_group;
                    ::prost::encoding::group::merge(
                            tag,
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "optional_group");
                            error
                        })
                }
                10u32 | 11u32 => {
                    let mut value = &mut *self.oneof_field;
                    my_message::OneofField::merge(value, tag, wire_type, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "oneof_field");
                            error
                        })
                }
                12u32 => {
                    let mut value = &mut *self.my_enum;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "my_enum");
                            error
                        })
                }
                13u32 => {
                    let mut value = &mut *self.optional_my_enum;
                    ::prost::encoding::int32::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "optional_my_enum");
                            error
                        })
                }
                14u32 => {
                    let mut value = &mut *self.repeated_my_enum;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "repeated_my_enum");
                            error
                        })
                }
                15u32 => {
                    let mut value = &mut *self.packed_my_enum;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "packed_my_enum");
                            error
                        })
                }
                16u32 => {
                    let mut value = &mut *self.repeated_str;
                    ::prost::encoding::string::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "repeated_str");
                            error
                        })
                }
                17u32 => {
                    let mut value = &mut *self.repeated_payload;
                    ::prost::encoding::message::merge_repeated(
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "repeated_payload");
                            error
                        })
                }
                18u32 => {
                    let mut value = &mut *self.repeated_group;
                    ::prost::encoding::group::merge_repeated(
                            tag,
                            wire_type,
                            value,
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "repeated_group");
                            error
                        })
                }
                20u32 => {
                    let mut value = &mut *self.int;
                    ::prost::encoding::int32::merge(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "int");
                            error
                        })
                }
                21u32 => {
                    let mut value = &mut *self.optional_int;
                    ::prost::encoding::int32::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "optional_int");
                            error
                        })
                }
                22u32 => {
                    let mut value = &mut *self.repeated_int;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "repeated_int");
                            error
                        })
                }
                23u32 => {
                    let mut value = &mut *self.packed_int;
                    ::prost::encoding::int32::merge_repeated(wire_type, value, buf, ctx)
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "packed_int");
                            error
                        })
                }
                24u32 => {
                    let mut value = &mut *self.default_int;
                    ::prost::encoding::int32::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "default_int");
                            error
                        })
                }
                25u32 => {
                    let mut value = &mut *self.default_float;
                    ::prost::encoding::float::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "default_float");
                            error
                        })
                }
                26u32 => {
                    let mut value = &mut *self.default_string;
                    ::prost::encoding::string::merge(
                            wire_type,
                            value.get_or_insert_with(::core::default::Default::default),
                            buf,
                            ctx,
                        )
                        .map_err(|mut error| {
                            error.push(STRUCT_NAME, "default_string");
                            error
                        })
                }
                _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
            }
        }
        #[inline]
        fn encoded_len(&self) -> usize {
            0 + ::prost::encoding::string::encoded_len(1u32, &*self.str)
                + (&*self.optional_str)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::string::encoded_len(2u32, value),
                    ) + ::prost::encoding::message::encoded_len(3u32, &*self.payload)
                + (&*self.optional_payload)
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::message::encoded_len(4u32, msg))
                + ::prost::encoding::btree_map::encoded_len(
                    ::prost::encoding::int32::encoded_len,
                    ::prost::encoding::message::encoded_len,
                    5u32,
                    &self.map_payload,
                ) + ::prost::encoding::group::encoded_len(6u32, &*self.group)
                + (&*self.optional_group)
                    .as_ref()
                    .map_or(0, |msg| ::prost::encoding::group::encoded_len(8u32, msg))
                + (&*self.oneof_field)
                    .as_ref()
                    .map_or(0, my_message::OneofField::encoded_len)
                + ::prost::encoding::int32::encoded_len(12u32, &*self.my_enum)
                + (&*self.optional_my_enum)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::int32::encoded_len(13u32, value),
                    )
                + ::prost::encoding::int32::encoded_len_repeated(
                    14u32,
                    &*self.repeated_my_enum,
                )
                + ::prost::encoding::int32::encoded_len_packed(
                    15u32,
                    &*self.packed_my_enum,
                )
                + ::prost::encoding::string::encoded_len_repeated(
                    16u32,
                    &*self.repeated_str,
                )
                + ::prost::encoding::message::encoded_len_repeated(
                    17u32,
                    &*self.repeated_payload,
                )
                + ::prost::encoding::group::encoded_len_repeated(
                    18u32,
                    &*self.repeated_group,
                ) + ::prost::encoding::int32::encoded_len(20u32, &*self.int)
                + (&*self.optional_int)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::int32::encoded_len(21u32, value),
                    )
                + ::prost::encoding::int32::encoded_len_repeated(
                    22u32,
                    &*self.repeated_int,
                )
                + ::prost::encoding::int32::encoded_len_packed(23u32, &*self.packed_int)
                + (&*self.default_int)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::int32::encoded_len(24u32, value),
                    )
                + (&*self.default_float)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::float::encoded_len(25u32, value),
                    )
                + (&*self.default_string)
                    .as_ref()
                    .map_or(
                        0,
                        |value| ::prost::encoding::string::encoded_len(26u32, value),
                    )
        }
        fn clear(&mut self) {
            self.str.clear();
            self.optional_str.take();
            self.payload.clear();
            self.optional_payload.take();
            self.map_payload.clear();
            self.group.clear();
            self.optional_group.take();
            self.oneof_field.take();
            self.my_enum = ::prost::alloc::boxed::Box::new(MyEnum::default() as i32);
            self.optional_my_enum.take();
            self.repeated_my_enum.clear();
            self.packed_my_enum.clear();
            self.repeated_str.clear();
            self.repeated_payload.clear();
            self.repeated_group.clear();
            self.int = ::prost::alloc::boxed::Box::new(0i32);
            self.optional_int.take();
            self.repeated_int.clear();
            self.packed_int.clear();
            self.default_int.take();
            self.default_float.take();
            self.default_string.take();
        }
    }
    impl ::core::default::Default for MyMessage {
        fn default() -> Self {
            MyMessage {
                str: ::prost::alloc::boxed::Box::new(
                    ::prost::alloc::string::String::new(),
                ),
                optional_str: ::prost::alloc::boxed::Box::new(
                    ::core::option::Option::None,
                ),
                payload: ::prost::alloc::boxed::Box::new(
                    ::core::default::Default::default(),
                ),
                optional_payload: ::prost::alloc::boxed::Box::new(
                    ::core::default::Default::default(),
                ),
                map_payload: ::prost::alloc::boxed::Box::new(
                    ::core::default::Default::default(),
                ),
                group: ::prost::alloc::boxed::Box::new(
                    ::core::default::Default::default(),
                ),
                optional_group: ::prost::alloc::boxed::Box::new(
                    ::core::default::Default::default(),
                ),
                oneof_field: ::prost::alloc::boxed::Box::new(
                    ::core::default::Default::default(),
                ),
                my_enum: ::prost::alloc::boxed::Box::new(MyEnum::default() as i32),
                optional_my_enum: ::prost::alloc::boxed::Box::new(
                    ::core::option::Option::None,
                ),
                repeated_my_enum: ::prost::alloc::boxed::Box::new(
                    ::prost::alloc::vec::Vec::new(),
                ),
                packed_my_enum: ::prost::alloc::boxed::Box::new(
                    ::prost::alloc::vec::Vec::new(),
                ),
                repeated_str: ::prost::alloc::boxed::Box::new(
                    ::prost::alloc::vec::Vec::new(),
                ),
                repeated_payload: ::prost::alloc::boxed::Box::new(
                    ::core::default::Default::default(),
                ),
                repeated_group: ::prost::alloc::boxed::Box::new(
                    ::core::default::Default::default(),
                ),
                int: ::prost::alloc::boxed::Box::new(0i32),
                optional_int: ::prost::alloc::boxed::Box::new(
                    ::core::option::Option::None,
                ),
                repeated_int: ::prost::alloc::boxed::Box::new(
                    ::prost::alloc::vec::Vec::new(),
                ),
                packed_int: ::prost::alloc::boxed::Box::new(
                    ::prost::alloc::vec::Vec::new(),
                ),
                default_int: ::prost::alloc::boxed::Box::new(
                    ::core::option::Option::None,
                ),
                default_float: ::prost::alloc::boxed::Box::new(
                    ::core::option::Option::None,
                ),
                default_string: ::prost::alloc::boxed::Box::new(
                    ::core::option::Option::None,
                ),
            }
        }
    }
    impl ::core::fmt::Debug for MyMessage {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let mut builder = f.debug_struct("MyMessage");
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.int)
                };
                builder.field("int", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.optional_int)
                };
                builder.field("optional_int", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.repeated_int)
                };
                builder.field("repeated_int", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.packed_int)
                };
                builder.field("packed_int", &wrapper)
            };
            let builder = {
                let wrapper = {
                    #[allow(non_snake_case)]
                    fn ScalarWrapper<T>(v: T) -> T {
                        v
                    }
                    ScalarWrapper(&self.str)
                };
                builder.field("str", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::core::option::Option<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.optional_str)
                };
                builder.field("optional_str", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.repeated_str)
                };
                builder.field("repeated_str", &wrapper)
            };
            let builder = {
                let wrapper = &self.payload;
                builder.field("payload", &wrapper)
            };
            let builder = {
                let wrapper = &self.optional_payload;
                builder.field("optional_payload", &wrapper)
            };
            let builder = {
                let wrapper = &self.repeated_payload;
                builder.field("repeated_payload", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct MapWrapper<'a, V: 'a>(
                        &'a ::prost::alloc::collections::BTreeMap<i32, V>,
                    );
                    impl<'a, V> ::core::fmt::Debug for MapWrapper<'a, V>
                    where
                        V: ::core::fmt::Debug + 'a,
                    {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn KeyWrapper<T>(v: T) -> T {
                                v
                            }
                            fn ValueWrapper<T>(v: T) -> T {
                                v
                            }
                            let mut builder = f.debug_map();
                            for (k, v) in self.0 {
                                builder.entry(&KeyWrapper(k), &ValueWrapper(v));
                            }
                            builder.finish()
                        }
                    }
                    MapWrapper(&self.map_payload)
                };
                builder.field("map_payload", &wrapper)
            };
            let builder = {
                let wrapper = &self.group;
                builder.field("group", &wrapper)
            };
            let builder = {
                let wrapper = &self.optional_group;
                builder.field("optional_group", &wrapper)
            };
            let builder = {
                let wrapper = &self.repeated_group;
                builder.field("repeated_group", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a i32);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let res: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                                *self.0,
                            );
                            match res {
                                Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                            }
                        }
                    }
                    ScalarWrapper(&self.my_enum)
                };
                builder.field("my_enum", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            struct Inner<'a>(&'a i32);
                            impl<'a> ::core::fmt::Debug for Inner<'a> {
                                fn fmt(
                                    &self,
                                    f: &mut ::core::fmt::Formatter,
                                ) -> ::core::fmt::Result {
                                    let res: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                                        *self.0,
                                    );
                                    match res {
                                        Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                        Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                                    }
                                }
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.optional_my_enum)
                };
                builder.field("optional_my_enum", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                struct Inner<'a>(&'a i32);
                                impl<'a> ::core::fmt::Debug for Inner<'a> {
                                    fn fmt(
                                        &self,
                                        f: &mut ::core::fmt::Formatter,
                                    ) -> ::core::fmt::Result {
                                        let res: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                                            *self.0,
                                        );
                                        match res {
                                            Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                            Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                                        }
                                    }
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.repeated_my_enum)
                };
                builder.field("repeated_my_enum", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::prost::alloc::vec::Vec<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            let mut vec_builder = f.debug_list();
                            for v in self.0 {
                                struct Inner<'a>(&'a i32);
                                impl<'a> ::core::fmt::Debug for Inner<'a> {
                                    fn fmt(
                                        &self,
                                        f: &mut ::core::fmt::Formatter,
                                    ) -> ::core::fmt::Result {
                                        let res: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                                            *self.0,
                                        );
                                        match res {
                                            Err(_) => ::core::fmt::Debug::fmt(&self.0, f),
                                            Ok(en) => ::core::fmt::Debug::fmt(&en, f),
                                        }
                                    }
                                }
                                vec_builder.entry(&Inner(v));
                            }
                            vec_builder.finish()
                        }
                    }
                    ScalarWrapper(&self.packed_my_enum)
                };
                builder.field("packed_my_enum", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.default_int)
                };
                builder.field("default_int", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(&'a ::core::option::Option<f32>);
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.default_float)
                };
                builder.field("default_float", &wrapper)
            };
            let builder = {
                let wrapper = {
                    struct ScalarWrapper<'a>(
                        &'a ::core::option::Option<::prost::alloc::string::String>,
                    );
                    impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                        fn fmt(
                            &self,
                            f: &mut ::core::fmt::Formatter,
                        ) -> ::core::fmt::Result {
                            #[allow(non_snake_case)]
                            fn Inner<T>(v: T) -> T {
                                v
                            }
                            ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                        }
                    }
                    ScalarWrapper(&self.default_string)
                };
                builder.field("default_string", &wrapper)
            };
            let builder = {
                let wrapper = &self.oneof_field;
                builder.field("oneof_field", &wrapper)
            };
            builder.finish()
        }
    }
    #[allow(dead_code)]
    impl MyMessage {
        ///Returns the value of `optional_str`, or the default value if `optional_str` is unset.
        pub fn optional_str(&self) -> &str {
            match (&*self.optional_str).as_ref() {
                ::core::option::Option::Some(val) => &val[..],
                ::core::option::Option::None => "",
            }
        }
        ///Returns the enum value of `my_enum`, or the default if the field is set to an invalid enum value.
        pub fn my_enum(&self) -> MyEnum {
            ::core::convert::TryFrom::try_from(*&*self.my_enum)
                .unwrap_or(MyEnum::default())
        }
        ///Sets `my_enum` to the provided enum value.
        pub fn set_my_enum(&mut self, value: MyEnum) {
            self.my_enum = ::prost::alloc::boxed::Box::new(value as i32);
        }
        ///Returns the enum value of `optional_my_enum`, or the default if the field is unset or set to an invalid enum value.
        pub fn optional_my_enum(&self) -> MyEnum {
            self.optional_my_enum
                .and_then(|x| {
                    let result: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                        x,
                    );
                    result.ok()
                })
                .unwrap_or(MyEnum::default())
        }
        ///Sets `optional_my_enum` to the provided enum value.
        pub fn set_optional_my_enum(&mut self, value: MyEnum) {
            self.optional_my_enum = ::prost::alloc::boxed::Box::new(
                ::core::option::Option::Some(value as i32),
            );
        }
        ///Returns an iterator which yields the valid enum values contained in `repeated_my_enum`.
        pub fn repeated_my_enum(
            &self,
        ) -> ::core::iter::FilterMap<
            ::core::iter::Cloned<::core::slice::Iter<i32>>,
            fn(i32) -> ::core::option::Option<MyEnum>,
        > {
            self.repeated_my_enum
                .iter()
                .cloned()
                .filter_map(|x| {
                    let result: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                        x,
                    );
                    result.ok()
                })
        }
        ///Appends the provided enum value to `repeated_my_enum`.
        pub fn push_repeated_my_enum(&mut self, value: MyEnum) {
            let mut pushed = (*&*self.repeated_my_enum).clone();
            pushed.push(value as i32);
            self.repeated_my_enum = ::prost::alloc::boxed::Box::new(pushed);
        }
        ///Returns an iterator which yields the valid enum values contained in `packed_my_enum`.
        pub fn packed_my_enum(
            &self,
        ) -> ::core::iter::FilterMap<
            ::core::iter::Cloned<::core::slice::Iter<i32>>,
            fn(i32) -> ::core::option::Option<MyEnum>,
        > {
            self.packed_my_enum
                .iter()
                .cloned()
                .filter_map(|x| {
                    let result: ::core::result::Result<MyEnum, _> = ::core::convert::TryFrom::try_from(
                        x,
                    );
                    result.ok()
                })
        }
        ///Appends the provided enum value to `packed_my_enum`.
        pub fn push_packed_my_enum(&mut self, value: MyEnum) {
            let mut pushed = (*&*self.packed_my_enum).clone();
            pushed.push(value as i32);
            self.packed_my_enum = ::prost::alloc::boxed::Box::new(pushed);
        }
        ///Returns the value of `optional_int`, or the default value if `optional_int` is unset.
        pub fn optional_int(&self) -> i32 {
            match (&*self.optional_int).as_ref() {
                ::core::option::Option::Some(val) => *val,
                ::core::option::Option::None => 0i32,
            }
        }
        ///Returns the value of `default_int`, or the default value if `default_int` is unset.
        pub fn default_int(&self) -> i32 {
            match (&*self.default_int).as_ref() {
                ::core::option::Option::Some(val) => *val,
                ::core::option::Option::None => 42i32,
            }
        }
        ///Returns the value of `default_float`, or the default value if `default_float` is unset.
        pub fn default_float(&self) -> f32 {
            match (&*self.default_float).as_ref() {
                ::core::option::Option::Some(val) => *val,
                ::core::option::Option::None => 1f32,
            }
        }
        ///Returns the value of `default_string`, or the default value if `default_string` is unset.
        pub fn default_string(&self) -> &str {
            match (&*self.default_string).as_ref() {
                ::core::option::Option::Some(val) => &val[..],
                ::core::option::Option::None => "foobar",
            }
        }
    }
    /// Nested message and enum types in `MyMessage`.
    pub mod my_message {
        pub struct Group {
            #[prost(int32, optional, tag = "7")]
            pub i2: ::core::option::Option<i32>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Group {
            #[inline]
            fn clone(&self) -> Group {
                let _: ::core::clone::AssertParamIsClone<::core::option::Option<i32>>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for Group {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for Group {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for Group {
            #[inline]
            fn eq(&self, other: &Group) -> bool {
                self.i2 == other.i2
            }
        }
        impl ::prost::Message for Group {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let ::core::option::Option::Some(value) = &self.i2 {
                    ::prost::encoding::int32::encode(7u32, value, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "Group";
                match tag {
                    7u32 => {
                        let mut value = &mut self.i2;
                        ::prost::encoding::int32::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "i2");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + (&self.i2)
                        .as_ref()
                        .map_or(
                            0,
                            |value| ::prost::encoding::int32::encoded_len(7u32, value),
                        )
            }
            fn clear(&mut self) {
                self.i2.take();
            }
        }
        impl ::core::default::Default for Group {
            fn default() -> Self {
                Group {
                    i2: ::core::option::Option::None,
                }
            }
        }
        impl ::core::fmt::Debug for Group {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("Group");
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                            }
                        }
                        ScalarWrapper(&self.i2)
                    };
                    builder.field("i2", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(dead_code)]
        impl Group {
            ///Returns the value of `i2`, or the default value if `i2` is unset.
            pub fn i2(&self) -> i32 {
                match (&self.i2).as_ref() {
                    ::core::option::Option::Some(val) => *val,
                    ::core::option::Option::None => 0i32,
                }
            }
        }
        pub struct OptionalGroup {
            #[prost(int32, optional, tag = "9")]
            pub i2: ::core::option::Option<i32>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for OptionalGroup {
            #[inline]
            fn clone(&self) -> OptionalGroup {
                let _: ::core::clone::AssertParamIsClone<::core::option::Option<i32>>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for OptionalGroup {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OptionalGroup {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for OptionalGroup {
            #[inline]
            fn eq(&self, other: &OptionalGroup) -> bool {
                self.i2 == other.i2
            }
        }
        impl ::prost::Message for OptionalGroup {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let ::core::option::Option::Some(value) = &self.i2 {
                    ::prost::encoding::int32::encode(9u32, value, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "OptionalGroup";
                match tag {
                    9u32 => {
                        let mut value = &mut self.i2;
                        ::prost::encoding::int32::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "i2");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + (&self.i2)
                        .as_ref()
                        .map_or(
                            0,
                            |value| ::prost::encoding::int32::encoded_len(9u32, value),
                        )
            }
            fn clear(&mut self) {
                self.i2.take();
            }
        }
        impl ::core::default::Default for OptionalGroup {
            fn default() -> Self {
                OptionalGroup {
                    i2: ::core::option::Option::None,
                }
            }
        }
        impl ::core::fmt::Debug for OptionalGroup {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("OptionalGroup");
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                            }
                        }
                        ScalarWrapper(&self.i2)
                    };
                    builder.field("i2", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(dead_code)]
        impl OptionalGroup {
            ///Returns the value of `i2`, or the default value if `i2` is unset.
            pub fn i2(&self) -> i32 {
                match (&self.i2).as_ref() {
                    ::core::option::Option::Some(val) => *val,
                    ::core::option::Option::None => 0i32,
                }
            }
        }
        pub struct RepeatedGroup {
            #[prost(int32, optional, tag = "19")]
            pub i2: ::core::option::Option<i32>,
        }
        #[automatically_derived]
        impl ::core::clone::Clone for RepeatedGroup {
            #[inline]
            fn clone(&self) -> RepeatedGroup {
                let _: ::core::clone::AssertParamIsClone<::core::option::Option<i32>>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for RepeatedGroup {}
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for RepeatedGroup {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for RepeatedGroup {
            #[inline]
            fn eq(&self, other: &RepeatedGroup) -> bool {
                self.i2 == other.i2
            }
        }
        impl ::prost::Message for RepeatedGroup {
            #[allow(unused_variables)]
            fn encode_raw(&self, buf: &mut impl ::prost::bytes::BufMut) {
                if let ::core::option::Option::Some(value) = &self.i2 {
                    ::prost::encoding::int32::encode(19u32, value, buf);
                }
            }
            #[allow(unused_variables)]
            fn merge_field(
                &mut self,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                const STRUCT_NAME: &'static str = "RepeatedGroup";
                match tag {
                    19u32 => {
                        let mut value = &mut self.i2;
                        ::prost::encoding::int32::merge(
                                wire_type,
                                value.get_or_insert_with(::core::default::Default::default),
                                buf,
                                ctx,
                            )
                            .map_err(|mut error| {
                                error.push(STRUCT_NAME, "i2");
                                error
                            })
                    }
                    _ => ::prost::encoding::skip_field(wire_type, tag, buf, ctx),
                }
            }
            #[inline]
            fn encoded_len(&self) -> usize {
                0
                    + (&self.i2)
                        .as_ref()
                        .map_or(
                            0,
                            |value| ::prost::encoding::int32::encoded_len(19u32, value),
                        )
            }
            fn clear(&mut self) {
                self.i2.take();
            }
        }
        impl ::core::default::Default for RepeatedGroup {
            fn default() -> Self {
                RepeatedGroup {
                    i2: ::core::option::Option::None,
                }
            }
        }
        impl ::core::fmt::Debug for RepeatedGroup {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                let mut builder = f.debug_struct("RepeatedGroup");
                let builder = {
                    let wrapper = {
                        struct ScalarWrapper<'a>(&'a ::core::option::Option<i32>);
                        impl<'a> ::core::fmt::Debug for ScalarWrapper<'a> {
                            fn fmt(
                                &self,
                                f: &mut ::core::fmt::Formatter,
                            ) -> ::core::fmt::Result {
                                #[allow(non_snake_case)]
                                fn Inner<T>(v: T) -> T {
                                    v
                                }
                                ::core::fmt::Debug::fmt(&self.0.as_ref().map(Inner), f)
                            }
                        }
                        ScalarWrapper(&self.i2)
                    };
                    builder.field("i2", &wrapper)
                };
                builder.finish()
            }
        }
        #[allow(dead_code)]
        impl RepeatedGroup {
            ///Returns the value of `i2`, or the default value if `i2` is unset.
            pub fn i2(&self) -> i32 {
                match (&self.i2).as_ref() {
                    ::core::option::Option::Some(val) => *val,
                    ::core::option::Option::None => 0i32,
                }
            }
        }
        pub enum OneofField {
            #[prost(string, boxy, tag = "10")]
            A(::prost::alloc::boxed::Box<::prost::alloc::string::String>),
            #[prost(bytes, boxy, tag = "11")]
            B(::prost::alloc::boxed::Box<::prost::alloc::vec::Vec<u8>>),
        }
        #[automatically_derived]
        impl ::core::clone::Clone for OneofField {
            #[inline]
            fn clone(&self) -> OneofField {
                match self {
                    OneofField::A(__self_0) => {
                        OneofField::A(::core::clone::Clone::clone(__self_0))
                    }
                    OneofField::B(__self_0) => {
                        OneofField::B(::core::clone::Clone::clone(__self_0))
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::marker::StructuralPartialEq for OneofField {}
        #[automatically_derived]
        impl ::core::cmp::PartialEq for OneofField {
            #[inline]
            fn eq(&self, other: &OneofField) -> bool {
                let __self_discr = ::core::intrinsics::discriminant_value(self);
                let __arg1_discr = ::core::intrinsics::discriminant_value(other);
                __self_discr == __arg1_discr
                    && match (self, other) {
                        (OneofField::A(__self_0), OneofField::A(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        (OneofField::B(__self_0), OneofField::B(__arg1_0)) => {
                            __self_0 == __arg1_0
                        }
                        _ => unsafe { ::core::intrinsics::unreachable() }
                    }
            }
        }
        impl OneofField {
            /// Encodes the message to a buffer.
            pub fn encode(&self, buf: &mut impl ::prost::bytes::BufMut) {
                match *self {
                    OneofField::A(ref value) => {
                        ::prost::encoding::string::encode(10u32, &**value, buf);
                    }
                    OneofField::B(ref value) => {
                        ::prost::encoding::bytes::encode(11u32, &**value, buf);
                    }
                }
            }
            /// Decodes an instance of the message from a buffer, and merges it into self.
            pub fn merge(
                field: &mut ::core::option::Option<OneofField>,
                tag: u32,
                wire_type: ::prost::encoding::wire_type::WireType,
                buf: &mut impl ::prost::bytes::Buf,
                ctx: ::prost::encoding::DecodeContext,
            ) -> ::core::result::Result<(), ::prost::DecodeError> {
                match tag {
                    10u32 => {
                        match field {
                            ::core::option::Option::Some(
                                OneofField::A(ref mut value),
                            ) => {
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::prost::alloc::boxed::Box::new(
                                    ::prost::alloc::string::String::new(),
                                );
                                let value = &mut owned_value;
                                ::prost::encoding::string::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            OneofField::A(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    11u32 => {
                        match field {
                            ::core::option::Option::Some(
                                OneofField::B(ref mut value),
                            ) => {
                                ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                            }
                            _ => {
                                let mut owned_value = ::prost::alloc::boxed::Box::new(
                                    ::core::default::Default::default(),
                                );
                                let value = &mut owned_value;
                                ::prost::encoding::bytes::merge(wire_type, value, buf, ctx)
                                    .map(|_| {
                                        *field = ::core::option::Option::Some(
                                            OneofField::B(owned_value),
                                        );
                                    })
                            }
                        }
                    }
                    _ => {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "internal error: entered unreachable code: {0}",
                                format_args!("invalid OneofField tag: {0}", tag),
                            ),
                        );
                    }
                }
            }
            /// Returns the encoded length of the message without a length delimiter.
            #[inline]
            pub fn encoded_len(&self) -> usize {
                match *self {
                    OneofField::A(ref value) => {
                        ::prost::encoding::string::encoded_len(10u32, &**value)
                    }
                    OneofField::B(ref value) => {
                        ::prost::encoding::bytes::encoded_len(11u32, &**value)
                    }
                }
            }
        }
        impl ::core::fmt::Debug for OneofField {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                match *self {
                    OneofField::A(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("A").field(&wrapper).finish()
                    }
                    OneofField::B(ref value) => {
                        let wrapper = {
                            #[allow(non_snake_case)]
                            fn ScalarWrapper<T>(v: T) -> T {
                                v
                            }
                            ScalarWrapper(&*value)
                        };
                        f.debug_tuple("B").field(&wrapper).finish()
                    }
                }
            }
        }
    }
    #[repr(i32)]
    pub enum MyEnum {
        Bar = 1,
        Foo = 2,
        Baz = 3,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for MyEnum {
        #[inline]
        fn clone(&self) -> MyEnum {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for MyEnum {}
    #[automatically_derived]
    impl ::core::fmt::Debug for MyEnum {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::write_str(
                f,
                match self {
                    MyEnum::Bar => "Bar",
                    MyEnum::Foo => "Foo",
                    MyEnum::Baz => "Baz",
                },
            )
        }
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for MyEnum {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for MyEnum {
        #[inline]
        fn eq(&self, other: &MyEnum) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Eq for MyEnum {
        #[inline]
        #[doc(hidden)]
        #[coverage(off)]
        fn assert_receiver_is_total_eq(&self) -> () {}
    }
    #[automatically_derived]
    impl ::core::hash::Hash for MyEnum {
        #[inline]
        fn hash<__H: ::core::hash::Hasher>(&self, state: &mut __H) -> () {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            ::core::hash::Hash::hash(&__self_discr, state)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::PartialOrd for MyEnum {
        #[inline]
        fn partial_cmp(
            &self,
            other: &MyEnum,
        ) -> ::core::option::Option<::core::cmp::Ordering> {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::PartialOrd::partial_cmp(&__self_discr, &__arg1_discr)
        }
    }
    #[automatically_derived]
    impl ::core::cmp::Ord for MyEnum {
        #[inline]
        fn cmp(&self, other: &MyEnum) -> ::core::cmp::Ordering {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            ::core::cmp::Ord::cmp(&__self_discr, &__arg1_discr)
        }
    }
    impl MyEnum {
        ///Returns `true` if `value` is a variant of `MyEnum`.
        pub fn is_valid(value: i32) -> bool {
            match value {
                1 => true,
                2 => true,
                3 => true,
                _ => false,
            }
        }
        #[deprecated = "Use the TryFrom<i32> implementation instead"]
        ///Converts an `i32` to a `MyEnum`, or `None` if `value` is not a valid variant.
        pub fn from_i32(value: i32) -> ::core::option::Option<MyEnum> {
            match value {
                1 => ::core::option::Option::Some(MyEnum::Bar),
                2 => ::core::option::Option::Some(MyEnum::Foo),
                3 => ::core::option::Option::Some(MyEnum::Baz),
                _ => ::core::option::Option::None,
            }
        }
    }
    impl ::core::default::Default for MyEnum {
        fn default() -> MyEnum {
            MyEnum::Bar
        }
    }
    impl ::core::convert::From<MyEnum> for i32 {
        fn from(value: MyEnum) -> i32 {
            value as i32
        }
    }
    impl ::core::convert::TryFrom<i32> for MyEnum {
        type Error = ::prost::UnknownEnumValue;
        fn try_from(
            value: i32,
        ) -> ::core::result::Result<MyEnum, ::prost::UnknownEnumValue> {
            match value {
                1 => ::core::result::Result::Ok(MyEnum::Bar),
                2 => ::core::result::Result::Ok(MyEnum::Foo),
                3 => ::core::result::Result::Ok(MyEnum::Baz),
                _ => ::core::result::Result::Err(::prost::UnknownEnumValue(value)),
            }
        }
    }
    impl MyEnum {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Self::Bar => "Bar",
                Self::Foo => "Foo",
                Self::Baz => "Baz",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "Bar" => Some(Self::Bar),
                "Foo" => Some(Self::Foo),
                "Baz" => Some(Self::Baz),
                _ => None,
            }
        }
    }
}
