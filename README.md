# cli_tree
A simple library that displays a set of defined nodes as a tree in the terminal

## Example:
```rust
use cli_tree::TreeNode;

// This will print:
// Test Node
//  ├─Child 1
//  ├─Child 2
//  │ ├─Child 2.1
//  │ ├─Child 2.2
//  │ ├─Child 2.3
//  │ │ ├─Child 2.3.1
//  │ │ └─Child 2.3.2
//  │ └─Child 2.4
//  │   ├─Child 2.4.1
//  │   ├─Child 2.4.2
//  │   └─Child 2.4.3
//  └─Child 3

let mut node = TreeNode::new("Test Node");
// add a single child
node.add_child(TreeNode::new("Child 1"));
// add a child with children which also have children
node.add_child(TreeNode::new_with_children("Child 2",
               vec![TreeNode::new("Child 2.1"),
                    TreeNode::new("Child 2.2"),
                    TreeNode::new_with_children("Child 2.3",
                                  vec![TreeNode::new("Child 2.3.1"),
                                       TreeNode::new("Child 2.3.2")]),
                    TreeNode::new_with_children("Child 2.4",
                                  vec![TreeNode::new("Child 2.4.1"),
                                       TreeNode::new("Child 2.4.2"),
                                       TreeNode::new("Child 2.4.3")])]));
// add another singular child
node.add_child(TreeNode::new("Child 3"));
println!("{}", node);
```

Plans:\
[X] Properly generate the tree\
[-] Small optimizations\
[-] Make it easier to generate nodes with many layers of children
