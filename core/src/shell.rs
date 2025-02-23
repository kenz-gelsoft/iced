use crate::window;
use crate::InputMethod;

/// A connection to the state of a shell.
///
/// A [`Widget`] can leverage a [`Shell`] to trigger changes in an application,
/// like publishing messages or invalidating the current layout.
///
/// [`Widget`]: crate::Widget
#[derive(Debug)]
pub struct Shell<'a, Message> {
    messages: &'a mut Vec<Message>,
    redraw_request: Option<window::RedrawRequest>,
    input_method: InputMethod,
    is_layout_invalid: bool,
    are_widgets_invalid: bool,
}

impl<'a, Message> Shell<'a, Message> {
    /// Creates a new [`Shell`] with the provided buffer of messages.
    pub fn new(messages: &'a mut Vec<Message>) -> Self {
        Self {
            messages,
            redraw_request: None,
            is_layout_invalid: false,
            are_widgets_invalid: false,
            input_method: InputMethod::Disabled,
        }
    }

    /// Returns true if the [`Shell`] contains no published messages
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Publish the given `Message` for an application to process it.
    pub fn publish(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Requests a new frame to be drawn.
    pub fn request_redraw(&mut self, request: window::RedrawRequest) {
        match self.redraw_request {
            None => {
                self.redraw_request = Some(request);
            }
            Some(current) if request < current => {
                self.redraw_request = Some(request);
            }
            _ => {}
        }
    }

    /// Returns the request a redraw should happen, if any.
    pub fn redraw_request(&self) -> Option<window::RedrawRequest> {
        self.redraw_request
    }

    /// Requests the current [`InputMethod`] strategy.
    ///
    /// __Important__: This request will only be honored by the
    /// [`Shell`] only during a [`window::Event::RedrawRequested`].
    pub fn request_input_method<T: AsRef<str>>(
        &mut self,
        ime: &InputMethod<T>,
    ) {
        self.input_method.merge(ime);
    }
    /// Returns the current [`InputMethod`] strategy.
    pub fn input_method(&self) -> &InputMethod {
        &self.input_method
    }
    /// Returns the current [`InputMethod`] strategy.
    pub fn input_method_mut(&mut self) -> &mut InputMethod {
        &mut self.input_method
    }

    /// Returns whether the current layout is invalid or not.
    pub fn is_layout_invalid(&self) -> bool {
        self.is_layout_invalid
    }

    /// Invalidates the current application layout.
    ///
    /// The shell will relayout the application widgets.
    pub fn invalidate_layout(&mut self) {
        self.is_layout_invalid = true;
    }

    /// Triggers the given function if the layout is invalid, cleaning it in the
    /// process.
    pub fn revalidate_layout(&mut self, f: impl FnOnce()) {
        if self.is_layout_invalid {
            self.is_layout_invalid = false;

            f();
        }
    }

    /// Returns whether the widgets of the current application have been
    /// invalidated.
    pub fn are_widgets_invalid(&self) -> bool {
        self.are_widgets_invalid
    }

    /// Invalidates the current application widgets.
    ///
    /// The shell will rebuild and relayout the widget tree.
    pub fn invalidate_widgets(&mut self) {
        self.are_widgets_invalid = true;
    }

    /// Merges the current [`Shell`] with another one by applying the given
    /// function to the messages of the latter.
    ///
    /// This method is useful for composition.
    pub fn merge<B>(&mut self, other: Shell<'_, B>, f: impl Fn(B) -> Message) {
        self.messages.extend(other.messages.drain(..).map(f));

        if let Some(at) = other.redraw_request {
            self.request_redraw(at);
        }

        self.is_layout_invalid =
            self.is_layout_invalid || other.is_layout_invalid;

        self.are_widgets_invalid =
            self.are_widgets_invalid || other.are_widgets_invalid;

        self.input_method.merge(&other.input_method);
    }
}
