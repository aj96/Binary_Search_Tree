mod lib;
use lib::Tree;
use lib::TreeNode;
// Setting up a Binary Search Tree to Test
pub fn test1() -> Tree<i32> {
    let TreeNode_20 = Box::new(TreeNode {key:20,leftChild:None,rightChild:None});
    let TreeNode_35 = Box::new(TreeNode {key:35,leftChild:None,rightChild:None});
    let TreeNode_42 = Box::new(TreeNode {key:42,leftChild:None,rightChild:None});
    let TreeNode_47 = Box::new(TreeNode {key:47,leftChild:None,rightChild:None});
    let TreeNode_52 = Box::new(TreeNode {key:52,leftChild:None,rightChild:None});
    let TreeNode_57 = Box::new(TreeNode {key:57,leftChild:None,rightChild:None});
    let TreeNode_65 = Box::new(TreeNode {key:65,leftChild:None,rightChild:None});
    let TreeNode_80 = Box::new(TreeNode {key:80,leftChild:None,rightChild:None});

    let TreeNode_30 = Box::new(TreeNode {key:30,leftChild:Some(TreeNode_20),rightChild:Some(TreeNode_35)});
    let TreeNode_45 = Box::new(TreeNode {key:45,leftChild:Some(TreeNode_42),rightChild:Some(TreeNode_47)});
    let TreeNode_55 = Box::new(TreeNode {key:55,leftChild:Some(TreeNode_52),rightChild:Some(TreeNode_57)});
    let TreeNode_70 = Box::new(TreeNode {key:70,leftChild:Some(TreeNode_65),rightChild:Some(TreeNode_80)});

    let TreeNode_40 = Box::new(TreeNode {key:40,leftChild:Some(TreeNode_30),rightChild:Some(TreeNode_45)});
    let TreeNode_60 = Box::new(TreeNode {key:60,leftChild:Some(TreeNode_55),rightChild:Some(TreeNode_70)});


    let root_TreeNode = Box::new(TreeNode {key:50, leftChild : Some(TreeNode_40), rightChild : Some(TreeNode_60)});
    let mut myTree = Tree {root: Some(root_TreeNode)};

    return myTree;
}

pub fn vectorEq(v1: &Vec<i32>, v2: &Vec<&i32>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    let mut i: usize = 0;
    while i < v1.len() {
        //println!("v1[{}]: {}, v2[{}]: {}\n",i,v1[i],i,*v2[i]);
        if v1[i] != *v2[i] {
            return false;
        }
        i += 1;
    }
    return true;
}

fn main() {
    
    let mut myTree = test1();
    assert_eq!(false, myTree.insert(20)); // 20 already exists in tree, so insert() should not insert and return false
    assert_eq!(true,myTree.insert(16)); // 16 is not in tree, so insert() should insert and return true
    assert_eq!(true, myTree.find(&55)); // 55 is in tree, so find() should return true
    assert_eq!(false, myTree.find(&100)); // 100 is not in tree, so find() should return false

    let postorderVector = vec![16,20,35,30,42,47,45,40,52,57,55,65,80,70,60,50];
    let mut vector = myTree.postorder();
    assert_eq!(true, vectorEq(&postorderVector, &vector)); // checking if postorder is correct
    
    let inorderVector = vec![16,20,30,35,40,42,45,47,50,52,55,57,60,65,70,80];
    vector = myTree.inorder();
    assert_eq!(true, vectorEq(&inorderVector, &vector));

    let preorderVector = vec![50,40,30,20,16,35,45,42,47,60,55,52,57,70,65,80];
    vector = myTree.preorder();
    assert_eq!(true, vectorEq(&preorderVector, &vector));
    

  
    

    
}
