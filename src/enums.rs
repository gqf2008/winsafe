#![allow(non_snake_case)]

use std::ffi::c_void;

use crate::co;
use crate::handles::{HBITMAP, HMENU};
use crate::structs::ATOM;
use crate::WString;

/// Variant parameter used in
/// [window class](https://docs.microsoft.com/en-us/windows/win32/winmsg/window-classes)
/// functions:
///
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `lpClassName`;
/// * [`UnregisterClass`](crate::UnregisterClass) `lpClassName`.
pub enum AtomStr {
	Atom(ATOM),
	Str(WString),
}

impl AtomStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			AtomStr::Str(u16) => unsafe { u16.as_ptr() },
			AtomStr::Atom(atom) => atom.as_ptr(),
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter used in [menu](crate::HMENU) methods:
///
/// * [`AppendMenu`](crate::HMENU::AppendMenu) `lpNewItem`;
/// * [`InsertMenu`](crate::HMENU::InsertMenu) `lpNewItem`.
pub enum BitmapPtrStr {
	Bitmap(HBITMAP),
	Str(WString),
	Param(*const c_void),
}

impl BitmapPtrStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			BitmapPtrStr::Bitmap(hbmp) => unsafe { hbmp.as_ptr() as *const u16 },
			BitmapPtrStr::Str(u16) => unsafe { u16.as_ptr() },
			BitmapPtrStr::Param(lp) => *lp as *const u16,
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter for:
///
/// * [`LoadCursor`](crate::HINSTANCE::LoadCursor) `lpCursorName`.
pub enum IdIdcStr {
	Id(i32),
	Idc(co::IDC),
	Str(WString),
}

impl IdIdcStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			IdIdcStr::Id(id) => *id as *const u16,
			IdIdcStr::Idc(idc) => usize::from(*idc) as *const u16,
			IdIdcStr::Str(u16) => unsafe { u16.as_ptr() },
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter for:
///
/// * [`LoadIcon`](crate::HINSTANCE::LoadIcon) `lpIconName`.
pub enum IdIdiStr {
	Id(i32),
	Idi(co::IDI),
	Str(WString),
}

impl IdIdiStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			IdIdiStr::Id(id) => *id as *const u16,
			IdIdiStr::Idi(idi) => usize::from(*idi) as *const u16,
			IdIdiStr::Str(u16) => unsafe { u16.as_ptr() },
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter used in [menu](crate::HMENU) methods:
///
/// * [`AppendMenu`](crate::HMENU::AppendMenu) `uIDNewItem`;
/// * [`CreateWindowEx`](crate::HWND::CreateWindowEx) `hMenu`;
/// * [`InsertMenu`](crate::HMENU::InsertMenu) `uIDNewItem`.
pub enum IdMenu {
	Id(i32),
	Menu(HMENU),
	None,
}

impl From<IdMenu> for usize {
	fn from(v: IdMenu) -> usize {
		match v {
			IdMenu::Id(id) => id as usize,
			IdMenu::Menu(hMenu) => (unsafe { hMenu.as_ptr() }) as usize,
			IdMenu::None => 0,
		}
	}
}

impl IdMenu {
	/// Converts the internal value to a `*mut c_void`.
	pub fn as_ptr(&self) -> *mut c_void {
		match self {
			IdMenu::Id(id) => *id as *mut c_void,
			IdMenu::Menu(hMenu) => unsafe { hMenu.as_ptr() },
			IdMenu::None => std::ptr::null_mut(),
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter used in [menu](crate::HMENU) methods:
///
/// * [`CheckMenuItem`](crate::HMENU::CheckMenuItem) `uIDCheckItem`;
/// * [`DeleteMenu`](crate::HMENU::DeleteMenu) `uPosition`;
/// * [`EnableMenuItem`](crate::HMENU::EnableMenuItem) `uIDEnableItem`;
/// * [`HiliteMenuItem`](crate::HWND::HiliteMenuItem) `uIDHiliteItem`;
/// * [`InsertMenuItem`](crate::HMENU::InsertMenuItem) `item`;
/// * [`RemoveMenu`](crate::HMENU::RemoveMenu) `uPosition`;
/// * [`SetMenuItemInfo`](crate::HMENU::SetMenuItemInfo) `item`.
pub enum IdPos {
	Id(i32),
	Pos(u32),
}

impl From<IdPos> for u32 {
	fn from(v: IdPos) -> u32 {
		match v {
			IdPos::Id(id) => id as u32,
			IdPos::Pos(pos) => pos,
		}
	}
}

//------------------------------------------------------------------------------

/// Variant parameter for:
///
/// * [`LoadAccelerators`](crate::HINSTANCE::LoadAccelerators) `lpTableName`.
/// * [`WNDCLASSEX`](crate::WNDCLASSEX) `lpszMenuName`;
pub enum IdStr {
	Id(i32),
	Str(WString),
}

impl IdStr {
	/// Converts the internal value to a `*const u16`.
	pub fn as_ptr(&self) -> *const u16 {
		match self {
			IdStr::Id(id) => *id as *const u16,
			IdStr::Str(u16) => unsafe { u16.as_ptr() },
		}
	}
}

//------------------------------------------------------------------------------

/// Variant value returned by [`RegQueryValueEx`](crate::HKEY::RegQueryValueEx).
pub enum RegistryValue {
	Binary(Vec<u8>),
	Dword(u32),
	Qword(u64),
	Sz(WString),
	None,
}

impl RegistryValue {
	/// Converts the internal value to a `*const c_void`.
	pub fn as_ptr(&self) -> *const c_void {
		match self {
			RegistryValue::Binary(b) => b.as_ptr() as *const c_void,
			RegistryValue::Dword(n) => *n as *const c_void,
			RegistryValue::Qword(n) => *n as *const c_void,
			RegistryValue::Sz(u16) => unsafe { u16.as_ptr() as *const c_void },
			RegistryValue::None => std::ptr::null(),
		}
	}

	/// Returns the correspondent [`co::REG`](crate::co::REG) constant.
	pub fn reg_type(&self) -> co::REG {
		match self {
			RegistryValue::Binary(_) => co::REG::BINARY,
			RegistryValue::Dword(_) => co::REG::DWORD,
			RegistryValue::Qword(_) => co::REG::QWORD,
			RegistryValue::Sz(_) => co::REG::SZ,
			RegistryValue::None => co::REG::NONE,
		}
	}

	/// Returns the length of the stored data.
	pub fn len(&self) -> usize {
		match self {
			RegistryValue::Binary(b) => b.len(),
			RegistryValue::Dword(_) => std::mem::size_of::<u32>(),
			RegistryValue::Qword(_) => std::mem::size_of::<u64>(),
			RegistryValue::Sz(u16) => {
				(u16.len() + 1) * std::mem::size_of::<u16>() // including terminating null
			},
			RegistryValue::None => 0,
		}
	}
}