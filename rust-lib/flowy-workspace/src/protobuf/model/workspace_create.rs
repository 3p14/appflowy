// This file is generated by rust-protobuf 2.22.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `workspace_create.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_22_1;

#[derive(PartialEq,Clone,Default)]
pub struct CreateWorkspaceRequest {
    // message fields
    pub name: ::std::string::String,
    pub desc: ::std::string::String,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CreateWorkspaceRequest {
    fn default() -> &'a CreateWorkspaceRequest {
        <CreateWorkspaceRequest as ::protobuf::Message>::default_instance()
    }
}

impl CreateWorkspaceRequest {
    pub fn new() -> CreateWorkspaceRequest {
        ::std::default::Default::default()
    }

    // string name = 1;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // string desc = 2;


    pub fn get_desc(&self) -> &str {
        &self.desc
    }
    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: ::std::string::String) {
        self.desc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc(&mut self) -> &mut ::std::string::String {
        &mut self.desc
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.desc, ::std::string::String::new())
    }
}

impl ::protobuf::Message for CreateWorkspaceRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.desc)?;
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
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.name);
        }
        if !self.desc.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.desc);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.name.is_empty() {
            os.write_string(1, &self.name)?;
        }
        if !self.desc.is_empty() {
            os.write_string(2, &self.desc)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CreateWorkspaceRequest {
        CreateWorkspaceRequest::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &CreateWorkspaceRequest| { &m.name },
                |m: &mut CreateWorkspaceRequest| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "desc",
                |m: &CreateWorkspaceRequest| { &m.desc },
                |m: &mut CreateWorkspaceRequest| { &mut m.desc },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CreateWorkspaceRequest>(
                "CreateWorkspaceRequest",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CreateWorkspaceRequest {
        static instance: ::protobuf::rt::LazyV2<CreateWorkspaceRequest> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CreateWorkspaceRequest::new)
    }
}

impl ::protobuf::Clear for CreateWorkspaceRequest {
    fn clear(&mut self) {
        self.name.clear();
        self.desc.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CreateWorkspaceRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CreateWorkspaceRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Workspace {
    // message fields
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub desc: ::std::string::String,
    pub apps: ::protobuf::SingularPtrField<super::app_create::RepeatedApp>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Workspace {
    fn default() -> &'a Workspace {
        <Workspace as ::protobuf::Message>::default_instance()
    }
}

impl Workspace {
    pub fn new() -> Workspace {
        ::std::default::Default::default()
    }

    // string id = 1;


    pub fn get_id(&self) -> &str {
        &self.id
    }
    pub fn clear_id(&mut self) {
        self.id.clear();
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: ::std::string::String) {
        self.id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_id(&mut self) -> &mut ::std::string::String {
        &mut self.id
    }

    // Take field
    pub fn take_id(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.id, ::std::string::String::new())
    }

    // string name = 2;


    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        &mut self.name
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }

    // string desc = 3;


    pub fn get_desc(&self) -> &str {
        &self.desc
    }
    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }

    // Param is passed by value, moved
    pub fn set_desc(&mut self, v: ::std::string::String) {
        self.desc = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_desc(&mut self) -> &mut ::std::string::String {
        &mut self.desc
    }

    // Take field
    pub fn take_desc(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.desc, ::std::string::String::new())
    }

    // .RepeatedApp apps = 4;


    pub fn get_apps(&self) -> &super::app_create::RepeatedApp {
        self.apps.as_ref().unwrap_or_else(|| <super::app_create::RepeatedApp as ::protobuf::Message>::default_instance())
    }
    pub fn clear_apps(&mut self) {
        self.apps.clear();
    }

    pub fn has_apps(&self) -> bool {
        self.apps.is_some()
    }

    // Param is passed by value, moved
    pub fn set_apps(&mut self, v: super::app_create::RepeatedApp) {
        self.apps = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_apps(&mut self) -> &mut super::app_create::RepeatedApp {
        if self.apps.is_none() {
            self.apps.set_default();
        }
        self.apps.as_mut().unwrap()
    }

    // Take field
    pub fn take_apps(&mut self) -> super::app_create::RepeatedApp {
        self.apps.take().unwrap_or_else(|| super::app_create::RepeatedApp::new())
    }
}

impl ::protobuf::Message for Workspace {
    fn is_initialized(&self) -> bool {
        for v in &self.apps {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.id)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.name)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.desc)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.apps)?;
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
        if !self.id.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.id);
        }
        if !self.name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.name);
        }
        if !self.desc.is_empty() {
            my_size += ::protobuf::rt::string_size(3, &self.desc);
        }
        if let Some(ref v) = self.apps.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.id.is_empty() {
            os.write_string(1, &self.id)?;
        }
        if !self.name.is_empty() {
            os.write_string(2, &self.name)?;
        }
        if !self.desc.is_empty() {
            os.write_string(3, &self.desc)?;
        }
        if let Some(ref v) = self.apps.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Workspace {
        Workspace::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "id",
                |m: &Workspace| { &m.id },
                |m: &mut Workspace| { &mut m.id },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "name",
                |m: &Workspace| { &m.name },
                |m: &mut Workspace| { &mut m.name },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "desc",
                |m: &Workspace| { &m.desc },
                |m: &mut Workspace| { &mut m.desc },
            ));
            fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::app_create::RepeatedApp>>(
                "apps",
                |m: &Workspace| { &m.apps },
                |m: &mut Workspace| { &mut m.apps },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Workspace>(
                "Workspace",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Workspace {
        static instance: ::protobuf::rt::LazyV2<Workspace> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Workspace::new)
    }
}

impl ::protobuf::Clear for Workspace {
    fn clear(&mut self) {
        self.id.clear();
        self.name.clear();
        self.desc.clear();
        self.apps.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Workspace {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Workspace {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Workspaces {
    // message fields
    pub items: ::protobuf::RepeatedField<Workspace>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Workspaces {
    fn default() -> &'a Workspaces {
        <Workspaces as ::protobuf::Message>::default_instance()
    }
}

impl Workspaces {
    pub fn new() -> Workspaces {
        ::std::default::Default::default()
    }

    // repeated .Workspace items = 1;


    pub fn get_items(&self) -> &[Workspace] {
        &self.items
    }
    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<Workspace>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<Workspace> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<Workspace> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for Workspaces {
    fn is_initialized(&self) -> bool {
        for v in &self.items {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
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
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.items {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Workspaces {
        Workspaces::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Workspace>>(
                "items",
                |m: &Workspaces| { &m.items },
                |m: &mut Workspaces| { &mut m.items },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Workspaces>(
                "Workspaces",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Workspaces {
        static instance: ::protobuf::rt::LazyV2<Workspaces> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Workspaces::new)
    }
}

impl ::protobuf::Clear for Workspaces {
    fn clear(&mut self) {
        self.items.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Workspaces {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Workspaces {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16workspace_create.proto\x1a\x10app_create.proto\"@\n\x16CreateWorks\
    paceRequest\x12\x12\n\x04name\x18\x01\x20\x01(\tR\x04name\x12\x12\n\x04d\
    esc\x18\x02\x20\x01(\tR\x04desc\"e\n\tWorkspace\x12\x0e\n\x02id\x18\x01\
    \x20\x01(\tR\x02id\x12\x12\n\x04name\x18\x02\x20\x01(\tR\x04name\x12\x12\
    \n\x04desc\x18\x03\x20\x01(\tR\x04desc\x12\x20\n\x04apps\x18\x04\x20\x01\
    (\x0b2\x0c.RepeatedAppR\x04apps\".\n\nWorkspaces\x12\x20\n\x05items\x18\
    \x01\x20\x03(\x0b2\n.WorkspaceR\x05itemsJ\xf4\x03\n\x06\x12\x04\0\0\x0f\
    \x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\t\n\x02\x03\0\x12\x03\x01\0\x1a\n\
    \n\n\x02\x04\0\x12\x04\x03\0\x06\x01\n\n\n\x03\x04\0\x01\x12\x03\x03\x08\
    \x1e\n\x0b\n\x04\x04\0\x02\0\x12\x03\x04\x04\x14\n\x0c\n\x05\x04\0\x02\0\
    \x05\x12\x03\x04\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x04\x0b\x0f\n\
    \x0c\n\x05\x04\0\x02\0\x03\x12\x03\x04\x12\x13\n\x0b\n\x04\x04\0\x02\x01\
    \x12\x03\x05\x04\x14\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x05\x04\n\n\
    \x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x05\x0b\x0f\n\x0c\n\x05\x04\0\x02\
    \x01\x03\x12\x03\x05\x12\x13\n\n\n\x02\x04\x01\x12\x04\x07\0\x0c\x01\n\n\
    \n\x03\x04\x01\x01\x12\x03\x07\x08\x11\n\x0b\n\x04\x04\x01\x02\0\x12\x03\
    \x08\x04\x12\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x08\x04\n\n\x0c\n\x05\
    \x04\x01\x02\0\x01\x12\x03\x08\x0b\r\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x03\x08\x10\x11\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\t\x04\x14\n\x0c\n\
    \x05\x04\x01\x02\x01\x05\x12\x03\t\x04\n\n\x0c\n\x05\x04\x01\x02\x01\x01\
    \x12\x03\t\x0b\x0f\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\t\x12\x13\n\
    \x0b\n\x04\x04\x01\x02\x02\x12\x03\n\x04\x14\n\x0c\n\x05\x04\x01\x02\x02\
    \x05\x12\x03\n\x04\n\n\x0c\n\x05\x04\x01\x02\x02\x01\x12\x03\n\x0b\x0f\n\
    \x0c\n\x05\x04\x01\x02\x02\x03\x12\x03\n\x12\x13\n\x0b\n\x04\x04\x01\x02\
    \x03\x12\x03\x0b\x04\x19\n\x0c\n\x05\x04\x01\x02\x03\x06\x12\x03\x0b\x04\
    \x0f\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\x03\x0b\x10\x14\n\x0c\n\x05\x04\
    \x01\x02\x03\x03\x12\x03\x0b\x17\x18\n\n\n\x02\x04\x02\x12\x04\r\0\x0f\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03\r\x08\x12\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03\x0e\x04!\n\x0c\n\x05\x04\x02\x02\0\x04\x12\x03\x0e\x04\x0c\n\
    \x0c\n\x05\x04\x02\x02\0\x06\x12\x03\x0e\r\x16\n\x0c\n\x05\x04\x02\x02\0\
    \x01\x12\x03\x0e\x17\x1c\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03\x0e\x1f\
    \x20b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
