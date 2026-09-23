// ── Modules ─────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests;

// ── `struct SetLenOnDrop<'a>` Definition ────────────────────────────────────
pub(super) struct SetLenOnDrop<'a> {
    len: &'a mut usize,
    local_len: usize
}

// ── `SetLenOnDrop<'a>` Implementation ───────────────────────────────────────
impl<'a> SetLenOnDrop<'a> {
    // ── Functions ───────────────────────────────────────────────────────────
    // TODO
    pub(super) fn new(len: &'a mut usize) -> Self { todo!(); }

    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    pub(super) fn increment_len(&mut self, increment: usize) { todo!(); }

    // TODO
    pub(super) fn current_len(&self) -> usize { todo!(); }
}

// ── `Drop for SetLenOnDrop<'_>` Implementation ──────────────────────────────
impl Drop for SetLenOnDrop<'_> {
    // ── Methods ─────────────────────────────────────────────────────────────
    // TODO
    fn drop(&mut self) { todo!(); }
}
