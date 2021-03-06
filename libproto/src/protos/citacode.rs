// CITA
// Copyright 2016-2018 Cryptape Technologies LLC.

// This program is free software: you can redistribute it
// and/or modify it under the terms of the GNU General Public
// License as published by the Free Software Foundation,
// either version 3 of the License, or (at your option) any
// later version.

// This program is distributed in the hope that it will be
// useful, but WITHOUT ANY WARRANTY; without even the implied
// warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR
// PURPOSE. See the GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// 3for 287494524@qq.com learning// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct ActionParams {
    // message fields
    pub code_address: ::std::string::String,
    pub sender: ::std::string::String,
    pub gas: ::std::string::String,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ActionParams {}

impl ActionParams {
    pub fn new() -> ActionParams {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ActionParams {
        static mut instance: ::protobuf::lazy::Lazy<ActionParams> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ActionParams,
        };
        unsafe {
            instance.get(ActionParams::new)
        }
    }

    // string code_address = 1;

    pub fn clear_code_address(&mut self) {
        self.code_address.clear();
    }

    // Param is passed by value, moved
    pub fn set_code_address(&mut self, v: ::std::string::String) {
        self.code_address = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_code_address(&mut self) -> &mut ::std::string::String {
        &mut self.code_address
    }

    // Take field
    pub fn take_code_address(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.code_address, ::std::string::String::new())
    }

    pub fn get_code_address(&self) -> &str {
        &self.code_address
    }

    fn get_code_address_for_reflect(&self) -> &::std::string::String {
        &self.code_address
    }

    fn mut_code_address_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.code_address
    }

    // string sender = 2;

    pub fn clear_sender(&mut self) {
        self.sender.clear();
    }

    // Param is passed by value, moved
    pub fn set_sender(&mut self, v: ::std::string::String) {
        self.sender = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sender(&mut self) -> &mut ::std::string::String {
        &mut self.sender
    }

    // Take field
    pub fn take_sender(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.sender, ::std::string::String::new())
    }

    pub fn get_sender(&self) -> &str {
        &self.sender
    }

    fn get_sender_for_reflect(&self) -> &::std::string::String {
        &self.sender
    }

    fn mut_sender_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.sender
    }

    // string gas = 3;

    pub fn clear_gas(&mut self) {
        self.gas.clear();
    }

    // Param is passed by value, moved
    pub fn set_gas(&mut self, v: ::std::string::String) {
        self.gas = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gas(&mut self) -> &mut ::std::string::String {
        &mut self.gas
    }

    // Take field
    pub fn take_gas(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gas, ::std::string::String::new())
    }

    pub fn get_gas(&self) -> &str {
        &self.gas
    }

    fn get_gas_for_reflect(&self) -> &::std::string::String {
        &self.gas
    }

    fn mut_gas_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.gas
    }

    // bytes data = 4;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for ActionParams {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.code_address)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.sender)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gas)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.code_address.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.code_address);
        }
        if !self.sender.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.sender);
        }
        if !self.gas.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.gas);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.code_address.is_empty() {
            os.write_string(1, &self.code_address)?;
        }
        if !self.sender.is_empty() {
            os.write_string(2, &self.sender)?;
        }
        if !self.gas.is_empty() {
            os.write_string(3, &self.gas)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(4, &self.data)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ActionParams {
    fn new() -> ActionParams {
        ActionParams::new()
    }

    fn descriptor_static(_: ::std::option::Option<ActionParams>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "code_address",
                    ActionParams::get_code_address_for_reflect,
                    ActionParams::mut_code_address_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "sender",
                    ActionParams::get_sender_for_reflect,
                    ActionParams::mut_sender_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gas",
                    ActionParams::get_gas_for_reflect,
                    ActionParams::mut_gas_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    ActionParams::get_data_for_reflect,
                    ActionParams::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ActionParams>(
                    "ActionParams",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ActionParams {
    fn clear(&mut self) {
        self.clear_code_address();
        self.clear_sender();
        self.clear_gas();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ActionParams {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ActionParams {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct EnvInfo {
    // message fields
    pub number: ::std::string::String,
    pub author: ::std::string::String,
    pub timestamp: ::std::string::String,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for EnvInfo {}

impl EnvInfo {
    pub fn new() -> EnvInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static EnvInfo {
        static mut instance: ::protobuf::lazy::Lazy<EnvInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const EnvInfo,
        };
        unsafe {
            instance.get(EnvInfo::new)
        }
    }

    // string number = 1;

    pub fn clear_number(&mut self) {
        self.number.clear();
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: ::std::string::String) {
        self.number = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_number(&mut self) -> &mut ::std::string::String {
        &mut self.number
    }

    // Take field
    pub fn take_number(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.number, ::std::string::String::new())
    }

    pub fn get_number(&self) -> &str {
        &self.number
    }

    fn get_number_for_reflect(&self) -> &::std::string::String {
        &self.number
    }

    fn mut_number_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.number
    }

    // string author = 2;

    pub fn clear_author(&mut self) {
        self.author.clear();
    }

    // Param is passed by value, moved
    pub fn set_author(&mut self, v: ::std::string::String) {
        self.author = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_author(&mut self) -> &mut ::std::string::String {
        &mut self.author
    }

    // Take field
    pub fn take_author(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.author, ::std::string::String::new())
    }

    pub fn get_author(&self) -> &str {
        &self.author
    }

    fn get_author_for_reflect(&self) -> &::std::string::String {
        &self.author
    }

    fn mut_author_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.author
    }

    // string timestamp = 3;

    pub fn clear_timestamp(&mut self) {
        self.timestamp.clear();
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: ::std::string::String) {
        self.timestamp = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timestamp(&mut self) -> &mut ::std::string::String {
        &mut self.timestamp
    }

    // Take field
    pub fn take_timestamp(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.timestamp, ::std::string::String::new())
    }

    pub fn get_timestamp(&self) -> &str {
        &self.timestamp
    }

    fn get_timestamp_for_reflect(&self) -> &::std::string::String {
        &self.timestamp
    }

    fn mut_timestamp_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.timestamp
    }
}

impl ::protobuf::Message for EnvInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.number)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.author)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.timestamp)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.number.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.number);
        }
        if !self.author.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.author);
        }
        if !self.timestamp.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.timestamp);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.number.is_empty() {
            os.write_string(1, &self.number)?;
        }
        if !self.author.is_empty() {
            os.write_string(2, &self.author)?;
        }
        if !self.timestamp.is_empty() {
            os.write_string(3, &self.timestamp)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for EnvInfo {
    fn new() -> EnvInfo {
        EnvInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<EnvInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "number",
                    EnvInfo::get_number_for_reflect,
                    EnvInfo::mut_number_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "author",
                    EnvInfo::get_author_for_reflect,
                    EnvInfo::mut_author_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "timestamp",
                    EnvInfo::get_timestamp_for_reflect,
                    EnvInfo::mut_timestamp_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<EnvInfo>(
                    "EnvInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for EnvInfo {
    fn clear(&mut self) {
        self.clear_number();
        self.clear_author();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for EnvInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for EnvInfo {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InvokeRequest {
    // message fields
    pub param: ::protobuf::SingularPtrField<ActionParams>,
    pub env_info: ::protobuf::SingularPtrField<EnvInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InvokeRequest {}

impl InvokeRequest {
    pub fn new() -> InvokeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InvokeRequest {
        static mut instance: ::protobuf::lazy::Lazy<InvokeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InvokeRequest,
        };
        unsafe {
            instance.get(InvokeRequest::new)
        }
    }

    // .citacode.ActionParams param = 1;

    pub fn clear_param(&mut self) {
        self.param.clear();
    }

    pub fn has_param(&self) -> bool {
        self.param.is_some()
    }

    // Param is passed by value, moved
    pub fn set_param(&mut self, v: ActionParams) {
        self.param = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_param(&mut self) -> &mut ActionParams {
        if self.param.is_none() {
            self.param.set_default();
        }
        self.param.as_mut().unwrap()
    }

    // Take field
    pub fn take_param(&mut self) -> ActionParams {
        self.param.take().unwrap_or_else(|| ActionParams::new())
    }

    pub fn get_param(&self) -> &ActionParams {
        self.param.as_ref().unwrap_or_else(|| ActionParams::default_instance())
    }

    fn get_param_for_reflect(&self) -> &::protobuf::SingularPtrField<ActionParams> {
        &self.param
    }

    fn mut_param_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ActionParams> {
        &mut self.param
    }

    // .citacode.EnvInfo env_info = 2;

    pub fn clear_env_info(&mut self) {
        self.env_info.clear();
    }

    pub fn has_env_info(&self) -> bool {
        self.env_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_env_info(&mut self, v: EnvInfo) {
        self.env_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_env_info(&mut self) -> &mut EnvInfo {
        if self.env_info.is_none() {
            self.env_info.set_default();
        }
        self.env_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_env_info(&mut self) -> EnvInfo {
        self.env_info.take().unwrap_or_else(|| EnvInfo::new())
    }

    pub fn get_env_info(&self) -> &EnvInfo {
        self.env_info.as_ref().unwrap_or_else(|| EnvInfo::default_instance())
    }

    fn get_env_info_for_reflect(&self) -> &::protobuf::SingularPtrField<EnvInfo> {
        &self.env_info
    }

    fn mut_env_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<EnvInfo> {
        &mut self.env_info
    }
}

impl ::protobuf::Message for InvokeRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.param {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.env_info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.param)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.env_info)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(ref v) = self.param.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.env_info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.param.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.env_info.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InvokeRequest {
    fn new() -> InvokeRequest {
        InvokeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<InvokeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ActionParams>>(
                    "param",
                    InvokeRequest::get_param_for_reflect,
                    InvokeRequest::mut_param_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<EnvInfo>>(
                    "env_info",
                    InvokeRequest::get_env_info_for_reflect,
                    InvokeRequest::mut_env_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InvokeRequest>(
                    "InvokeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InvokeRequest {
    fn clear(&mut self) {
        self.clear_param();
        self.clear_env_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InvokeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InvokeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Log {
    // message fields
    pub topic: ::std::vec::Vec<u8>,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Log {}

impl Log {
    pub fn new() -> Log {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Log {
        static mut instance: ::protobuf::lazy::Lazy<Log> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Log,
        };
        unsafe {
            instance.get(Log::new)
        }
    }

    // bytes topic = 1;

    pub fn clear_topic(&mut self) {
        self.topic.clear();
    }

    // Param is passed by value, moved
    pub fn set_topic(&mut self, v: ::std::vec::Vec<u8>) {
        self.topic = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_topic(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.topic
    }

    // Take field
    pub fn take_topic(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.topic, ::std::vec::Vec::new())
    }

    pub fn get_topic(&self) -> &[u8] {
        &self.topic
    }

    fn get_topic_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.topic
    }

    fn mut_topic_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.topic
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for Log {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.topic)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.topic.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.topic);
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.topic.is_empty() {
            os.write_bytes(1, &self.topic)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Log {
    fn new() -> Log {
        Log::new()
    }

    fn descriptor_static(_: ::std::option::Option<Log>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "topic",
                    Log::get_topic_for_reflect,
                    Log::mut_topic_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    Log::get_data_for_reflect,
                    Log::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Log>(
                    "Log",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Log {
    fn clear(&mut self) {
        self.clear_topic();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Log {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Log {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KV {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KV {}

impl KV {
    pub fn new() -> KV {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KV {
        static mut instance: ::protobuf::lazy::Lazy<KV> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KV,
        };
        unsafe {
            instance.get(KV::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for KV {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KV {
    fn new() -> KV {
        KV::new()
    }

    fn descriptor_static(_: ::std::option::Option<KV>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    KV::get_key_for_reflect,
                    KV::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    KV::get_value_for_reflect,
                    KV::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KV>(
                    "KV",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KV {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KV {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KV {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct InvokeResponse {
    // message fields
    pub gas_left: ::std::string::String,
    pub message: ::std::string::String,
    pub logs: ::protobuf::RepeatedField<Log>,
    pub storages: ::protobuf::RepeatedField<KV>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for InvokeResponse {}

impl InvokeResponse {
    pub fn new() -> InvokeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static InvokeResponse {
        static mut instance: ::protobuf::lazy::Lazy<InvokeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const InvokeResponse,
        };
        unsafe {
            instance.get(InvokeResponse::new)
        }
    }

    // string gas_left = 1;

    pub fn clear_gas_left(&mut self) {
        self.gas_left.clear();
    }

    // Param is passed by value, moved
    pub fn set_gas_left(&mut self, v: ::std::string::String) {
        self.gas_left = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_gas_left(&mut self) -> &mut ::std::string::String {
        &mut self.gas_left
    }

    // Take field
    pub fn take_gas_left(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.gas_left, ::std::string::String::new())
    }

    pub fn get_gas_left(&self) -> &str {
        &self.gas_left
    }

    fn get_gas_left_for_reflect(&self) -> &::std::string::String {
        &self.gas_left
    }

    fn mut_gas_left_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.gas_left
    }

    // string message = 2;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        &self.message
    }

    fn get_message_for_reflect(&self) -> &::std::string::String {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.message
    }

    // repeated .citacode.Log logs = 3;

    pub fn clear_logs(&mut self) {
        self.logs.clear();
    }

    // Param is passed by value, moved
    pub fn set_logs(&mut self, v: ::protobuf::RepeatedField<Log>) {
        self.logs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_logs(&mut self) -> &mut ::protobuf::RepeatedField<Log> {
        &mut self.logs
    }

    // Take field
    pub fn take_logs(&mut self) -> ::protobuf::RepeatedField<Log> {
        ::std::mem::replace(&mut self.logs, ::protobuf::RepeatedField::new())
    }

    pub fn get_logs(&self) -> &[Log] {
        &self.logs
    }

    fn get_logs_for_reflect(&self) -> &::protobuf::RepeatedField<Log> {
        &self.logs
    }

    fn mut_logs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Log> {
        &mut self.logs
    }

    // repeated .citacode.KV storages = 4;

    pub fn clear_storages(&mut self) {
        self.storages.clear();
    }

    // Param is passed by value, moved
    pub fn set_storages(&mut self, v: ::protobuf::RepeatedField<KV>) {
        self.storages = v;
    }

    // Mutable pointer to the field.
    pub fn mut_storages(&mut self) -> &mut ::protobuf::RepeatedField<KV> {
        &mut self.storages
    }

    // Take field
    pub fn take_storages(&mut self) -> ::protobuf::RepeatedField<KV> {
        ::std::mem::replace(&mut self.storages, ::protobuf::RepeatedField::new())
    }

    pub fn get_storages(&self) -> &[KV] {
        &self.storages
    }

    fn get_storages_for_reflect(&self) -> &::protobuf::RepeatedField<KV> {
        &self.storages
    }

    fn mut_storages_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KV> {
        &mut self.storages
    }
}

impl ::protobuf::Message for InvokeResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.logs {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.storages {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.gas_left)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.message)?;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.logs)?;
                },
                4 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.storages)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.gas_left.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.gas_left);
        }
        if !self.message.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.message);
        }
        for value in &self.logs {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.storages {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.gas_left.is_empty() {
            os.write_string(1, &self.gas_left)?;
        }
        if !self.message.is_empty() {
            os.write_string(2, &self.message)?;
        }
        for v in &self.logs {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.storages {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for InvokeResponse {
    fn new() -> InvokeResponse {
        InvokeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<InvokeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "gas_left",
                    InvokeResponse::get_gas_left_for_reflect,
                    InvokeResponse::mut_gas_left_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "message",
                    InvokeResponse::get_message_for_reflect,
                    InvokeResponse::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Log>>(
                    "logs",
                    InvokeResponse::get_logs_for_reflect,
                    InvokeResponse::mut_logs_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KV>>(
                    "storages",
                    InvokeResponse::get_storages_for_reflect,
                    InvokeResponse::mut_storages_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<InvokeResponse>(
                    "InvokeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for InvokeResponse {
    fn clear(&mut self) {
        self.clear_gas_left();
        self.clear_message();
        self.clear_logs();
        self.clear_storages();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for InvokeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for InvokeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0ecitacode.proto\x12\x08citacode\"o\n\x0cActionParams\x12!\n\x0ccode\
    _address\x18\x01\x20\x01(\tR\x0bcodeAddress\x12\x16\n\x06sender\x18\x02\
    \x20\x01(\tR\x06sender\x12\x10\n\x03gas\x18\x03\x20\x01(\tR\x03gas\x12\
    \x12\n\x04data\x18\x04\x20\x01(\x0cR\x04data\"W\n\x07EnvInfo\x12\x16\n\
    \x06number\x18\x01\x20\x01(\tR\x06number\x12\x16\n\x06author\x18\x02\x20\
    \x01(\tR\x06author\x12\x1c\n\ttimestamp\x18\x03\x20\x01(\tR\ttimestamp\"\
    k\n\rInvokeRequest\x12,\n\x05param\x18\x01\x20\x01(\x0b2\x16.citacode.Ac\
    tionParamsR\x05param\x12,\n\x08env_info\x18\x02\x20\x01(\x0b2\x11.citaco\
    de.EnvInfoR\x07envInfo\"/\n\x03Log\x12\x14\n\x05topic\x18\x01\x20\x01(\
    \x0cR\x05topic\x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04data\",\n\x02K\
    V\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\x12\x14\n\x05value\x18\
    \x02\x20\x01(\x0cR\x05value\"\x92\x01\n\x0eInvokeResponse\x12\x19\n\x08g\
    as_left\x18\x01\x20\x01(\tR\x07gasLeft\x12\x18\n\x07message\x18\x02\x20\
    \x01(\tR\x07message\x12!\n\x04logs\x18\x03\x20\x03(\x0b2\r.citacode.LogR\
    \x04logs\x12(\n\x08storages\x18\x04\x20\x03(\x0b2\x0c.citacode.KVR\x08st\
    orages2\x8d\x01\n\x0fCitacodeService\x12;\n\x04Init\x12\x17.citacode.Inv\
    okeRequest\x1a\x18.citacode.InvokeResponse\"\0\x12=\n\x06Invoke\x12\x17.\
    citacode.InvokeRequest\x1a\x18.citacode.InvokeResponse\"\0J\xd6\x0b\n\
    \x06\x12\x04\0\0'\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\
    \x12\x03\x01\x08\x10\n\n\n\x02\x04\0\x12\x04\x02\0\x07\x01\n\n\n\x03\x04\
    \0\x01\x12\x03\x02\x08\x14\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x04\x1c\n\
    \r\n\x05\x04\0\x02\0\x04\x12\x04\x03\x04\x02\x16\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x03\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x03\x0b\x17\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x1a\x1b\n\x0b\n\x04\x04\0\x02\x01\
    \x12\x03\x04\x04\x16\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x04\x04\x03\x1c\
    \n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x04\x0b\x11\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\
    \x14\x15\n\x0b\n\x04\x04\0\x02\x02\x12\x03\x05\x04\x13\n\r\n\x05\x04\0\
    \x02\x02\x04\x12\x04\x05\x04\x04\x16\n\x0c\n\x05\x04\0\x02\x02\x05\x12\
    \x03\x05\x04\n\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x05\x0b\x0e\n\x0c\n\
    \x05\x04\0\x02\x02\x03\x12\x03\x05\x11\x12\n\x0b\n\x04\x04\0\x02\x03\x12\
    \x03\x06\x04\x13\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x06\x04\x05\x13\n\
    \x0c\n\x05\x04\0\x02\x03\x05\x12\x03\x06\x04\t\n\x0c\n\x05\x04\0\x02\x03\
    \x01\x12\x03\x06\n\x0e\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x06\x11\x12\
    \n\n\n\x02\x04\x01\x12\x04\x08\0\x0c\x01\n\n\n\x03\x04\x01\x01\x12\x03\
    \x08\x08\x0f\n\x0b\n\x04\x04\x01\x02\0\x12\x03\t\x04\x16\n\r\n\x05\x04\
    \x01\x02\0\x04\x12\x04\t\x04\x08\x11\n\x0c\n\x05\x04\x01\x02\0\x05\x12\
    \x03\t\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\t\x0b\x11\n\x0c\n\x05\
    \x04\x01\x02\0\x03\x12\x03\t\x14\x15\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\
    \n\x04\x16\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\n\x04\t\x16\n\x0c\n\x05\
    \x04\x01\x02\x01\x05\x12\x03\n\x04\n\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\
    \x03\n\x0b\x11\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\n\x14\x15\n\x0b\n\
    \x04\x04\x01\x02\x02\x12\x03\x0b\x04\x19\n\r\n\x05\x04\x01\x02\x02\x04\
    \x12\x04\x0b\x04\n\x16\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03\x0b\x04\n\
    \n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\x0b\x0b\x14\n\x0c\n\x05\x04\x01\
    \x02\x02\x03\x12\x03\x0b\x17\x18\n\n\n\x02\x04\x02\x12\x04\x0e\0\x11\x01\
    \n\n\n\x03\x04\x02\x01\x12\x03\x0e\x08\x15\n\x0b\n\x04\x04\x02\x02\0\x12\
    \x03\x0f\x04\x1b\n\r\n\x05\x04\x02\x02\0\x04\x12\x04\x0f\x04\x0e\x17\n\
    \x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x0f\x04\x10\n\x0c\n\x05\x04\x02\x02\
    \0\x01\x12\x03\x0f\x11\x16\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x0f\x19\
    \x1a\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x10\x04\x19\n\r\n\x05\x04\x02\
    \x02\x01\x04\x12\x04\x10\x04\x0f\x1b\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\
    \x03\x10\x04\x0b\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03\x10\x0c\x14\n\
    \x0c\n\x05\x04\x02\x02\x01\x03\x12\x03\x10\x17\x18\n\n\n\x02\x04\x03\x12\
    \x04\x13\0\x16\x01\n\n\n\x03\x04\x03\x01\x12\x03\x13\x08\x0b\n\x0b\n\x04\
    \x04\x03\x02\0\x12\x03\x14\x04\x14\n\r\n\x05\x04\x03\x02\0\x04\x12\x04\
    \x14\x04\x13\r\n\x0c\n\x05\x04\x03\x02\0\x05\x12\x03\x14\x04\t\n\x0c\n\
    \x05\x04\x03\x02\0\x01\x12\x03\x14\n\x0f\n\x0c\n\x05\x04\x03\x02\0\x03\
    \x12\x03\x14\x12\x13\n\x0b\n\x04\x04\x03\x02\x01\x12\x03\x15\x04\x13\n\r\
    \n\x05\x04\x03\x02\x01\x04\x12\x04\x15\x04\x14\x14\n\x0c\n\x05\x04\x03\
    \x02\x01\x05\x12\x03\x15\x04\t\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x03\
    \x15\n\x0e\n\x0c\n\x05\x04\x03\x02\x01\x03\x12\x03\x15\x11\x12\n\n\n\x02\
    \x04\x04\x12\x04\x18\0\x1b\x01\n\n\n\x03\x04\x04\x01\x12\x03\x18\x08\n\n\
    \x0b\n\x04\x04\x04\x02\0\x12\x03\x19\x04\x12\n\r\n\x05\x04\x04\x02\0\x04\
    \x12\x04\x19\x04\x18\x0c\n\x0c\n\x05\x04\x04\x02\0\x05\x12\x03\x19\x04\t\
    \n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03\x19\n\r\n\x0c\n\x05\x04\x04\x02\0\
    \x03\x12\x03\x19\x10\x11\n\x0b\n\x04\x04\x04\x02\x01\x12\x03\x1a\x04\x14\
    \n\r\n\x05\x04\x04\x02\x01\x04\x12\x04\x1a\x04\x19\x12\n\x0c\n\x05\x04\
    \x04\x02\x01\x05\x12\x03\x1a\x04\t\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\
    \x03\x1a\n\x0f\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x03\x1a\x12\x13\n\n\n\
    \x02\x04\x05\x12\x04\x1d\0\"\x01\n\n\n\x03\x04\x05\x01\x12\x03\x1d\x08\
    \x16\n\x0b\n\x04\x04\x05\x02\0\x12\x03\x1e\x04\x18\n\r\n\x05\x04\x05\x02\
    \0\x04\x12\x04\x1e\x04\x1d\x18\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03\x1e\
    \x04\n\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03\x1e\x0b\x13\n\x0c\n\x05\x04\
    \x05\x02\0\x03\x12\x03\x1e\x16\x17\n\x0b\n\x04\x04\x05\x02\x01\x12\x03\
    \x1f\x04\x17\n\r\n\x05\x04\x05\x02\x01\x04\x12\x04\x1f\x04\x1e\x18\n\x0c\
    \n\x05\x04\x05\x02\x01\x05\x12\x03\x1f\x04\n\n\x0c\n\x05\x04\x05\x02\x01\
    \x01\x12\x03\x1f\x0b\x12\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03\x1f\x15\
    \x16\n\x0b\n\x04\x04\x05\x02\x02\x12\x03\x20\x04\x1a\n\x0c\n\x05\x04\x05\
    \x02\x02\x04\x12\x03\x20\x04\x0c\n\x0c\n\x05\x04\x05\x02\x02\x06\x12\x03\
    \x20\r\x10\n\x0c\n\x05\x04\x05\x02\x02\x01\x12\x03\x20\x11\x15\n\x0c\n\
    \x05\x04\x05\x02\x02\x03\x12\x03\x20\x18\x19\n\x0b\n\x04\x04\x05\x02\x03\
    \x12\x03!\x04\x1d\n\x0c\n\x05\x04\x05\x02\x03\x04\x12\x03!\x04\x0c\n\x0c\
    \n\x05\x04\x05\x02\x03\x06\x12\x03!\r\x0f\n\x0c\n\x05\x04\x05\x02\x03\
    \x01\x12\x03!\x10\x18\n\x0c\n\x05\x04\x05\x02\x03\x03\x12\x03!\x1b\x1c\n\
    \n\n\x02\x06\0\x12\x04$\0'\x01\n\n\n\x03\x06\0\x01\x12\x03$\x08\x17\n\
    \x0b\n\x04\x06\0\x02\0\x12\x03%\x046\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\
    %\x08\x0c\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03%\r\x1a\n\x0c\n\x05\x06\0\
    \x02\0\x03\x12\x03%%3\n\x0b\n\x04\x06\0\x02\x01\x12\x03&\x048\n\x0c\n\
    \x05\x06\0\x02\x01\x01\x12\x03&\x08\x0e\n\x0c\n\x05\x06\0\x02\x01\x02\
    \x12\x03&\x0f\x1c\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03&'5b\x06proto3\
";

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
