use ratatui::{
  backend::Backend,
  layout::{Constraint, Rect},
  style::Style,
  text::Text,
  widgets::{Block, Borders, Paragraph, Wrap},
  Frame,
};

use super::utils::{
  horizontal_chunks, layout_block_with_line, style_default, style_primary, style_secondary,
  title_with_dual_style, vertical_chunks, vertical_chunks_with_margin,
};
use crate::app::{ActiveBlock, App, InputMode, Route, TextInput};

pub fn draw_decoder<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let chunks = horizontal_chunks(
    vec![Constraint::Percentage(50), Constraint::Percentage(50)],
    area,
  );
  draw_encoded_block(f, app, chunks[0]);
  draw_decoded_block(f, app, chunks[1]);
}

fn draw_encoded_block<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let block = get_selectable_block(
    "Encoded Token",
    app.data.decoder.blocks.get_active_route(),
    ActiveBlock::DecoderToken,
    Some(&app.data.decoder.encoded.input_mode),
    app.light_theme,
  );

  f.render_widget(block, area);

  let chunks = vertical_chunks_with_margin(vec![Constraint::Min(2)], area, 1);
  render_input_widget(f, chunks[0], &app.data.decoder.encoded, app.light_theme);
}

fn draw_decoded_block<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let chunks = vertical_chunks(
    vec![
      Constraint::Percentage(30),
      Constraint::Percentage(40),
      Constraint::Percentage(30),
    ],
    area,
  );

  draw_header_block(f, app, chunks[0]);
  draw_payload_block(f, app, chunks[1]);
  draw_signature_block(f, app, chunks[2]);
}

fn draw_header_block<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let block = get_selectable_block(
    "Header: Algorithm & Token Type",
    app.data.decoder.blocks.get_active_route(),
    ActiveBlock::DecoderHeader,
    None,
    app.light_theme,
  );

  f.render_widget(block, area);

  let chunks = vertical_chunks_with_margin(vec![Constraint::Min(2)], area, 1);

  let header = app.data.decoder.header.get_txt();
  let mut txt = Text::from(header.clone());
  txt.patch_style(style_primary(app.light_theme));

  let paragraph = Paragraph::new(txt)
    .block(Block::default())
    .wrap(Wrap { trim: false })
    .scroll((app.data.decoder.header.offset, 0));
  f.render_widget(paragraph, chunks[0]);
}

fn draw_payload_block<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let block = get_selectable_block(
    "Payload: Claims",
    app.data.decoder.blocks.get_active_route(),
    ActiveBlock::DecoderPayload,
    None,
    app.light_theme,
  );
  f.render_widget(block, area);

  let chunks = vertical_chunks_with_margin(vec![Constraint::Min(2)], area, 1);

  let payload = app.data.decoder.payload.get_txt();
  let mut txt = Text::from(payload.clone());
  txt.patch_style(style_primary(app.light_theme));

  let paragraph = Paragraph::new(txt)
    .block(Block::default())
    .wrap(Wrap { trim: false })
    .scroll((app.data.decoder.payload.offset, 0));
  f.render_widget(paragraph, chunks[0]);
}

fn draw_signature_block<B: Backend>(f: &mut Frame<'_, B>, app: &App, area: Rect) {
  let block = get_selectable_block(
    "Verify Signature",
    app.data.decoder.blocks.get_active_route(),
    ActiveBlock::DecoderSecret,
    Some(&app.data.decoder.secret.input_mode),
    app.light_theme,
  );

  f.render_widget(block, area);

  let chunks =
    vertical_chunks_with_margin(vec![Constraint::Length(1), Constraint::Min(2)], area, 1);

  let mut text = Text::from(
    "Prepend 'b64:' for base64 encoded secret. Prepend '@' for file path (.pem, .pk8, .der)",
  );
  text.patch_style(style_default(app.light_theme));
  let paragraph = Paragraph::new(text).block(Block::default());

  f.render_widget(paragraph, chunks[0]);

  render_input_widget(f, chunks[1], &app.data.decoder.secret, app.light_theme);
}

fn render_input_widget<B: Backend>(
  f: &mut Frame<'_, B>,
  chunk: Rect,
  text_input: &TextInput,
  light_theme: bool,
) {
  let width = chunk.width.max(3) - 3;
  // keep 2 for borders and 1 for cursor
  let scroll = text_input.input.visual_scroll(width as usize);
  let input = Paragraph::new(text_input.input.value())
    .wrap(Wrap { trim: false })
    .style(get_input_style(&text_input.input_mode, light_theme))
    .scroll((0, scroll as u16))
    .block(
      Block::default()
        .borders(Borders::ALL)
        .style(get_input_style(&text_input.input_mode, light_theme)),
    );

  f.render_widget(input, chunk);

  match text_input.input_mode {
    InputMode::Normal => {
      // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
    }

    InputMode::Editing => {
      // Make the cursor visible and ask tui-rs to put it at the specified coordinates after rendering
      f.set_cursor(
        // Put cursor past the end of the input text
        chunk.x + ((text_input.input.visual_cursor()).max(scroll) - scroll) as u16 + 1,
        // Move one line down, from the border to the input line
        chunk.y + 1,
      )
    }
  }
}

// Utility methods
fn get_hint(input_mode: &InputMode, is_active: bool) -> &str {
  if is_active {
    match input_mode {
      InputMode::Normal => "(Press <e> to edit | <c> to copy) ",
      InputMode::Editing => "(Press <esc> to stop editing | <c> to copy) ",
    }
  } else {
    ""
  }
}

fn get_selectable_block(
  title: &str,
  route: &Route,
  block: ActiveBlock,
  input_mode: Option<&InputMode>,
  light_theme: bool,
) -> Block<'static> {
  let is_active = route.active_block == block;
  let title_hint = if let Some(im) = input_mode {
    get_hint(im, is_active)
  } else if is_active {
    "(Press <c> to copy) "
  } else {
    ""
  };

  let block = layout_block_with_line(
    title_with_dual_style(format!(" {} ", title), title_hint.into()),
    light_theme,
    is_active,
  );
  block
}

fn get_input_style(input_mode: &InputMode, light: bool) -> Style {
  match input_mode {
    InputMode::Normal => style_default(light),
    InputMode::Editing => style_secondary(light),
  }
}

#[cfg(test)]
mod tests {}
