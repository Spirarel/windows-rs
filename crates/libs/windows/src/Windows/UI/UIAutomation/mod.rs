#[cfg(feature = "UI_UIAutomation_Core")]
pub mod Core;
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationConnection(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationConnection {
    type Vtable = IAutomationConnection_Vtbl;
}
impl ::core::clone::Clone for IAutomationConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAutomationConnection {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xaad262ed_0ef4_5d43_97be_a834e27b65b9);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationConnection_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsRemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationConnectionBoundObject(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationConnectionBoundObject {
    type Vtable = IAutomationConnectionBoundObject_Vtbl;
}
impl ::core::clone::Clone for IAutomationConnectionBoundObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAutomationConnectionBoundObject {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x5e8558fb_ca52_5b65_9830_dd2905816093);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationConnectionBoundObject_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub Connection: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut *mut ::core::ffi::c_void) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationElement(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationElement {
    type Vtable = IAutomationElement_Vtbl;
}
impl ::core::clone::Clone for IAutomationElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAutomationElement {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0xa1898370_2c07_56fd_993f_61a72a08058c);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationElement_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
    pub IsRemoteSystem: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut bool) -> ::windows_core::HRESULT,
    pub AppUserModelId: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
    pub ExecutableFileName: unsafe extern "system" fn(this: *mut ::core::ffi::c_void, result__: *mut ::std::mem::MaybeUninit<::windows_core::HSTRING>) -> ::windows_core::HRESULT,
}
#[doc(hidden)]
#[repr(transparent)]
pub struct IAutomationTextRange(::windows_core::IUnknown);
unsafe impl ::windows_core::Interface for IAutomationTextRange {
    type Vtable = IAutomationTextRange_Vtbl;
}
impl ::core::clone::Clone for IAutomationTextRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::ComInterface for IAutomationTextRange {
    const IID: ::windows_core::GUID = ::windows_core::GUID::from_u128(0x7e101b65_40d3_5994_85a9_0a0cb9a4ec98);
}
#[repr(C)]
#[doc(hidden)]
pub struct IAutomationTextRange_Vtbl {
    pub base__: ::windows_core::IInspectable_Vtbl,
}
#[doc = "*Required features: `\"UI_UIAutomation\"`*"]
#[repr(transparent)]
pub struct AutomationConnection(::windows_core::IUnknown);
impl AutomationConnection {
    pub fn IsRemoteSystem(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRemoteSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppUserModelId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExecutableFileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExecutableFileName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AutomationConnection {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationConnection {}
impl ::core::fmt::Debug for AutomationConnection {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationConnection").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationConnection {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationConnection;{aad262ed-0ef4-5d43-97be-a834e27b65b9})");
}
impl ::core::clone::Clone for AutomationConnection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AutomationConnection {
    type Vtable = IAutomationConnection_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AutomationConnection {
    const IID: ::windows_core::GUID = <IAutomationConnection as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AutomationConnection {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationConnection";
}
::windows_core::imp::interface_hierarchy!(AutomationConnection, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AutomationConnection {}
unsafe impl ::core::marker::Sync for AutomationConnection {}
#[doc = "*Required features: `\"UI_UIAutomation\"`*"]
#[repr(transparent)]
pub struct AutomationConnectionBoundObject(::windows_core::IUnknown);
impl AutomationConnectionBoundObject {
    pub fn Connection(&self) -> ::windows_core::Result<AutomationConnection> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).Connection)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AutomationConnectionBoundObject {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationConnectionBoundObject {}
impl ::core::fmt::Debug for AutomationConnectionBoundObject {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationConnectionBoundObject").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationConnectionBoundObject {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationConnectionBoundObject;{5e8558fb-ca52-5b65-9830-dd2905816093})");
}
impl ::core::clone::Clone for AutomationConnectionBoundObject {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AutomationConnectionBoundObject {
    type Vtable = IAutomationConnectionBoundObject_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AutomationConnectionBoundObject {
    const IID: ::windows_core::GUID = <IAutomationConnectionBoundObject as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AutomationConnectionBoundObject {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationConnectionBoundObject";
}
::windows_core::imp::interface_hierarchy!(AutomationConnectionBoundObject, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AutomationConnectionBoundObject {}
unsafe impl ::core::marker::Sync for AutomationConnectionBoundObject {}
#[doc = "*Required features: `\"UI_UIAutomation\"`*"]
#[repr(transparent)]
pub struct AutomationElement(::windows_core::IUnknown);
impl AutomationElement {
    pub fn IsRemoteSystem(&self) -> ::windows_core::Result<bool> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).IsRemoteSystem)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn AppUserModelId(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).AppUserModelId)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
    pub fn ExecutableFileName(&self) -> ::windows_core::Result<::windows_core::HSTRING> {
        let this = self;
        unsafe {
            let mut result__ = ::std::mem::zeroed();
            (::windows_core::Interface::vtable(this).ExecutableFileName)(::windows_core::Interface::as_raw(this), &mut result__).from_abi(result__)
        }
    }
}
impl ::core::cmp::PartialEq for AutomationElement {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationElement {}
impl ::core::fmt::Debug for AutomationElement {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationElement").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationElement {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationElement;{a1898370-2c07-56fd-993f-61a72a08058c})");
}
impl ::core::clone::Clone for AutomationElement {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AutomationElement {
    type Vtable = IAutomationElement_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AutomationElement {
    const IID: ::windows_core::GUID = <IAutomationElement as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AutomationElement {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationElement";
}
::windows_core::imp::interface_hierarchy!(AutomationElement, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AutomationElement {}
unsafe impl ::core::marker::Sync for AutomationElement {}
#[doc = "*Required features: `\"UI_UIAutomation\"`*"]
#[repr(transparent)]
pub struct AutomationTextRange(::windows_core::IUnknown);
impl AutomationTextRange {}
impl ::core::cmp::PartialEq for AutomationTextRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl ::core::cmp::Eq for AutomationTextRange {}
impl ::core::fmt::Debug for AutomationTextRange {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_tuple("AutomationTextRange").field(&self.0).finish()
    }
}
impl ::windows_core::RuntimeType for AutomationTextRange {
    const SIGNATURE: ::windows_core::imp::ConstBuffer = ::windows_core::imp::ConstBuffer::from_slice(b"rc(Windows.UI.UIAutomation.AutomationTextRange;{7e101b65-40d3-5994-85a9-0a0cb9a4ec98})");
}
impl ::core::clone::Clone for AutomationTextRange {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
unsafe impl ::windows_core::Interface for AutomationTextRange {
    type Vtable = IAutomationTextRange_Vtbl;
}
unsafe impl ::windows_core::ComInterface for AutomationTextRange {
    const IID: ::windows_core::GUID = <IAutomationTextRange as ::windows_core::ComInterface>::IID;
}
impl ::windows_core::RuntimeName for AutomationTextRange {
    const NAME: &'static str = "Windows.UI.UIAutomation.AutomationTextRange";
}
::windows_core::imp::interface_hierarchy!(AutomationTextRange, ::windows_core::IUnknown, ::windows_core::IInspectable);
unsafe impl ::core::marker::Send for AutomationTextRange {}
unsafe impl ::core::marker::Sync for AutomationTextRange {}
