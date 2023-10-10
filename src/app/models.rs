use ratatui::{
  backend::Backend,
  layout::Rect,
  widgets::{ListState, TableState},
  Frame,
};

use super::{ActiveBlock, App, Route};

pub trait AppResource {
  fn render<B: Backend>(block: ActiveBlock, f: &mut Frame<'_, B>, app: &mut App, area: Rect);
}

pub trait Scrollable {
  fn handle_scroll(&mut self, up: bool, page: bool) {
    // support page up/down
    let inc_or_dec = if page { 10 } else { 1 };
    if up {
      self.scroll_up(inc_or_dec);
    } else {
      self.scroll_down(inc_or_dec);
    }
  }
  fn scroll_down(&mut self, inc_or_dec: usize);
  fn scroll_up(&mut self, inc_or_dec: usize);
}

pub struct StatefulList<T> {
  pub state: ListState,
  pub items: Vec<T>,
}

impl<T> StatefulList<T> {
  pub fn new() -> StatefulList<T> {
    StatefulList {
      state: ListState::default(),
      items: Vec::new(),
    }
  }
  pub fn with_items(items: Vec<T>) -> StatefulList<T> {
    let mut state = ListState::default();
    if !items.is_empty() {
      state.select(Some(0));
    }
    StatefulList { state, items }
  }
}

impl<T> Scrollable for StatefulList<T> {
  // for lists we cycle back to the beginning when we reach the end
  fn scroll_down(&mut self, increment: usize) {
    let i = match self.state.selected() {
      Some(i) => {
        if i >= self.items.len().saturating_sub(increment) {
          0
        } else {
          i + increment
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
  }
  // for lists we cycle back to the end when we reach the beginning
  fn scroll_up(&mut self, decrement: usize) {
    let i = match self.state.selected() {
      Some(i) => {
        if i == 0 {
          self.items.len().saturating_sub(decrement)
        } else {
          i.saturating_sub(decrement)
        }
      }
      None => 0,
    };
    self.state.select(Some(i));
  }
}

#[derive(Clone, Debug)]
pub struct StatefulTable<T> {
  pub state: TableState,
  pub items: Vec<T>,
}

impl<T> StatefulTable<T> {
  pub fn new() -> StatefulTable<T> {
    StatefulTable {
      state: TableState::default(),
      items: Vec::new(),
    }
  }

  pub fn with_items(items: Vec<T>) -> StatefulTable<T> {
    let mut table = StatefulTable::new();
    if !items.is_empty() {
      table.state.select(Some(0));
    }
    table.set_items(items);
    table
  }

  pub fn set_items(&mut self, items: Vec<T>) {
    let item_len = items.len();
    self.items = items;
    if !self.items.is_empty() {
      let i = self.state.selected().map_or(0, |i| {
        if i > 0 && i < item_len {
          i
        } else if i >= item_len {
          item_len - 1
        } else {
          0
        }
      });
      self.state.select(Some(i));
    }
  }
}

impl<T> Scrollable for StatefulTable<T> {
  fn scroll_down(&mut self, increment: usize) {
    if let Some(i) = self.state.selected() {
      if (i + increment) < self.items.len() {
        self.state.select(Some(i + increment));
      } else {
        self.state.select(Some(self.items.len().saturating_sub(1)));
      }
    }
  }

  fn scroll_up(&mut self, decrement: usize) {
    if let Some(i) = self.state.selected() {
      if i != 0 {
        self.state.select(Some(i.saturating_sub(decrement)));
      }
    }
  }
}

impl<T: Clone> StatefulTable<T> {
  /// a clone of the currently selected item.
  /// for mutable ref use state.selected() and fetch from items when needed
  pub fn get_selected_item_copy(&self) -> Option<T> {
    if !self.items.is_empty() {
      self.state.selected().map(|i| self.items[i].clone())
    } else {
      None
    }
  }
}

#[derive(Clone)]
pub struct TabRoute {
  pub title: String,
  pub route: Route,
}
#[derive(Default)]
pub struct TabsState {
  pub items: Vec<TabRoute>,
  pub index: usize,
}

impl TabsState {
  pub fn new(items: Vec<TabRoute>) -> TabsState {
    TabsState { items, index: 0 }
  }
  pub fn set_index(&mut self, index: usize) -> &TabRoute {
    self.index = index;
    &self.items[self.index]
  }
  pub fn get_active_route(&self) -> &Route {
    &self.items[self.index].route
  }

  pub fn next(&mut self) {
    self.index = (self.index + 1) % self.items.len();
  }
  pub fn previous(&mut self) {
    if self.index > 0 {
      self.index -= 1;
    } else {
      self.index = self.items.len() - 1;
    }
  }
}

#[derive(Debug, Eq, PartialEq, Default)]
pub struct ScrollableTxt {
  items: Vec<String>,
  pub offset: u16,
}

impl ScrollableTxt {
  pub fn new(item: String) -> ScrollableTxt {
    let items: Vec<&str> = item.split('\n').collect();
    let items: Vec<String> = items.iter().map(|it| it.to_string()).collect();
    ScrollableTxt { items, offset: 0 }
  }

  pub fn get_txt(&self) -> String {
    self.items.join("\n")
  }
}

impl Scrollable for ScrollableTxt {
  fn scroll_down(&mut self, increment: usize) {
    // scroll only if offset is less than total lines in text
    // we subtract increment + 2 to keep the text in view. Its just an arbitrary number that works
    if self.offset < self.items.len().saturating_sub(increment + 2) as u16 {
      self.offset += increment as u16;
    }
  }
  fn scroll_up(&mut self, decrement: usize) {
    // scroll up and avoid going negative
    if self.offset > 0 {
      self.offset = self.offset.saturating_sub(decrement as u16);
    }
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn test_kube_resource() {
    struct TestStruct {
      name: String,
    }

    // assert_eq!(
    //   ts.resource_to_yaml(),
    //   "apiVersion: v1\nkind: Namespace\nmetadata:\n  name: test\n  namespace: test\n"
    // )
  }

  //   #[test]
  //   fn test_stateful_table() {
  //     let mut sft: StatefulTable<KubeNs> = StatefulTable::new();

  //     assert_eq!(sft.items.len(), 0);
  //     assert_eq!(sft.state.selected(), None);
  //     // check default selection on set
  //     sft.set_items(vec![KubeNs::default(), KubeNs::default()]);
  //     assert_eq!(sft.items.len(), 2);
  //     assert_eq!(sft.state.selected(), Some(0));
  //     // check selection retain on set
  //     sft.state.select(Some(1));
  //     sft.set_items(vec![
  //       KubeNs::default(),
  //       KubeNs::default(),
  //       KubeNs::default(),
  //     ]);
  //     assert_eq!(sft.items.len(), 3);
  //     assert_eq!(sft.state.selected(), Some(1));
  //     // check selection overflow prevention
  //     sft.state.select(Some(2));
  //     sft.set_items(vec![KubeNs::default(), KubeNs::default()]);
  //     assert_eq!(sft.items.len(), 2);
  //     assert_eq!(sft.state.selected(), Some(1));
  //     // check scroll down
  //     sft.state.select(Some(0));
  //     assert_eq!(sft.state.selected(), Some(0));
  //     sft.scroll_down(1);
  //     assert_eq!(sft.state.selected(), Some(1));
  //     // check scroll overflow
  //     sft.scroll_down(1);
  //     assert_eq!(sft.state.selected(), Some(1));
  //     sft.scroll_up(1);
  //     assert_eq!(sft.state.selected(), Some(0));
  //     // check scroll overflow
  //     sft.scroll_up(1);
  //     assert_eq!(sft.state.selected(), Some(0));
  //     // check increment
  //     sft.scroll_down(10);
  //     assert_eq!(sft.state.selected(), Some(1));

  //     let sft2 = StatefulTable::with_items(vec![KubeNs::default(), KubeNs::default()]);
  //     assert_eq!(sft2.state.selected(), Some(0));
  //   }

  //   #[test]
  //   fn test_handle_table_scroll() {
  //     let mut item: StatefulTable<&str> = StatefulTable::new();
  //     item.set_items(vec!["A", "B", "C"]);

  //     assert_eq!(item.state.selected(), Some(0));

  //     item.handle_scroll(false, false);
  //     assert_eq!(item.state.selected(), Some(1));

  //     item.handle_scroll(false, false);
  //     assert_eq!(item.state.selected(), Some(2));

  //     item.handle_scroll(false, false);
  //     assert_eq!(item.state.selected(), Some(2));
  //     // previous
  //     item.handle_scroll(true, false);
  //     assert_eq!(item.state.selected(), Some(1));
  //     // page down
  //     item.handle_scroll(false, true);
  //     assert_eq!(item.state.selected(), Some(2));
  //     // page up
  //     item.handle_scroll(true, true);
  //     assert_eq!(item.state.selected(), Some(0));
  //   }

  //   #[test]
  //   fn test_stateful_tab() {
  //     let mut tab = TabsState::new(vec![
  //       TabRoute {
  //         title: "Hello".into(),
  //         route: Route {
  //           active_block: ActiveBlock::Pods,
  //           id: RouteId::Home,
  //         },
  //       },
  //       TabRoute {
  //         title: "Test".into(),
  //         route: Route {
  //           active_block: ActiveBlock::Nodes,
  //           id: RouteId::Home,
  //         },
  //       },
  //     ]);

  //     assert_eq!(tab.index, 0);
  //     assert_eq!(tab.get_active_route().active_block, ActiveBlock::Pods);
  //     tab.next();
  //     assert_eq!(tab.index, 1);
  //     assert_eq!(tab.get_active_route().active_block, ActiveBlock::Nodes);
  //     tab.next();
  //     assert_eq!(tab.index, 0);
  //     assert_eq!(tab.get_active_route().active_block, ActiveBlock::Pods);
  //     tab.previous();
  //     assert_eq!(tab.index, 1);
  //     assert_eq!(tab.get_active_route().active_block, ActiveBlock::Nodes);
  //     tab.previous();
  //     assert_eq!(tab.index, 0);
  //     assert_eq!(tab.get_active_route().active_block, ActiveBlock::Pods);
  //   }
  #[test]
  fn test_scrollable_txt() {
    let mut stxt = ScrollableTxt::new("test\n multiline\n string".into());

    assert_eq!(stxt.offset, 0);
    assert_eq!(stxt.items.len(), 3);

    assert_eq!(stxt.get_txt(), "test\n multiline\n string");

    stxt.scroll_down(1);
    assert_eq!(stxt.offset, 0);

    let mut stxt2 = ScrollableTxt::new("te\nst\nmul\ntil\ni\nne\nstr\ni\nn\ng".into());
    assert_eq!(stxt2.items.len(), 10);
    stxt2.scroll_down(1);
    assert_eq!(stxt2.offset, 1);
    stxt2.scroll_down(1);
    assert_eq!(stxt2.offset, 2);
    stxt2.scroll_down(5);
    assert_eq!(stxt2.offset, 7);
    stxt2.scroll_down(1);
    // no overflow past (len - 2)
    assert_eq!(stxt2.offset, 7);
    stxt2.scroll_up(1);
    assert_eq!(stxt2.offset, 6);
    stxt2.scroll_up(6);
    assert_eq!(stxt2.offset, 0);
    stxt2.scroll_up(1);
    // no overflow past (0)
    assert_eq!(stxt2.offset, 0);
  }
}
