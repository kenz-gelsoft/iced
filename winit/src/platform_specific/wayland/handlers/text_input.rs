use std::ops::Deref;

use cctk::sctk::globals::GlobalData;
use cctk::sctk::reexports::client::{Connection, Proxy, QueueHandle};

use cctk::sctk::reexports::client::delegate_dispatch;
use cctk::sctk::reexports::client::globals::{BindError, GlobalList};
use cctk::sctk::reexports::client::protocol::wl_surface::WlSurface;
use cctk::sctk::reexports::client::Dispatch;
use cctk::sctk::reexports::protocols::wp::text_input::zv3::client::zwp_text_input_manager_v3::ZwpTextInputManagerV3;
use cctk::sctk::reexports::protocols::wp::text_input::zv3::client::zwp_text_input_v3::Event as TextInputEvent;
use cctk::sctk::reexports::protocols::wp::text_input::zv3::client::zwp_text_input_v3::{
    ContentHint, ContentPurpose, ZwpTextInputV3,
};
use winit::event::{Ime, WindowEvent};
use winit::window::ImePurpose;

use crate::event_loop::state::SctkState;

pub struct TextInputManager {
    text_input_manager: ZwpTextInputManagerV3,
}

impl TextInputManager {
    pub fn new(
        globals: &GlobalList,
        queue_handle: &QueueHandle<SctkState>,
    ) -> Result<Self, BindError> {
        let text_input_manager =
            globals.bind(queue_handle, 1..=1, GlobalData)?;
        Ok(Self { text_input_manager })
    }
}

impl Deref for TextInputManager {
    type Target = ZwpTextInputManagerV3;

    fn deref(&self) -> &Self::Target {
        &self.text_input_manager
    }
}

impl Dispatch<ZwpTextInputManagerV3, GlobalData, SctkState>
    for TextInputManager
{
    fn event(
        _state: &mut SctkState,
        _proxy: &ZwpTextInputManagerV3,
        _event: <ZwpTextInputManagerV3 as Proxy>::Event,
        _data: &GlobalData,
        _conn: &Connection,
        _qhandle: &QueueHandle<SctkState>,
    ) {
    }
}

impl Dispatch<ZwpTextInputV3, (), SctkState> for TextInputManager {
    fn event(
        state: &mut SctkState,
        text_input: &ZwpTextInputV3,
        event: <ZwpTextInputV3 as Proxy>::Event,
        data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<SctkState>,
    ) {
    }
}

delegate_dispatch!(SctkState: [ZwpTextInputManagerV3: GlobalData] => TextInputManager);
delegate_dispatch!(SctkState: [ZwpTextInputV3: ()] => TextInputManager);
