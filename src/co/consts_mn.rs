const_type! { MB, u32,
	/// [`MessageBox`](crate::HWND::MessageBox) `uType` (`u32`).

	ABORTRETRYIGNORE, 0x00000002
	CANCELTRYCONTINUE, 0x00000006
	HELP, 0x00004000
	OK, 0x00000000
	OKCANCEL, 0x00000001
	RETRYCANCEL, 0x00000005
	YESNO, 0x00000004
	YESNOCANCEL, 0x00000003

	ICONEXCLAMATION, 0x00000030
	ICONWARNING, Self::ICONEXCLAMATION.0
	ICONINFORMATION, 0x00000040
	ICONASTERISK, Self::ICONINFORMATION.0
	ICONQUESTION, 0x00000020
	ICONSTOP, Self::ICONERROR.0
	ICONERROR, 0x00000010
	ICONHAND, Self::ICONERROR.0

	DEFBUTTON1, 0x00000000
	DEFBUTTON2, 0x00000100
	DEFBUTTON3, 0x00000200
	DEFBUTTON4, 0x00000300

	APPLMODAL, 0x00000000
	SYSTEMMODAL, 0x00001000
	TASKMODAL, 0x00002000

	DEFAULT_DESKTOP_ONLY, 0x00020000
	RIGHT, 0x00080000
	RTLREADING, 0x00100000
	SETFOREGROUND, 0x00010000
	TOPMOST, 0x00040000
	SERVICE_NOTIFICATION, 0x00200000
}

const_type! { MF, u32,
	/// [`AppendMenu`](crate::HMENU::AppendMenu) and
	/// [`InsertMenu`](crate::HMENU::InsertMenu) `uFlags` (`u32`).

	INSERT, 0x00000000
	CHANGE, 0x00000080
	APPEND, 0x00000100
	DELETE, 0x00000200
	REMOVE, 0x00001000
	BYCOMMAND, 0x00000000
	BYPOSITION, 0x00000400
	SEPARATOR, 0x00000800
	ENABLED, 0x00000000
	GRAYED, 0x00000001
	DISABLED, 0x00000002
	UNCHECKED, 0x00000000
	CHECKED, 0x00000008
	USECHECKBITMAPS, 0x00000200
	STRING, 0x00000000
	BITMAP, 0x00000004
	OWNERDRAW, 0x00000100
	POPUP, 0x00000010
	MENUBARBREAK, 0x00000020
	MENUBREAK, 0x00000040
	UNHILITE, 0x00000000
	HILITE, 0x00000080
	DEFAULT, 0x00001000
	SYSMENU, 0x00002000
	HELP, 0x00004000
	RIGHTJUSTIFY, 0x00004000
	MOUSESELECT, 0x00008000
}

const_type! { MFS, u32,
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fState` (`u32`).

	GRAYED, 0x00000003
	DISABLED, MFS::GRAYED.0
	CHECKED, MF::CHECKED.0
	HILITE, MF::HILITE.0
	ENABLED, MF::ENABLED.0
	UNCHECKED, MF::UNCHECKED.0
	UNHILITE, MF::UNHILITE.0
	DEFAULT, MF::DEFAULT.0
}

const_type! { MFT, u32,
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fType` (`u32`).

	STRING, MF::STRING.0
	BITMAP, MF::BITMAP.0
	MENUBARBREAK, MF::MENUBARBREAK.0
	MENUBREAK, MF::MENUBREAK.0
	OWNERDRAW, MF::OWNERDRAW.0
	RADIOCHECK, 0x00000200
	SEPARATOR, MF::SEPARATOR.0
	RIGHTORDER, 0x00002000
	RIGHTJUSTIFY, MF::RIGHTJUSTIFY.0
}

const_type! { MIM, u32,
	/// [`MENUINFO`](crate::MENUINFO) `fMask` (`u32`).

	MAXHEIGHT, 0x00000001
	BACKGROUND, 0x00000002
	HELPID, 0x00000004
	MENUDATA, 0x00000008
	STYLE, 0x00000010
	APPLYTOSUBMENUS, 0x80000000
}

const_type! { MIIM, u32,
	/// [`MENUITEMINFO`](crate::MENUITEMINFO) `fMask` (`u32`).

	MAXHEIGHT, 0x00000001
	BACKGROUND, 0x00000002
	HELPID, 0x00000004
	MENUDATA, 0x00000008
	STYLE, 0x00000010
	APPLYTOSUBMENUS, 0x80000000
}

const_type! { MK, u16,
	/// [`WM_LBUTTONDOWN`](crate::msg::WmLButtonDown) (and similar) virtual keys
	/// (`u16`).

	LBUTTON, 0x0001
	RBUTTON, 0x0002
	SHIFT, 0x0004
	CONTROL, 0x0008
	MBUTTON, 0x0010
	XBUTTON1, 0x0020
	XBUTTON2, 0x0040
}

const_type! { MNS, u32,
	/// [`MENUINFO`](crate::MENUINFO) `dwStyle` (`u32`).

	NOCHECK, 0x80000000
	MODELESS, 0x40000000
	DRAGDROP, 0x20000000
	AUTODISMISS, 0x10000000
	NOTIFYBYPOS, 0x08000000
	CHECKORBMP, 0x04000000
}

const_type! { MSGF, u8,
	/// [`WM_ENTERIDLE`](crate::msg::WmEnterIdle) reason (`u8`).

	DIALOGBOX, 0
	MENU, 2
}

const_type! { NM, i32,
	/// [`WM_NOTIFY`](crate::msg::WmNotify)
	/// notifications (`i32`) for:
	///
	/// * [common controls](https://docs.microsoft.com/en-us/windows/win32/controls/common-control-reference#notifications);
	/// * [button](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-button-control-reference-notifications);
	/// * [ListView](https://docs.microsoft.com/en-us/windows/win32/controls/bumper-list-view-control-reference-notifications).

	OUTOFMEMORY, Self::FIRST.0 - 1
	CLICK, Self::FIRST.0 - 2
	DBLCLK, Self::FIRST.0 - 3
	RETURN, Self::FIRST.0 - 4
	RCLICK, Self::FIRST.0 - 5
	RDBLCLK, Self::FIRST.0 - 6
	SETFOCUS, Self::FIRST.0 - 7
	KILLFOCUS, Self::FIRST.0 - 8
	CUSTOMDRAW, Self::FIRST.0 - 12
	HOVER, Self::FIRST.0 - 13
	NCHITTEST, Self::FIRST.0 - 14
	KEYDOWN, Self::FIRST.0 - 15
	RELEASEDCAPTURE, Self::FIRST.0 - 16
	SETCURSOR, Self::FIRST.0 - 17
	CHAR, Self::FIRST.0 - 18
	TOOLTIPSCREATED, Self::FIRST.0 - 19
	LDOWN, Self::FIRST.0 - 20
	RDOWN, Self::FIRST.0 - 21
	THEMECHANGED, Self::FIRST.0 - 22

	BCN_HOTITEMCHANGE, Self::BCN_FIRST.0 + 0x0001
	BCN_DROPDOWN, Self::BCN_FIRST.0 + 0x0002

	LVN_ITEMCHANGING, Self::LVN_FIRST.0 - 0
	LVN_ITEMCHANGED, Self::LVN_FIRST.0 - 1
	LVN_INSERTITEM, Self::LVN_FIRST.0 - 2
	LVN_DELETEITEM, Self::LVN_FIRST.0 - 3
	LVN_DELETEALLITEMS, Self::LVN_FIRST.0 - 4
	LVN_BEGINLABELEDIT, Self::LVN_FIRST.0 - 75
	LVN_ENDLABELEDIT, Self::LVN_FIRST.0 - 76
	LVN_COLUMNCLICK, Self::LVN_FIRST.0 - 8
	LVN_BEGINDRAG, Self::LVN_FIRST.0 - 9
	LVN_BEGINRDRAG, Self::LVN_FIRST.0 - 11
	LVN_ODCACHEHINT, Self::LVN_FIRST.0 - 13
	LVN_ODFINDITEM, Self::LVN_FIRST.0 - 79
	LVN_ITEMACTIVATE, Self::LVN_FIRST.0 - 14
	LVN_ODSTATECHANGED, Self::LVN_FIRST.0 - 15
	LVN_HOTTRACK, Self::LVN_FIRST.0 - 21
	LVN_GETDISPINFO, Self::LVN_FIRST.0 - 77
	LVN_SETDISPINFO, Self::LVN_FIRST.0 - 78
	LVN_KEYDOWN, Self::LVN_FIRST.0 - 55
	LVN_MARQUEEBEGIN, Self::LVN_FIRST.0 - 56
	LVN_GETINFOTIP, Self::LVN_FIRST.0 - 58
	LVN_INCREMENTALSEARCH, Self::LVN_FIRST.0 - 63
	LVN_COLUMNDROPDOWN, Self::LVN_FIRST.0 - 64
	LVN_COLUMNOVERFLOWCLICK, Self::LVN_FIRST.0 - 66
	LVN_BEGINSCROLL, Self::LVN_FIRST.0 - 80
	LVN_ENDSCROLL, Self::LVN_FIRST.0 - 81
	LVN_LINKCLICK, Self::LVN_FIRST.0 - 84
	LVN_GETEMPTYMARKUP, Self::LVN_FIRST.0 - 87

	TVN_SELCHANGING, Self::TVN_FIRST.0 - 50
	TVN_SELCHANGED, Self::TVN_FIRST.0 - 51
	TVN_GETDISPINFO, Self::TVN_FIRST.0 - 52
	TVN_SETDISPINFO, Self::TVN_FIRST.0 - 53
	TVN_ITEMEXPANDING, Self::TVN_FIRST.0 - 54
	TVN_ITEMEXPANDED, Self::TVN_FIRST.0 - 55
	TVN_BEGINDRAG, Self::TVN_FIRST.0 - 56
	TVN_BEGINRDRAG, Self::TVN_FIRST.0 - 57
	TVN_DELETEITEM, Self::TVN_FIRST.0 - 58
	TVN_BEGINLABELEDIT, Self::TVN_FIRST.0 - 59
	TVN_ENDLABELEDIT, Self::TVN_FIRST.0 - 60
	TVN_KEYDOWN, Self::TVN_FIRST.0 - 12
	TVN_GETINFOTIP, Self::TVN_FIRST.0 - 14
	TVN_SINGLEEXPAND, Self::TVN_FIRST.0 - 15
	TVN_ITEMCHANGING, Self::TVN_FIRST.0 - 17
	TVN_ITEMCHANGED, Self::TVN_FIRST.0 - 19
	TVN_ASYNCDRAW, Self::TVN_FIRST.0 - 20
}
impl NM {
	const FIRST: Self = Self(0);
	const BCN_FIRST: Self = Self(-1250);
	const LVN_FIRST: Self = Self(-100);
	const TVN_FIRST: Self = Self(-400);
}
