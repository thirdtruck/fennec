use crate::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EditorEvent {
    NoOp,
    ToggleSegmentOnSelectedGlyph(Segment),
    MoveGlyphCursorForward,
    MoveGlyphCursorBackward,
    MoveWordCursorForward,
    MoveWordCursorBackward,
    MoveSnippetCursorForward,
    MoveSnippetCursorBackward,
    ToggleGlyphEditingMode,
    AddNewTunicWordAtCursor,
    AddNewGlyphToTunicWordAtCursor,
    DeleteGlyphAtCursor,
    DeleteWordAtCursor,
    RequestLoadFromFile,
    RequestSaveToFile,
    ConfirmLoadFromFileRequest,
    ConfirmSaveToFileRequest,
    ReportLoadedFromFile(Notebook),
    ReportSavedToFile,
    ReportFailedToLoadFromFile(FileEditorError),
    ReportFailedToSaveToFile(FileEditorError),
    ResetFileEditorToIdle,
    EnableSnippetNavigationMode,
    EnableSnippetEditingMode,
}
