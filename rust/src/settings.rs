// Copyright 2021 Vector 35 Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub use binaryninjacore_sys::BNSettingsScope as SettingsScope;
use binaryninjacore_sys::*;

use crate::binaryview::BinaryView;
use crate::rc::*;
use crate::string::{BnStrCompatible, BnString};

use std::ptr;

#[derive(PartialEq, Eq, Hash)]
pub struct Settings {
    pub(crate) handle: *mut BNSettings,
}

unsafe impl Send for Settings {}
unsafe impl Sync for Settings {}

impl Settings {
    pub(crate) unsafe fn from_raw(handle: *mut BNSettings) -> Ref<Self> {
        debug_assert!(!handle.is_null());

        Ref::new(Self { handle })
    }

    pub fn new<S: BnStrCompatible>(instance_id: S) -> Ref<Self> {
        let instance_id = instance_id.as_bytes_with_nul();
        unsafe {
            let handle = BNCreateSettings(instance_id.as_ref().as_ptr() as *mut _);

            debug_assert!(!handle.is_null());

            Ref::new(Self { handle })
        }
    }

    pub fn set_resource_id<S: BnStrCompatible>(&mut self, resource_id: S) {
        let resource_id = resource_id.as_bytes_with_nul();
        unsafe { BNSettingsSetResourceId(self.handle, resource_id.as_ref().as_ptr() as *mut _) };
    }

    pub fn serialize_schema(&self) -> BnString {
        unsafe { BnString::from_raw(BNSettingsSerializeSchema(self.handle)) }
    }

    pub fn deserialize_schema<S: BnStrCompatible>(&self, schema: S) -> bool {
        let schema = schema.as_bytes_with_nul();
        unsafe {
            BNSettingsDeserializeSchema(
                self.handle,
                schema.as_ref().as_ptr() as *mut _,
                BNSettingsScope::SettingsAutoScope,
                true,
            )
        }
    }

    pub fn contains<S: BnStrCompatible>(&self, key: S) -> bool {
        let key = key.as_bytes_with_nul();

        unsafe { BNSettingsContains(self.handle, key.as_ref().as_ptr() as *mut _) }
    }

    pub fn set_json<S1: BnStrCompatible, S2: BnStrCompatible>(
        &mut self,
        key: S1,
        value: S2,
        view: Option<&BinaryView>,
        scope: Option<SettingsScope>,
    ) -> bool {
        let key = key.as_bytes_with_nul();
        let value = value.as_bytes_with_nul();

        let view_handle = match view {
            Some(view) => view.handle,
            None => ptr::null_mut() as *mut _,
        };

        let scope = match scope {
            Some(scope) => scope,
            None => SettingsScope::SettingsAutoScope,
        };

        unsafe {
            BNSettingsSetJson(
                self.handle,
                view_handle,
                scope,
                key.as_ref().as_ptr() as *mut _,
                value.as_ref().as_ptr() as *mut _,
            )
        }
    }
}

impl AsRef<Settings> for Settings {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl ToOwned for Settings {
    type Owned = Ref<Self>;

    fn to_owned(&self) -> Self::Owned {
        unsafe { RefCountable::inc_ref(self) }
    }
}

unsafe impl RefCountable for Settings {
    unsafe fn inc_ref(handle: &Self) -> Ref<Self> {
        Ref::new(Self {
            handle: BNNewSettingsReference(handle.handle),
        })
    }

    unsafe fn dec_ref(handle: &Self) {
        BNFreeSettings(handle.handle);
    }
}
