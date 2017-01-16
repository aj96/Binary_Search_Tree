#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}

use std::cmp::Ordering;
use std::ops::Deref;
use std::mem;
#[derive(PartialEq)]
#[derive(Ord)]
#[derive(PartialOrd)]
#[derive(Eq,Clone,Debug)]


pub struct TreeNode<T> {
    pub key: T,
    pub leftChild: Option<Box<TreeNode<T>>>,
    pub rightChild: Option<Box<TreeNode<T>>>,
}



impl<T: Ord> TreeNode<T> {
        
    pub fn insert(&mut self, key: T) -> bool {  
        if key == self.key {        
            return false;
        }
        else if key < self.key {
            match self.leftChild {
                None => {
                    let treenode = Box::new(TreeNode{key:key, leftChild:None, rightChild:None});
                    self.leftChild = Some(treenode);
                    return true;
                },
                Some(ref mut x) => {
                   let return_value = x.insert(key);
                   if return_value == false {
                       return false;
                   }
                   else {
                       return true;
                   }
                },
            }
        }

        else {
            match self.rightChild {
                None => {
                    let treenode = Box::new(TreeNode{key:key, leftChild:None, rightChild:None});
                    self.rightChild = Some(treenode);
                    return true;
                },
                Some(ref mut x) => {
                   let return_value = x.insert(key);
                   if return_value == false {
                       return false;
                   }
                   else {
                       return true;
                   }
                },
            }
        }
        

    
    }

    pub fn find(&self, key: &T) -> bool {
        if *key == self.key {
            return true;
        }
        else if *key < self.key {
            match self.leftChild {
                None => {
                    return false;
                },
                Some(ref x) => {
                   let return_value = x.find(key);
                   if return_value == false {
                       return false;
                   }
                   else {
                       return true;
                   }
                },
            }
        }

        else {
            match self.rightChild {
                None => {
                    return false;
                },
                Some(ref x) => {
                   let return_value = x.find(key);
                   if return_value == false {
                       return false;
                   }
                   else {
                       return true;
                   }
                },
            }
        }


    }

    pub fn preorder<'a>(&'a self, vector: &mut Vec<&'a T>) {
        vector.push(&self.key);
        match self.leftChild {
            None => {},
            Some(ref x) => {
                x.preorder(vector);
            },
        }
        match self.rightChild {
            None => {},
            Some(ref x) => {
                x.preorder(vector);
            },
        }
        
    }

    pub fn inorder<'a>(&'a self, vector: &mut Vec<&'a T>) {
        match self.leftChild {
            None => {},
            Some(ref x) => {
                x.inorder(vector);
            },
        }
        
        vector.push(&self.key);
        
        match self.rightChild {
            None => {},
            Some(ref x) => {
                x.inorder(vector);
            },
        }
        
    }

    pub fn postorder<'a>(&'a self, vector: &mut Vec<&'a T>) {
        match self.leftChild {
            None => {},
            Some(ref x) => {
                x.postorder(vector);
            },
        }

        match self.rightChild {
            None => {},
            Some(ref x) => {
                x.postorder(vector);
            },
        }
        
        vector.push(&self.key);
        
        
        
    }

}
pub struct Tree<T> {
    pub root: Option<Box<TreeNode<T>>>,
}

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        Tree{root: None}
    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {  
      
        match self.root {
            None => {
                let root_treenode = Box::new(TreeNode {key: key, leftChild: None, rightChild: None});
                self.root = Some(root_treenode);
                return true;
            },
            
            Some(ref mut x) =>  {
                
                let return_value = x.insert(key); 
                if return_value == false {
                    return false;
                }
                else {
                    return true;
                }
                      
            },
        }
    }

    // Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.root {
            None => {
                return false;
            },
            
            Some(ref x) =>  {
                
                let return_value = x.find(key); 
                if return_value == false {
                    return false;
                }
                else {
                    return true;
                }
                      
            },
        }
    }

    // Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
       let mut return_vector: Vec<&T> = Vec::new();
       match self.root {
           None => {
               return return_vector;
           },
           Some(ref x) => {
               x.preorder(&mut return_vector);
        
               return return_vector;
           },
       }

       
    }

    // Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut return_vector: Vec<&T> = Vec::new();
        match self.root {
            None => {
                return return_vector;
            },
            Some(ref x) => {
                x.inorder(&mut return_vector);
         
                return return_vector;
            },
        }
    }

    // Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut return_vector: Vec<&T> = Vec::new();
        match self.root {
            None => {
                return return_vector;
            },
            Some(ref x) => {
                x.postorder(&mut return_vector);
         
                return return_vector;
            },
        }
    }
}