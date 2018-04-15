from __future__ import print_function
from __future__ import absolute_import

class BstNode(object):
    """Binary search tree node"""

    def __init__(self, data=None, parent=None):
        self.data = data
        self.parent = parent
        self.left_child = None
        self.right_child = None
    
    def isleftchild(self):
        """Check if the node is the left child of its parent"""
        if self.parent is None:
            return False
        return self == self.parent.left_child

    def isrightchild(self):
        """Check if the node is the right child of its parent"""
        if self.parent is None:
            return False
        return self == self.parent.right_child

    def isroot(self):
        """Check if the node is the root of the BST"""
        return self.parent is None

    def __repr__(self):
        return '<%r>' % self.data

class BstIterator(object):
    """Generic BST iterator class"""

    def __init__(self, tree, return_nodes=False):
        self.start = True
        self.curr = None
        self.tree = tree
        self.return_nodes = return_nodes

    def __iter__(self):
        return self

    def next_impl(self):
        """Get next item for iterations >1
        Must change self.curr"""
        raise NotImplementedError("Method needs to be defined in real iterators")
    
    def first_impl(self):
        """Get first item in the iteration
        
        If this method is called, the tree is guaranteed to be not empty"""
        raise NotImplementedError("Method needs to be defined in real iterators")

    def next(self):
        if self.tree.root is None:
            raise StopIteration()
        if self.start:
            self.start = False
            self.first_impl()
        else:
            self.next_impl()
        return self.curr if self.return_nodes else self.curr.data

class BstInorderIterator(BstIterator):
    """BstIterator for the in-order traversal"""
    def first_impl(self):
        self.curr = self.tree.min()

    def next_impl(self):
        node = self.curr
        if node.right_child is not None:
            node = node.right_child
            while node.left_child is not None:
                node = node.left_child
            self.curr = node
            return
        # self.right_child is None
        while node.isrightchild():
            node = node.parent
        if node.isroot():
            raise StopIteration()
        # now node is guaranteed to be a left_child
        self.curr = node.parent

class BstPreorderIterator(BstIterator):
    """BstIterator for the pre-order traversal"""
    def first_impl(self):
        self.curr = self.tree.root

    def next_impl(self):
        node = self.curr
        if node.left_child is not None:
            self.curr = node.left_child
            return
        prev = None
        while node and (node.right_child is None or node.right_child is prev):
            prev = node
            node = node.parent
        if node is None:
            raise StopIteration()
        self.curr = node.right_child

class BstPostorderIterator(BstIterator):
    """BstIterator for post-order traversal"""
    def _start_in_subtree(self, node):
        while not(node.left_child is None and node.right_child is None):
            if node.left_child is not None:
                node = node.left_child
            else:
                node = node.right_child
        return node

    def first_impl(self):
        self.curr = self._start_in_subtree(self.tree.root)

    def next_impl(self):
        if self.curr.isroot():
            raise StopIteration()

        node = self.curr
        if node.isleftchild() and node.parent.right_child is not None:
            self.curr = self._start_in_subtree(node.parent.right_child)
        else:
            self.curr = node.parent

class BST(object):
    """Basic binary search tree"""
    def __init__(self, root=None):
        self.root = root

    def insert(self, data):
        """Naive implementation of BST insert"""
        if self.root is None:
            self.root = BstNode(data=data)
            return self.root

        parent = None
        node = self.root
        while node != None:
            parent = node
            node = node.left_child if node.data > data else node.right_child
        node = BstNode(data=data, parent=parent)
        if node.data < parent.data:
            parent.left_child = node
        else:
            parent.right_child = node

    def min(self):
        """Get the smallest element in the BST"""
        node = self.root
        while node.left_child is not None:
            node = node.left_child
        return node

    def iter_inorder(self):
        """Get the in-order traversal iterator for the BST"""
        return BstInorderIterator(self)
    
    def iter_preorder(self):
        """Get the pre-order traversal iterator for the BST"""
        return BstPreorderIterator(self)

    def iter_postorder(self):
        """Get the post-order traversal iterator for the BST"""
        return BstPostorderIterator(self)

    def __iter__(self):
        return self.iter_inorder()