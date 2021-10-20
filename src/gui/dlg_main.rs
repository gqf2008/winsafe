use std::sync::Arc;

use crate::aliases::{ErrResult, WinResult};
use crate::co;
use crate::enums::IdStr;
use crate::funcs::PostQuitMessage;
use crate::gui::base::Base;
use crate::gui::dlg_base::DlgBase;
use crate::gui::events::{EventsView, WindowEventsAll};
use crate::gui::traits::{ParentEvents, UiThread, Window};
use crate::handles::{HINSTANCE, HWND};
use crate::msg::wm;

#[derive(Clone)]
pub(in crate::gui) struct DlgMain(Arc<Obj>);

struct Obj { // actual fields of DlgMain
	base: DlgBase,
	icon_id: Option<u16>,
	accel_table_id: Option<u16>,
}

impl Window for DlgMain {
	fn hwnd(&self) -> HWND {
		self.0.base.hwnd()
	}
}

impl UiThread for DlgMain {
	fn run_ui_thread<F>(&self, func: F)
		where F: FnOnce() -> ErrResult<()>,
	{
		self.0.base.run_ui_thread(func);
	}
}

impl ParentEvents for DlgMain {
	fn on(&self) -> &WindowEventsAll {
		self.0.base.on()
	}
}

impl DlgMain {
	pub(in crate::gui) fn new(
		dialog_id: u16,
		icon_id: Option<u16>,
		accel_table_id: Option<u16>) -> DlgMain
	{
		let dlg = Self(
			Arc::new(
				Obj {
					base: DlgBase::new(None, dialog_id),
					icon_id,
					accel_table_id,
				},
			),
		);
		dlg.default_message_handlers();
		dlg
	}

	pub(in crate::gui) fn base_ref(&self) -> &Base {
		self.0.base.base_ref()
	}

	pub(in crate::gui) fn run_main(&self,
		cmd_show: Option<co::SW>) -> ErrResult<i32>
	{
		self.0.base.create_dialog_param()?; // may panic
		let hinst = self.base_ref().parent_hinstance()?;
		let haccel = self.0.accel_table_id
			.map(|id| hinst.LoadAccelerators(IdStr::Id(id))) // resources are automatically freed
			.transpose()?;

		self.set_icon_if_any(hinst)?;

		let hwnd = *self.base_ref().hwnd_ref();
		hwnd.ShowWindow(cmd_show.unwrap_or(co::SW::SHOW));

		Base::run_main_loop(haccel) // blocks until window is closed
	}

	fn default_message_handlers(&self) {
		self.base_ref().default_message_handlers();

		self.on().wm_close({
			let self2 = self.clone();
			move || { self2.base_ref().hwnd_ref().DestroyWindow()?; Ok(()) }
		});

		self.on().wm_nc_destroy(|| {
			PostQuitMessage(0);
			Ok(())
		});
	}

	fn set_icon_if_any(&self, hinst: HINSTANCE) -> WinResult<()> {
		// If an icon ID was specified, load it from the resources.
		// Resource icons are automatically released by the system.
		if let Some(id) = self.0.icon_id {
			self.base_ref().hwnd_ref().SendMessage(
				wm::SetIcon {
					hicon: hinst.LoadImageIcon(id, 16, 16, co::LR::DEFAULTCOLOR)?,
					size: co::ICON_SZ::SMALL,
				},
			);

			self.base_ref().hwnd_ref().SendMessage(
				wm::SetIcon {
					hicon: hinst.LoadImageIcon(id, 32, 32, co::LR::DEFAULTCOLOR)?,
					size: co::ICON_SZ::BIG,
				},
			);
		}

		Ok(())
	}
}
