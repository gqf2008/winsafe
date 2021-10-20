//! Structs which expose the event methods of windows and controls.
//!
//! You cannot directly instantiate these objects, they are created internally
//! by the controls.

#[macro_use]
mod macros;

mod events_bn_rg;
mod events_bn;
mod events_cbn;
mod events_dtn;
mod events_en;
mod events_lbn;
mod events_lvn;
mod events_mcn;
mod events_sbn;
mod events_stn;
mod events_trbn;
mod events_tvn;
mod events_wm;
mod events_wm_nfy;
mod func_store;

pub use events_bn_rg::RadioGroupEvents;
pub use events_bn::ButtonEvents;
pub use events_cbn::ComboBoxEvents;
pub use events_dtn::DateTimePickerEvents;
pub use events_en::EditEvents;
pub use events_lbn::ListBoxEvents;
pub use events_lvn::ListViewEvents;
pub use events_mcn::MonthCalendarEvents;
pub use events_sbn::StatusBarEvents;
pub use events_stn::LabelEvents;
pub use events_trbn::TrackbarEvents;
pub use events_tvn::TreeViewEvents;
pub use events_wm::WindowEvents;
pub use events_wm_nfy::WindowEventsAll;
pub(in crate::gui) use events_wm::*;
pub(in crate::gui) use events_wm_nfy::*;
pub(in crate::gui) use func_store::FuncStore;

pub mod prelude {
	pub use super::events_wm::EventsView;
	pub use super::events_wm_nfy::EventsViewAll;
}
