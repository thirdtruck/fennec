use std::error::Error;

use crate::prelude::*;

pub struct FennecState {
    file_editor: FileEditor,
    dictionary: Dictionary,
}

impl FennecState {
    pub fn new(snippet: Snippet, dictionary: Dictionary) -> Self {
        let notebook: Notebook = vec![snippet].into();
        let file_editor = FileEditor::new(notebook.clone(), DEFAULT_NOTEBOOK_FILE);
        let file_editor = file_editor.apply(EditorEvent::ConfirmLoadFromFileRequest);

        Self { file_editor, dictionary }
    }

    fn render(&self, map: &mut GlyphMap, ctx: &mut BTerm) -> Result<(), Box<dyn Error>> {
        self.file_editor.render_with(|file_editor_view| {
            ctx.set_active_console(FILE_CONSOLE);
            ctx.cls();
            ctx.set_active_console(NOTEBOOK_CONSOLE);
            ctx.cls();
            ctx.set_active_console(SNIPPET_CONSOLE);
            ctx.cls();

            match &file_editor_view.state {
                FileEditorState::Idle => {
                    let notebook_view = &file_editor_view.notebook_view;
                    render_notebook_on(notebook_view, &self.dictionary, map, ctx, 1, 1)?;
                }
                _ => render_file_editor_view_onto(&file_editor_view, ctx)?,
            };

            Ok(())
        })?;

        map.draw_on(ctx, 1, 1)?;

        Ok(())
    }

    fn emergency_backup_and_abort(&self, error: Box<dyn Error>) {
        println!("Rendering error");
        dbg!(error);

        let file = self.file_editor.target_file();
        let file = format!("{}~", file);

        println!("Trying to save notebook backup to: {}", &file);

        let notebook = self.file_editor.to_source();

        notebook_to_yaml_file(&notebook, &file).expect("Failed to save notebook backup");

        println!("Saved notebook backup");

        panic!("Aborting!");
    }
}

impl GameState for FennecState {
    fn tick(&mut self, ctx: &mut BTerm) {
        let mut map = GlyphMap::new(100, 100).expect("Invalid map dimensions");

        let ctx_clone = ctx.clone();

        self.file_editor = {
            let file_editor = self.file_editor.clone();

            let event = file_editor.on_input(Box::new(move |editor| {
                on_file_editor_input(editor, &ctx_clone)
            }));

            if event != EditorEvent::NoOp {
                file_editor.apply(event)
            } else {
                file_editor
            }
        };

        self.render(&mut map, ctx)
            .map_err(|error| self.emergency_backup_and_abort(error))
            .unwrap();

        render_draw_buffer(ctx)
            .map_err(|error| self.emergency_backup_and_abort(error))
            .unwrap();
    }
}
