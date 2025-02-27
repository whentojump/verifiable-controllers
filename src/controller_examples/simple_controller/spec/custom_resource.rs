// Copyright 2022 VMware, Inc.
// SPDX-License-Identifier: MIT
use crate::kubernetes_api_objects::api_resource::*;
use crate::kubernetes_api_objects::common::*;
use crate::kubernetes_api_objects::dynamic::*;
use crate::kubernetes_api_objects::object_meta::*;
use crate::kubernetes_api_objects::resource::*;
use crate::pervasive_ext::string_view::*;
use vstd::prelude::*;
use vstd::string::*;

use deps_hack::SimpleCR;
use deps_hack::SimpleCRSpec;

use deps_hack::k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as K8SObjectMeta;

verus! {

// TODO: CustomResource should be defined by the controller developer
#[verifier(external_body)]
pub struct CustomResource {
    inner: SimpleCR
}

pub struct CustomResourceView {
    pub metadata: ObjectMetaView,
    pub spec: CustomResourceSpecView,
    pub status: Option<CustomResourceStatusView>,
}

impl CustomResource {
    pub spec fn view(&self) -> CustomResourceView;

    #[verifier(external_body)]
    pub fn api_resource() -> (res: ApiResource)
        ensures
            res@.kind == Kind::CustomResourceKind,
    {
        ApiResource::from_kube_api_resource(deps_hack::kube::api::ApiResource::erase::<SimpleCR>(&()))
    }

    #[verifier(external_body)]
    pub fn metadata(&self) -> (metadata: ObjectMeta)
        ensures
            metadata@ == self@.metadata,
    {
        ObjectMeta::from_kube_object_meta(self.inner.metadata.clone())
    }

    #[verifier(external_body)]
    pub fn spec(&self) -> (spec: CustomResourceSpec)
        ensures
            spec@ == self@.spec,
    {
        CustomResourceSpec {
            inner: self.inner.spec.clone()
        }
    }

    #[verifier(external_body)]
    pub fn status(&self) -> (status: Option<CustomResourceStatus>)
        ensures
            self@.status.is_Some() == status.is_Some(),
            status.is_Some() ==> status.get_Some_0()@ == self@.status.get_Some_0(),
    {
        todo!()
    }

    /// Convert a DynamicObject to a CustomResource
    // NOTE: This function assumes try_parse won't fail!
    #[verifier(external_body)]
    pub fn from_dynamic_object(obj: DynamicObject) -> (cr: CustomResource)
        ensures
            cr@ == CustomResourceView::from_dynamic_object(obj@),
    {
        CustomResource {inner: obj.into_kube_obj().try_parse::<SimpleCR>().unwrap()}
    }
}

impl CustomResourceView {}

impl ResourceView for CustomResourceView {
    open spec fn metadata(self) -> ObjectMetaView {
        self.metadata
    }

    open spec fn kind(self) -> Kind {
        Kind::CustomResourceKind
    }

    open spec fn object_ref(self) -> ObjectRef {
        ObjectRef {
            kind: self.kind(),
            name: self.metadata.name.get_Some_0(),
            namespace: self.metadata.namespace.get_Some_0(),
        }
    }

    open spec fn to_dynamic_object(self) -> DynamicObjectView {
        DynamicObjectView {
            kind: self.kind(),
            metadata: self.metadata,
            data: Value::Object(Map::empty()
                                    .insert(spec_field(), Value::Object(Map::empty().insert(spec_content_field(), Value::String(self.spec.content)))
                                    )
                                    .insert(status_field(), if self.status.is_None() {Value::Null} else {
                                        Value::Object(Map::empty().insert(status_echoed_content_field(), Value::String(self.status.get_Some_0().echoed_content)))
                                    })
                                ),
        }
    }

    open spec fn from_dynamic_object(obj: DynamicObjectView) -> CustomResourceView {
        CustomResourceView {
            metadata: obj.metadata,
            spec: CustomResourceSpecView{
                content: obj.data.get_Object_0()[spec_field()].get_Object_0()[spec_content_field()].get_String_0(),
            },
            status: if obj.data.get_Object_0()[status_field()].is_Null() {Option::None} else {Option::Some(CustomResourceStatusView{
                echoed_content: obj.data.get_Object_0()[status_field()].get_Object_0()[status_echoed_content_field()].get_String_0(),
            })},
        }
    }

    proof fn integrity_check() {}
}

#[verifier(external_body)]
pub struct CustomResourceSpec {
    inner: SimpleCRSpec
}

pub struct CustomResourceSpecView {
    pub content: StringView,
}

impl CustomResourceSpec {
    pub spec fn view(&self) -> CustomResourceSpecView;

    #[verifier(external_body)]
    pub fn content(&self) -> (content: String)
        ensures
            content@ == self@.content,
    {
        String::from_rust_string(self.inner.content.to_string())
    }
}

#[verifier(external_body)]
pub struct CustomResourceStatus {
    // TODO: add the content
}

pub struct CustomResourceStatusView {
    pub echoed_content: StringView,
}

impl CustomResourceStatus {
    pub spec fn view(&self) -> CustomResourceStatusView;
}

pub open spec fn spec_field() -> nat {0}

pub open spec fn status_field() -> nat {1}

pub open spec fn spec_content_field() -> nat {0}

pub open spec fn status_echoed_content_field() -> nat {0}

}
