use crate::prelude::*;

pub fn map_key_to_glyph_segment(key: VirtualKeyCode) -> Option<Segment> {
    match key {
        VirtualKeyCode::W => Some(0),
        VirtualKeyCode::E => Some(1),
        VirtualKeyCode::R => Some(2),

        VirtualKeyCode::A => Some(3),
        VirtualKeyCode::S => Some(4),
        VirtualKeyCode::D => Some(5),
        VirtualKeyCode::F => Some(6),

        VirtualKeyCode::U => Some(7),
        VirtualKeyCode::I => Some(8),
        VirtualKeyCode::O => Some(9),
        VirtualKeyCode::P => Some(10),

        VirtualKeyCode::J => Some(11),
        VirtualKeyCode::K => Some(12),
        VirtualKeyCode::L => Some(13),
        VirtualKeyCode::Semicolon => Some(14),
        VirtualKeyCode::Q => Some(15),

        _ => None,
    }
}

pub fn on_modify_selected_glyph(_editor: &GlyphEditor, key: Option<VirtualKeyCode>) -> EditorEvent {
    if let Some(key) = key {
        if let Some(segment) = map_key_to_glyph_segment(key) {
            EditorEvent::ToggleSegmentOnSelectedGlyph(segment)
        } else {
            match key {
                VirtualKeyCode::Left => EditorEvent::MoveGlyphCursorBackward,
                VirtualKeyCode::Right => EditorEvent::MoveGlyphCursorForward,
                VirtualKeyCode::Return => EditorEvent::AddNewTunicWordAtCursor,
                VirtualKeyCode::Space => EditorEvent::AddNewGlyphToTunicWordAtCursor,
                VirtualKeyCode::Back => EditorEvent::DeleteGlyphAtCursor,

                _ => EditorEvent::NoOp,
            }
        }
    } else {
        EditorEvent::NoOp
    }
}

pub fn on_modify_glyph_set(_editor: &WordEditor, key: Option<VirtualKeyCode>) -> EditorEvent {
    if let Some(key) = key {
        if let Some(segment) = map_key_to_glyph_segment(key) {
            EditorEvent::ToggleSegmentOnSelectedGlyph(segment)
        } else {
            match key {
                VirtualKeyCode::Left => EditorEvent::MoveGlyphCursorBackward,
                VirtualKeyCode::Right => EditorEvent::MoveGlyphCursorForward,
                VirtualKeyCode::Return => EditorEvent::AddNewTunicWordAtCursor,
                VirtualKeyCode::Space => EditorEvent::AddNewGlyphToTunicWordAtCursor,
                VirtualKeyCode::Back => EditorEvent::DeleteGlyphAtCursor,

                _ => EditorEvent::NoOp,
            }
        }
    } else {
        EditorEvent::NoOp
    }
}

pub fn on_snippet_editor_input(editor: &SnippetEditor, ctx: &BTerm) -> EditorEvent {
    if let Some(key) = ctx.key {
        match key {
            VirtualKeyCode::Up => EditorEvent::MoveWordCursorBackward,
            VirtualKeyCode::Down => EditorEvent::MoveWordCursorForward,
            VirtualKeyCode::Q => EditorEvent::ToggleGlyphEditingMode,
            _ => {
                let callbacks = WordEditorCallbacks {
                    on_modify_selected_glyph: Box::new(move |glyph_editor| {
                        on_modify_selected_glyph(glyph_editor, Some(key))
                    }),
                    on_modify_glyph_set: Box::new(move |word_editor| {
                        on_modify_glyph_set(word_editor, Some(key))
                    }),
                };

                editor.on_word_editor_input(callbacks)
            }
        }
    } else {
        EditorEvent::NoOp
    }
}

pub fn on_notebook_editor_input(editor: &NotebookEditor, ctx: &BTerm) -> EditorEvent {
    let ctx = ctx.clone();

    let callback: Box<dyn Fn(&SnippetEditor) -> EditorEvent> = Box::new(move |snippet_editor| {
        on_snippet_editor_input(snippet_editor, &ctx)
    });

    editor.on_snippet_editor_input(callback)
}
