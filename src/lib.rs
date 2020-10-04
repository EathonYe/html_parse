pub fn parse(html: &str) -> Node {
  let mut stack: Vec<String> = Vec::new();

  let mut in_tag = false;
  let mut tag = String::new();
  for char in html.chars() {
    if in_tag {
      if char != '>' {
        tag.push(char);
      } else {
        if &tag[tag.len() -1..tag.len()] == "/" {
          stack.push(String::from(String::from(&tag[0..tag.len()-1]).trim()));
        } else {
          stack.push(tag.clone());
        }
        in_tag = false;
        tag.clear();
      }
    } else {
      if char == '<' {
        in_tag = true;
      }
    }
  }

  println!("{:?}", stack);
  gen_tree(&stack)
}

fn gen_tree(stack: &Vec<String>) -> Node {
  let mut root = Node::new(String::from(&stack[0]));
  let node = &mut root;
  let mut start_index = 1;
  let mut last_index = stack.len() - 2;

  while start_index <= last_index {
    let tag = &stack[start_index];
    if &tag[0..1] == "/" {
      panic!("Tag `{}` is not matched.", tag);
    }

    let mut close_tag = String::from("/");
    close_tag.push_str(&stack[start_index]);
    if close_tag == stack[last_index] || start_index == last_index {
      node.children.push(Node::new(String::from(&stack[start_index])));
      start_index += 1;
      last_index -= 1;
    } else {
      node.children.push(Node::new(String::from(&stack[start_index])));
      start_index += 1;
    }
  }

  root
}

#[derive(Debug)]
pub struct Node {
  tag: String,
  children: Vec<Node>,
}

impl Node {
  fn new(tag: String) -> Node {
    Node {
      tag,
      children: Vec::new()
    }
  }
}
