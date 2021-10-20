/// Implements Debug trait to control.
macro_rules! impl_debug {
	($name:ident) => {
		impl std::fmt::Debug for $name {
			fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
				write!(f, "HWND {}, ID {}",
					self.hwnd(),
					self.ctrl_id(),
				)
			}
		}
	};
}

/// Implements Window trait to control.
macro_rules! impl_window {
	($name:ident) => {
		impl crate::gui::traits::Window for $name {
			fn hwnd(&self) -> crate::handles::HWND {
				self.0.base.hwnd()
			}
		}
	};
}

/// Implements Child trait to control.
macro_rules! impl_child {
	($name:ident) => {
		impl crate::gui::traits::Child for $name {
			fn ctrl_id(&self) -> u16 {
				match &self.0.opts_id {
					OptsId::Wnd(opts) => opts.ctrl_id,
					OptsId::Dlg(ctrl_id) => *ctrl_id,
				}
			}
		}
	};
}

/// Implements NativeControl trait to control.
macro_rules! impl_nativecontrol {
	($name:ident) => {
		impl crate::gui::traits::NativeControl for $name {
			fn on_subclass(&self) -> &crate::gui::events::WindowEvents {
				self.0.base.on_subclass()
			}
		}
	};
}

/// Implements NativeControlEvents trait to control.
macro_rules! impl_nativecontrolevents {
	($name:ident, $events:ty) => {
		impl crate::gui::traits::NativeControlEvents<$events> for $name {
			fn on(&self) -> &$events {
				if !self.0.base.hwnd_ref().is_null() {
					panic!("Cannot add events after the control is created.");
				} else if !self.0.base.parent_base_ref().hwnd_ref().is_null() {
					panic!("Cannot add events after the parent window is created.");
				}
				&self.0.events
			}
		}
	};
}

/// Implements Focus trait to control.
macro_rules! impl_focus {
	($name:ident) => {
		impl crate::gui::traits::Focus for $name {}
	};
}
