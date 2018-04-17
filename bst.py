from __future__ import print_function
from __future__ import absolute_import

class ProgrammerError(Exception):
    pass

class BstNode(object):
    """Binary search tree node"""

    def __init__(self, data=None, parent=None):
        self.data = data
        self.parent = parent
        self.left_child = None
        self.right_child = None
        # the longest path from this node to any leaf
        # in a subtree rooted at this node
        self.height = 1
        # the total number of nodes in a subtree
        # rooted at this node
        self.weight = 1

    def fix_augmentations(self, propagate=True):
        heights = [
            (self.left_child is not None) and self.left_child.height or 0,
            (self.right_child is not None) and self.right_child.height or 0,
        ]
        weights  = [
            (self.left_child is not None) and self.left_child.weight or 0,
            (self.right_child is not None) and self.right_child.weight or 0,
        ]
        self.height = max(heights) + 1
        self.weight = sum(weights) + 1
        if propagate and self.parent is not None:
            self.parent.fix_augmentations(propagate=True)

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

    def isleaf(self):
        """"Check if the node is a leaf of the BST"""
        return self.left_child is None and self.right_child is None

    def replace_subtree(self, node):
        """Replace the subtree rooted in self with a different subtree"""
        if node is not None:
            node.parent = self.parent
        if self.isroot():
            # can't really do anything as there's no parent
            return
        if self.isleftchild():
            self.parent.left_child = node
        else:
            self.parent.right_child = node

    def replace_node(self, node):
        """Replace just the current node with a given node"""
        node.left_child = self.left_child
        node.right_child = self.right_child
        if node.left_child is not None:
            node.left_child.parent = node
        if node.right_child is not None:
            node.right_child.parent = node
        self.replace_subtree(node)

    def min(self):
        """Return a leftmost node of a subtree, rooted at self"""
        node = self
        while node.left_child is not None:
            node = node.left_child
        return node

    def max(self):
        """Return a rightmost node of a subtree, rooted at self"""
        node = self
        while node.right_child is not None:
            node = node.right_child
        return node

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
    LEFT = 0
    RIGHT = 1

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
        # newly inserted node is always a leaf,
        # so we can just fix augmentations as we go up
        node.fix_augmentations(propagate=True)

    def delete(self, node):
        if node.isleaf():
            if node == self.root:
                # both a leaf and a root, must be the only node of a tree
                self.root = None
                return
            parent = node.parent
            node.replace_subtree(None)
            parent.fix_augmentations(propagate=True)
            return

        replace_node = False
        to_fix_augmentations = None
        if node.left_child is None:
            replacement = node.right_child
        elif node.right_child is None:
            replacement = node.left_child
        else:
            # complex case: node with both children
            replacement = node.right_child.min()
            if replacement.parent is not node:
                to_fix_augmentations = replacement.parent
            replacement_parent = replacement.parent
            self.delete(replacement)
            replace_node = True

        if replace_node:
            node.replace_node(replacement)
        else:
            node.replace_subtree(replacement)
        
        if to_fix_augmentations is None:
            to_fix_augmentations = replacement
        to_fix_augmentations.fix_augmentations(propagate=True)

        if node == self.root:
            self.root = replacement

    def find(self, value):
        parent = None
        node = self.root
        while node is not None:
            if node.data == value:
                break
            if node.data < value and node.right_child is not None:
                node = node.right_child
                continue
            if node.data > value and node.left_child is not None:
                node = node.left_child
                continue
            node = None
        return node

    def rotate(self, root, direction):
        """Rotate a binary tree around the node in a given direction"""
        root_is_absolute = root.isroot()
        root_is_left_child = root.isleftchild()
        parent = root.parent
        if direction == self.RIGHT:
            pivot = root.left_child
        else:
            pivot = root.right_child

        if pivot is None:
            raise ProgrammerError("Rotation pivot cannot be None")

        root.parent = pivot
        if direction == self.RIGHT:
            root.left_child = pivot.right_child
            pivot.right_child = root
            if root.left_child is not None:
                root.left_child.parent = root
        else:
            root.right_child = pivot.left_child
            pivot.left_child = root
            if root.right_child is not None:
                root.right_child.parent = root

        pivot.parent = parent
        if root_is_absolute:
            self.root = pivot
        elif root_is_left_child:
            parent.left_child = pivot
        else:
            parent.right_child = pivot

    def min(self):
        """Get the smallest element in the BST"""
        if self.root is None:
            return None
        return self.root.min()

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