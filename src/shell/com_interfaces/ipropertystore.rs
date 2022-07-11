#![allow(non_camel_case_types, non_snake_case)]

use std::marker::PhantomData;
use std::mem::ManuallyDrop;

use crate::ffi_types::{HRES, PCVOID, PVOID};
use crate::ole::decl::{ComPtr, HrResult};
use crate::ole::privs::ok_to_hrresult;
use crate::prelude::ole_IUnknown;
use crate::shell::decl::PROPERTYKEY;
use crate::vt::IUnknownVT;

/// [`IPropertyStore`](crate::IPropertyStore) virtual table.
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
#[repr(C)]
pub struct IPropertyStoreVT {
	pub IUnknownVT: IUnknownVT,
	pub GetCount: fn(ComPtr, *mut u32) -> HRES,
	pub GetAt: fn(ComPtr, u32, PVOID) -> HRES,
	pub GetValue: fn(ComPtr, PCVOID, PVOID) -> HRES,
	pub SetValue: fn(ComPtr, PCVOID, PCVOID) -> HRES,
	pub Commit: fn(ComPtr) -> HRES,
}

/// [`IPropertyStore`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nn-propsys-ipropertystore)
/// COM interface over [`IPropertyStoreVT`](crate::vt::IPropertyStoreVT).
///
/// Automatically calls
/// [`IUnknown::Release`](https://docs.microsoft.com/en-us/windows/win32/api/unknwn/nf-unknwn-iunknown-release)
/// when the object goes out of scope.
///
/// # Examples
///
/// Instantiating from an [`IShellItem`](crate::IShellItem) object:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// use winsafe::{co, IPropertyStore, IShellItem, SHCreateItemFromParsingName};
///
/// let file = SHCreateItemFromParsingName::<IShellItem>("C:\\Temp\\foo.txt", None)?;
/// let props = file.BindToHandler::<IPropertyStore>(None, &co::BHID::PropertyStore)?;
/// # Ok::<_, co::HRESULT>(())
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub struct IPropertyStore(ComPtr);

impl_iunknown!(IPropertyStore, "886d8eeb-8cf2-4446-8d02-cdba1dbdcf99");
impl shell_IPropertyStore for IPropertyStore {}

/// This trait is enabled with the `shell` feature, and provides methods for
/// [`IPropertyStore`](crate::IPropertyStore).
///
/// Prefer importing this trait through the prelude:
///
/// ```rust,no_run
/// use winsafe::prelude::*;
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "shell")))]
pub trait shell_IPropertyStore: ole_IUnknown {
	/// Returns an iterator over the [`PROPERTYKEY`](crate::PROPERTYKEY)
	/// elements by calling
	/// [`IPropertyStore::GetCount`](crate::prelude::shell_IPropertyStore::GetCount)
	/// and
	/// [`IPropertyStore::GetAt`](crate::prelude::shell_IPropertyStore::GetAt)
	/// consecutively.
	///
	/// # Examples
	///
	/// ```rust,no_run
	/// use winsafe::prelude::*;
	/// use winsafe::IPropertyStore;
	///
	/// let pstore: IPropertyStore; // initialized somewhere
	/// # let pstore = IPropertyStore::from(unsafe { winsafe::ComPtr::null() });
	///
	/// for ppk in pstore.iter()? {
	///     let ppk = ppk?;
	///     // ...
	/// }
	/// # Ok::<_, winsafe::co::HRESULT>(())
	/// ```
	#[must_use]
	fn iter<'a>(&'a self) -> HrResult<Box<dyn Iterator<Item = HrResult<PROPERTYKEY>> + 'a>> {
		Ok(Box::new(PropertyStoreIter::new(unsafe { self.ptr() })?))
	}

	/// [`IPropertyStore::Commit`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-commit)
	/// method.
	fn Commit(&self) -> HrResult<()> {
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPropertyStoreVT);
			ok_to_hrresult((vt.Commit)(self.ptr()))
		}
	}

	/// [`IPropertyStore::GetAt`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getat)
	/// method.
	#[must_use]
	fn GetAt(&self, index: u32) -> HrResult<PROPERTYKEY> {
		let mut ppk = PROPERTYKEY::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPropertyStoreVT);
			ok_to_hrresult(
				(vt.GetAt)(self.ptr(), index, &mut ppk as *const _ as _),
			)
		}.map(|_| ppk)
	}

	/// [`IPropertyStore::GetCount`](https://docs.microsoft.com/en-us/windows/win32/api/propsys/nf-propsys-ipropertystore-getcount)
	/// method.
	#[must_use]
	fn GetCount(&self) -> HrResult<u32> {
		let mut count = u32::default();
		unsafe {
			let vt = &**(self.ptr().0 as *mut *mut IPropertyStoreVT);
			ok_to_hrresult((vt.GetCount)(self.ptr(), &mut count))
		}.map(|_| count)
	}
}

//------------------------------------------------------------------------------

struct PropertyStoreIter<'a> {
	array: ManuallyDrop<IPropertyStore>,
	count: u32,
	current: u32,
	_owner: PhantomData<&'a ()>,
}

impl<'a> Iterator for PropertyStoreIter<'a> {
	type Item = HrResult<PROPERTYKEY>;

	fn next(&mut self) -> Option<Self::Item> {
		if self.current == self.count {
			return None;
		}

		match self.array.GetAt(self.current) {
			Err(e) => {
				self.current = self.count; // no further iterations will be made
				Some(Err(e))
			},
			Ok(ppk) => {
				self.current += 1;
				Some(Ok(ppk))
			},
		}
	}
}

impl<'a> PropertyStoreIter<'a> {
	fn new(com_ptr: ComPtr) -> HrResult<Self> {
		let array = ManuallyDrop::new(IPropertyStore(com_ptr));
		let count = array.GetCount()?;

		Ok(Self {
			array,
			count,
			current: 0,
			_owner: PhantomData,
		})
	}
}
