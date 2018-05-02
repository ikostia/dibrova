from __future__ import print_function
from __future__ import absolute_import

class ProgrammerError(Exception):
    pass

LEFT = 0
RIGHT = 1

def other_direction(d):
    return 1 - d

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

    def child(self, direction):
        """Return node's child on given direction"""
        if direction == LEFT:
            return self.left_child
        else:
            return self.right_child

    def ischild(self, direction):
        """Check if the node is a child on given direction"""
        if self.parent is None:
            return False
        return self == self.parent.child(direction)

    def direction_from_parent(self):
        """Return the direction this node is on from its parent"""
        if self.parent is None:
            raise ProgrammerError("Root does not have a direction from parent")
        if self.ischild(LEFT):
            return LEFT
        return RIGHT

    def set_child(self, direction, node):
        """Set node's child in a given direction"""
        if direction is not LEFT and direction is not RIGHT:
            raise ProgrammerError("direction must be either LEFT or RIGHT")
        if direction == LEFT:
            self.left_child = node
        else:
            self.right_child = node

    def sibling(self):
        """Return node's sibling if present or None otherwise"""
        if self.isroot():
            raise ProgrammerError("root does not have siblings")
        return self.parent.child(other_direction(self.direction_from_parent()))

    def isroot(self):
        """Check if the node is the root of the BST"""
        return self.parent is None

    def isleaf(self):
        """"Check if the node is a leaf of the BST"""
        return self.left_child is None and self.right_child is None

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

    def replace_subtree(self, node):
        """Replace the subtree rooted in self with a different subtree"""
        if node is not None:
            node.parent = self.parent
        if self.isroot():
            # can't really do anything as there's no parent
            return
        self.parent.set_child(self.direction_from_parent(), node)

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
        while node.ischild(RIGHT):
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
        if node.ischild(LEFT) and node.sibling() is not None:
            self.curr = self._start_in_subtree(node.sibling())
        else:
            self.curr = node.parent

class BST(object):
    NodeClass = BstNode

    """Basic binary search tree"""
    def __init__(self, root=None):
        assert issubclass(self.NodeClass, BstNode)
        self.root = root

    def insert(self, data):
        """Naive implementation of BST insert"""
        if self.root is None:
            self.root = self.NodeClass(data=data)
            return self.root

        parent = None
        node = self.root
        while node != None:
            parent = node
            node = node.left_child if node.data > data else node.right_child
        node = self.NodeClass(data=data, parent=parent)
        if node.data < parent.data:
            parent.left_child = node
        else:
            parent.right_child = node
        # newly inserted node is always a leaf,
        # so we can just fix augmentations as we go up
        node.fix_augmentations(propagate=True)
        return node

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
        if not root_is_absolute:
            root_direction_from_parent = root.direction_from_parent()
        parent = root.parent
        opposite_direction = other_direction(direction)
        pivot = root.child(opposite_direction)

        if pivot is None:
            raise ProgrammerError("Rotation pivot cannot be None")

        root.parent = pivot
        root.set_child(opposite_direction, pivot.child(direction))
        pivot.set_child(direction, root)
        if root.child(opposite_direction) is not None:
            root.child(opposite_direction).parent = root

        pivot.parent = parent
        if root_is_absolute:
            self.root = pivot
        else:
            parent.set_child(root_direction_from_parent, pivot)

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

class AvlNode(BstNode):
    def avl_balance(self):
        if self.isleaf():
            return 0
        if self.left_child is None:
            return self.right_child.height
        if self.right_child is None:
            return -self.left_child.height
        return self.right_child.height - self.left_child.height

    def heavy(self, direction):
        if self.avl_balance() < 0 and direction == LEFT:
            return True
        elif self.avl_balance() > 0 and direction == RIGHT:
            return True
        return False

    def left_heavy(self):
        return self.avl_balance() < 0

    def right_heavy(self):
        return self.avl_balance() > 0

class AvlBst(BST):
    NodeClass = AvlNode

    def rebalance(self, node):
        while node is not None and -1 <= node.avl_balance() <= 1:
            node = node.parent
        if node is None:
            return
        dir1 = LEFT if node.heavy(LEFT) else RIGHT
        dir2 = other_direction(dir1)
        child = node.child(dir1)

        if not child.heavy(dir2):
            self.rotate(node, dir2)
            node.fix_augmentations()
        else:
            child.rotate(dir1)
            child.fix_augmentations()
            node.rotate(dir2)
            node.fix_augmentations() 

    def insert(self, data):
        inserted = super(AvlBst, self).insert(data)
        self.rebalance(inserted)
        return inserted