// Created by Eric "Sk3pz" Shreve under the MIT license.

use std::fmt::{Display, Formatter};

/// A node system that will display as a tree in the terminal
/// using unicode characters.
/// Originally designed to display the Abstract Syntax Tree of my Programming language.
/// 
/// # Example:
/// ```
/// use cli_tree::TreeNode;
///
/// // This will print:
/// // Test Node
/// //  ├─Child 1
/// //  ├─Child 2
/// //  │ ├─Child 2.1
/// //  │ ├─Child 2.2
/// //  │ ├─Child 2.3
/// //  │ │ ├─Child 2.3.1
/// //  │ │ └─Child 2.3.2
/// //  │ └─Child 2.4
/// //  │   ├─Child 2.4.1
/// //  │   ├─Child 2.4.2
/// //  │   └─Child 2.4.3
/// //  └─Child 3
///
/// let mut node = TreeNode::new("Test Node");
/// // add a single child
/// node.add_child(TreeNode::new("Child 1"));
/// // add a child with children which also have children
/// node.add_child(TreeNode::new_with_children("Child 2",
///                vec![TreeNode::new("Child 2.1"),
///                     TreeNode::new("Child 2.2"),
///                     TreeNode::new_with_children("Child 2.3",
///                                   vec![TreeNode::new("Child 2.3.1"),
///                                        TreeNode::new("Child 2.3.2")]),
///                     TreeNode::new_with_children("Child 2.4",
///                                   vec![TreeNode::new("Child 2.4.1"),
///                                        TreeNode::new("Child 2.4.2"),
///                                        TreeNode::new("Child 2.4.3")])]));
/// // add another singular child
/// node.add_child(TreeNode::new("Child 3"));
/// println!("{}", node);
/// ```
///
pub struct TreeNode {
    name: String,
    children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn new<S: Into<String>>(name: S) -> TreeNode {
        TreeNode {
            name: name.into(),
            children: Vec::new(),
        }
    }

    pub fn new_with_children<S: Into<String>>(name: S, children: Vec<TreeNode>) -> TreeNode {
        TreeNode {
            name: name.into(),
            children,
        }
    }

    pub fn add_child(&mut self, child: TreeNode) {
        self.children.push(child);
    }

    pub fn has_children(&self) -> bool {
        !self.children.is_empty()
    }

    fn display(&self, indent: &mut Vec<bool>, is_child: bool, is_last: bool) -> String {
        // each "Node" is represented by a line in the terminal
        // draw the lines before the current node
        let mut output = String::new();

        if indent.len() != 0 {
            // print out the indentation
            for b in indent.clone() {
                match b {
                    true => output.push_str("│ "),
                    false => output.push_str("  "),
                }
            }
        }

        // print out the connection type and the name of the node
        if is_child {
            if is_last {
                output.push_str("└─");
            } else {
                output.push_str("├─");
            }
        }
        output.push_str(&self.name);
        output.push_str("\n");

        let pushed = self.has_children() && is_child;
        if pushed {
            indent.push(!is_last);
        }

        // print out the children
        for x in 0..self.children.len() {
            let child = self.children.get(x).unwrap();
            let last = x == self.children.len() - 1;

            output.push_str(child.display(indent, true, last).as_str());
        }

        if !indent.is_empty() && pushed {
            indent.remove(indent.len() - 1);
        }

        output
    }
}

impl Display for TreeNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut indent: Vec<bool> = Vec::new();
        write!(f, "{}", self.display(&mut indent, false, false))
    }
}