//mod deserialize;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Weak};
use std::thread;

/*
{
  "nodes": [
    {
      "x": 0
    },
    {
      "x": 1
    }
  ]
}
 */

#[derive(Debug, Default, Serialize, Deserialize)]
struct Node {
    x: i32,
    #[serde(skip)]
    children: Vec<Weak<Node>>,
    #[serde(skip)]
    parent: Weak<Node>,
}

impl Node {
    // fn new(x: i32) -> Self {
    //     Self { x, .. Self::default() }
    // }
}

impl Drop for Node {
    fn drop(&mut self) {
        println!("drop {}", self.x);
    }
}

fn create_child(x: i32, parent: Weak<Node>) -> Arc<Node> {
    Arc::new(Node {
        x: x,
        parent: parent,
        children: vec![],
    })
}

fn create_group(v: &mut Vec<Arc<Node>>, x: i32, members: Vec<i32>) -> Arc<Node> {
    Arc::new_cyclic(|weak| {
        let mut node = Node {
            x: x,
            parent: Weak::default(),
            children: vec![],
        };

        for id in members {
            let n = create_child(id, weak.clone());
            v.push(n.clone());
            node.children.push(Arc::downgrade(&n));
        }

        node
    })
}

#[derive(Debug, Serialize, Deserialize)]
struct World {
    //nodes: Vec<Arc<Node>>,
    nodes: Vec<Node>,
}

fn serialize() {
    let world = World {
        nodes: vec![
            Node { x: 0, parent: Default::default(), children: Default::default() },
            Node { x: 1, parent: Default::default(), children: Default::default() },
        ]
    };
    let serialized = serde_json::to_string(&world).unwrap();
    println!("serialized = {}", serialized);
}

fn deserialize() {
    let json = r#"{
  "nodes": [
    {
      "x": 0
    },
    {
      "x": 1
    }
  ]
}"#;

    let deserialized: World = serde_json::from_str(json).unwrap();
    println!("deserialized = {:?}", deserialized);
}

fn main() {
    serialize();
    deserialize();
    return;

    let mut v: Vec<Arc<Node>> = vec![];

    // Can we have children first? In a separate vec:
    let mut existing_children = vec![Arc::new(Node {
        x: 10,
        parent: Weak::default(),
        children: vec![],
    })];

    // How about in the same vec? Nope...
    //v.push(Arc::new(Node {x: 20, parent: Weak::default(), children: vec![] }));
    //let mut sn = &mut v[0];

    // Parent:
    // So, provide the DATA to construct the children, not the children themselves
    let n0 = create_group(&mut v, 0, vec![1, 2, 3]);
    // let n0 = Arc::new_cyclic(|weak| {
    //     let mut node =  Node { x: 0, parent: Weak::default(), children: vec![] };
    //
    //     //let n1 = Arc::new(Node { x: 1, parent: weak.clone(), children: vec![] });
    //     let n1 = create_child(1, weak.clone());
    //     v.push(n1.clone());
    //     node.children.push(Arc::downgrade(&n1));
    //
    //     let n2 = create_child(2, weak.clone());
    //     v.push(n2.clone());
    //     node.children.push(Arc::downgrade(&n2));
    //
    //     let n3 = create_child(3, weak.clone());
    //     v.push(n3.clone());
    //     node.children.push(Arc::downgrade(&n3));
    //
    //     Arc::get_mut(&mut existing_children[0]).unwrap().parent = weak.clone();
    //     v.push(existing_children[0].clone());
    //     node.children.push(Arc::downgrade(&existing_children[0]));
    //
    //     //Arc::get_mut(&mut sn).unwrap().parent = weak.clone();
    //     //node.children.push(Arc::downgrade(&sn));
    //
    //     node
    // });
    v.push(n0);

    // // the parent:
    // let mut n0 = Arc::new(Node::new(0));
    //
    // // the children of n0:
    // let n1 = Arc::new_cyclic(|weak| {
    //     Arc::get_mut(&mut n0).unwrap().children.push(weak.clone());
    //     Node { x: 1, parent: Arc::downgrade(&n0), children: vec![] }
    // });
    //
    // // can't create a second child :(
    // let n2 = Arc::new_cyclic(|weak| {
    //     Arc::get_mut(&mut n0).unwrap().children.push(weak.clone());
    //     Node { x: 2, parent: Arc::downgrade(&n0), children: vec![] }
    // });

    //v.push(n0);
    //v.push(n1);
    //v.push(n2);

    let arc_v = Arc::new(v); // Wrap the vector in an Arc for shared ownership

    thread::scope(|scope| {
        for i in 0..arc_v.len() {
            let arc_v_clone = Arc::clone(&arc_v); // Clone the Arc, not the data itself
            scope.spawn(move || {
                let node = &arc_v_clone[i];
                println!("{:?}", node);
                if let Some(parent) = node.parent.upgrade() {
                    println!("x {}, parent {:?}", node.x, parent.x);
                }
                for child in node.children.iter() {
                    println!("x {}, child {}", node.x, child.upgrade().unwrap().x);
                }
            });
        }
    });
}
