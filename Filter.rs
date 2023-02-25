pub struct Filter <'a> {
    /// The filter's filter function.
    pub kernel: Box<Fn(f32) -> f32 + 'a>,

    /// The window on which this filter operates.
    pub support: f32
}