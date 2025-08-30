use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

//A tree with nodes that know about their child nodes
//A Struct named node that holds its own i32 values as well as references to its children Node
//A node should own its children, and that ownership to be stored with variables so that each Node in the tree
//can be accessed directly
//To Modify nodes which are children of other nodes, there is  RefCell<T> in children around vec<Rc<Node>>

//Adding a reference from a Child to its parent
//To make the Child node aware of its parent, we need to add parent field to the Node struct
//The field cannot contain a Rc<T>because that creates a reference cycle with leaf.parent pointing to branch
//and branch.children pointing to leaf, which causes their strong_count values to never be zero
//A parent node should own its children; if a parent node is dropped its child node should be dropped as well
//However, a child should not own its parent, if a child node is dropped, the parent node should exist. This is a case for weak references

//so instead of Rc<T> we make the type of parent use Weak<T> specifically RefCell<Weak<Node>>
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    //a node will be able to refer to its parent node but does ot own the parent
    child: RefCell<Vec<Rc<Node>>>
}

fn main() {
    //A node instance named leaf with the value of 3 with no children
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![]),
    });
    println!("leaf strong ={}, weak = {}",Rc::strong_count(&leaf), Rc::weak_count(&leaf)
    );

    //A branch instance with the value of 5
    //We clone the Rc<Node> in leaf and store that in branch, meaning the node in leaf now has two owners;
    //leaf and branch
    //One can get from branch to leaf through branch.children but there is no way to
    //get from leaf to branch; The reason is that leaf has no reference to branch

    //when the branch node is created, it will also have a new Weak<Node> reference in the parent field
    //because branch does not have a parent node. We still have a leaf as one of the children of branch
    //Once we have a node instance in branch, the leaf can be modified  to give it a Weak<Node> reference to its parent
    //We use the borrow_mut method on the RefCell<Weak<Node>> in the parent field of the leaf and then
    //use the Rc::downgrade function to create a Weak<Node> reference to branch from the Rc<Node> in branch

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        child: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    println!("branch strong = {}, weak = {}", Rc::strong_count(&branch), Rc::weak_count(&branch),
    );
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf),
    );
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

   //trying to get a reference to the parent of the leaf by using the upgrade method,we get a None value
    //leaf parent = none
    //After leaf is created, its Rc<Node> has a strong count of 1 and a weak count of 0. In the inner scope we create branch
    //and associate it with leaf, at which point when count is printed, the Rc<Node> in branch will have a Strong count
    //of 1 and a weak count of 1(for leaf.parent pointing to branch with Weak<Node)
    //When the count in leaf is printed,  it has a strong count of 2 because branch now has a clone  of the Rc<Node> of leaf stored
    //in branch.children, but still a weak count of zero
    //When the inner scope ends , branch goes out of scope and the strong count of Rc<Node> decreases to zero
    //so the Node is dropped The Weak count of 1 from leaf.parent has no bearing on whether  or not the Node is dropped
    //so there are no memory leaks//If one tries to access the parent of leaf after the end of scope, we get None
    //At the end of the program,  the Rc<Node> in leaf has a strong count of 1 and a weak count of 0, because the variable leaf
    //is now the only reference to the Rc<Node> again
    //All of the logic that manages the counts and value dropping is built into Rc<T> and Weak<T> and their
    //implementations of the Drop trait
    //By specifying that the relationship from a child to a parent should be a Weak<T> reference in the definition of Node, you are able to point
    //to the child node through the parent node without creating a reference cycle and memory leaks

}


