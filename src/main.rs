use std::sync::{Arc, Weak};
use std::thread;

#[derive(Debug, Default)]
struct Node {
    x: i32,
    parent: Weak<Node>,
    children: Vec<Weak<Node>>,
}

impl Node {
    // fn new(x: i32) -> Self {
    //     Self { x, .. Self::default() }
    // }
}

fn create_child(x: i32, parent: Weak<Node>) -> Arc<Node> {
    Arc::new(Node { x: x, parent: parent, children: vec![] })
}

fn create_parent(x: i32) -> Arc<Node> {

}

fn main() {
    let mut v: Vec<Arc<Node>> = vec![];

    // Can we have children first? In a separate vec:
    let mut existing_children = vec![
        Arc::new(Node {x: 10, parent: Weak::default(), children: vec![] }),
    ];

    // How about in the same vec? Nope...
    //v.push(Arc::new(Node {x: 20, parent: Weak::default(), children: vec![] }));
    //let mut sn = &mut v[0];

    // Parent:
    let n0 = Arc::new_cyclic(|weak| {
        let mut node =  Node { x: 0, parent: Weak::default(), children: vec![] };

        //let n1 = Arc::new(Node { x: 1, parent: weak.clone(), children: vec![] });
        let n1 = create_child(1, weak.clone());
        v.push(n1.clone());
        node.children.push(Arc::downgrade(&n1));

        let n2 = create_child(1, weak.clone());
        v.push(n2.clone());
        node.children.push(Arc::downgrade(&n2));

        let n3 = create_child(3, weak.clone());
        v.push(n3.clone());
        node.children.push(Arc::downgrade(&n3));

        Arc::get_mut(&mut existing_children[0]).unwrap().parent = weak.clone();
        v.push(existing_children[0].clone());
        node.children.push(Arc::downgrade(&existing_children[0]));

        //Arc::get_mut(&mut sn).unwrap().parent = weak.clone();
        //node.children.push(Arc::downgrade(&sn));

        node
    });
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
