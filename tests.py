from __future__ import print_function
from __future__ import absolute_import

import random
import unittest

import bst as bstmod

class TestBasicBST(unittest.TestCase):
    def test_node_param(self):
        bst = bstmod.BST()
        bst.insert(7)
        self.assertIsInstance(bst.root, bstmod.BstNode)
        class NewNode(bstmod.BstNode):
            pass
        class NewBst(bstmod.BST):
            NodeClass = NewNode
        bst = NewBst()
        bst.insert(7)
        self.assertIsInstance(bst.root, NewNode)
        bst.insert(8)
        bst.delete(bst.root)
        bst.delete(bst.root)
        self.assertIsNone(bst.root)

    def test_insert(self):
        bst = bstmod.BST()
        self.assertIsNone(bst.root)
        bst.insert(8)
        self.assertEqual(bst.root.data, 8)
        bst.insert(4)
        self.assertEqual(bst.root.left_child.data, 4)
        self.assertIsNone(bst.root.right_child)
        bst.insert(6)
        self.assertEqual(bst.root.left_child.right_child.data, 6)
        bst.insert(12)
        self.assertEqual(bst.root.right_child.data, 12)

    def build(self, lst):
        bst = bstmod.BST()
        for item in lst:
            bst.insert(item)
        return bst
    
    def ale(self, tolist, lst):
        self.assertListEqual(list(tolist), lst)

    def test_inorder(self):
        # in-order is the default traversal
        self.assertListEqual(list(self.build([1, 2, 3])), [1, 2, 3])

        def ale(lst1, lst2):
            self.assertListEqual(list(self.build(lst1).iter_inorder()), lst2)
        for lst in [[0], [2, 1, 3], [1, 1, 2], [1, 2], [2, 1], [1, 1],
                    [8, 4, 6, 12, 10, 13, 9], []]:
            ale(lst, sorted(lst))

    def test_preorder(self):
        cases = [
            ([], []),
            ([1], [1]),
            ([1,2], [1, 2]),
            ([2, 1], [2, 1]),
            ([2, 1, 3], [2, 1, 3]),
            ([2, 3, 1], [2, 1, 3]),
            ([12, 17, 8, 11, 13, 20, 6, 9, 15],
             [12, 8, 6, 11, 9, 17, 13, 15, 20]),
        ]
        for totree, order in cases:
            self.ale(self.build(totree).iter_preorder(), order)

    def test_postorder(self):
        cases = [
            ([], []),
            ([1], [1]),
            ([1, 2], [2, 1]),
            ([2, 1, 3], [1, 3, 2]),
            ([2, 3, 1], [1, 3, 2]),
            ([12, 17, 8, 11, 13, 20, 6, 7, 9, 15],
             [7, 6, 9, 11, 8, 15,13, 20, 17, 12]),
        ]
        for totree, order in cases:
            self.ale(self.build(totree).iter_postorder(), order)

    def test_relationships(self):
        #      7
        #     / \
        #    3   10
        bst = self.build([7, 3, 10])
        rt, l, r = bst.root, bst.root.left_child, bst.root.right_child
        self.assertRaises(bstmod.ProgrammerError, rt.direction_from_parent)
        self.assertRaises(bstmod.ProgrammerError, rt.sibling)
        self.assertEqual(rt.ischild(bstmod.LEFT), False)
        self.assertEqual(rt.ischild(bstmod.RIGHT), False)
        self.assertEqual(rt.left_child, rt.child(bstmod.LEFT))
        self.assertEqual(rt.right_child, rt.child(bstmod.RIGHT))
        self.assertEqual(l.sibling(), r)
        self.assertEqual(l.direction_from_parent(), bstmod.LEFT)
        self.assertEqual(r.direction_from_parent(), bstmod.RIGHT)
        rt.set_child(bstmod.RIGHT, l)
        self.assertEqual(rt.child(bstmod.RIGHT), l)

    def test_rotations_simple(self):
        #      7           3
        #     / \         / \
        #    3   10  =>  1   7
        #   / \             / \
        #  1   4           4   10
        bst = self.build([7, 3, 10, 1, 4])
        self.ale(bst, [1, 3, 4, 7, 10])
        self.ale(bst.iter_preorder(), [7, 3, 1, 4, 10])
        self.ale(bst.iter_postorder(), [1, 4, 3, 10, 7])
        bst.rotate(bst.root, bstmod.RIGHT)
        # rotation does not change in-order traversals
        self.ale(bst, [1, 3, 4, 7, 10])
        self.ale(bst.iter_preorder(), [3, 1, 7, 4, 10])
        self.ale(bst.iter_postorder(), [1, 4, 10, 7, 3])
        bst.rotate(bst.root, bstmod.LEFT)
        # mirrorring rotations produce the same tree
        self.ale(bst, [1, 3, 4, 7, 10])
        self.ale(bst.iter_preorder(), [7, 3, 1, 4, 10])
        self.ale(bst.iter_postorder(), [1, 4, 3, 10, 7])

    def test_rotations_complex(self):
        #    0           0
        #     \           \
        #      7           3
        #     / \         / \
        #    3   10  =>  1   7
        #   / \             / \
        #  1   4           4   10
        bst = self.build([0, 7, 3, 10, 1, 4])
        bst.rotate(bst.root.right_child, bstmod.RIGHT)
        self.ale(bst, [0, 1, 3, 4, 7, 10])
        self.ale(bst.iter_preorder(), [0, 3, 1, 7, 4, 10])
        self.ale(bst.iter_postorder(), [1, 4, 10, 7, 3, 0])
        bst.rotate(bst.root.right_child, bstmod.LEFT)
        self.ale(bst, [0, 1, 3, 4, 7, 10])
        self.ale(bst.iter_preorder(), [0, 7, 3, 1, 4, 10])
        self.ale(bst.iter_postorder(), [1, 4, 3, 10, 7, 0])

        #    0           0
        #     \           \
        #      7           3
        #     /           / \
        #    3     =>    1   7
        #   / \             /
        #  1   4           4
        bst = self.build([0, 7, 3, 1, 4])
        bst.rotate(bst.root.right_child, bstmod.RIGHT)
        self.ale(bst, [0, 1, 3, 4, 7])
        self.ale(bst.iter_preorder(), [0, 3, 1, 7, 4])
        self.ale(bst.iter_postorder(), [1, 4, 7, 3, 0])
        bst.rotate(bst.root.right_child, bstmod.LEFT)
        self.ale(bst, [0, 1, 3, 4, 7])
        self.ale(bst.iter_preorder(), [0, 7, 3, 1, 4])
        self.ale(bst.iter_postorder(), [1, 4, 3, 7, 0])

        #    0           0
        #     \           \
        #      7           3
        #     /           / \
        #    3     =>    1   7
        #   /
        #  1
        bst = self.build([0, 7, 3, 1])
        bst.rotate(bst.root.right_child, bstmod.RIGHT)
        self.ale(bst, [0, 1, 3, 7])
        self.ale(bst.iter_preorder(), [0, 3, 1, 7])
        self.ale(bst.iter_postorder(), [1, 7, 3, 0])
        bst.rotate(bst.root.right_child, bstmod.LEFT)
        self.ale(bst, [0, 1, 3, 7])
        self.ale(bst.iter_preorder(), [0, 7, 3, 1])
        self.ale(bst.iter_postorder(), [1, 3, 7, 0])

    def test_delete(self):
        #      7
        #     / \
        #    3   10
        #   / \
        #  1   4
        bst = self.build([7, 3, 10, 1, 4])
        self.ale(bst, [1, 3, 4, 7, 10])
        self.ale(bst.iter_preorder(), [7, 3, 1, 4, 10])
        self.ale(bst.iter_postorder(), [1, 4, 3, 10, 7])
        bst.delete(bst.root)
        #     10
        #    /
        #   3
        #  / \
        # 1   4
        self.ale(bst, [1, 3, 4, 10])
        self.ale(bst.iter_preorder(), [10, 3, 1, 4])
        self.ale(bst.iter_postorder(), [1, 4, 3, 10])
        bst.delete(bst.root.left_child)
        #     10
        #    /
        #   4
        #  /
        # 1
        self.ale(bst, [1, 4, 10])
        self.ale(bst.iter_preorder(), [10, 4, 1])
        self.ale(bst.iter_postorder(), [1, 4, 10])
        bst.delete(bst.root.left_child.left_child)
        #   10
        #  /
        # 4
        self.ale(bst, [4, 10])
        self.ale(bst.iter_preorder(), [10, 4])
        self.ale(bst.iter_postorder(), [4, 10])
        bst.delete(bst.root.left_child)
        # 10
        self.ale(bst, [10])
        self.ale(bst.iter_preorder(), [10])
        self.ale(bst.iter_postorder(), [10])
        bst.delete(bst.root)
        # empty tree
        self.ale(bst, [])
        self.ale(bst.iter_preorder(), [])
        self.ale(bst.iter_postorder(), [])

        #      7
        #     / \
        #    3   10
        #   / \
        #  1   4
        bst = self.build([7, 3, 10, 1, 4])
        bst.delete(bst.root.left_child)
        #      7
        #     / \
        #    4   10
        #   /
        #  1
        self.ale(bst, [1, 4, 7, 10])
        self.ale(bst.iter_preorder(), [7, 4, 1, 10])
        self.ale(bst.iter_postorder(), [1, 4, 10, 7])

    def test_find(self):
        cases = [
            [],
            [1],
            [10, 2],
            [7, 3, 10, 1, 4]
        ]
        for case in cases:
            bst = self.build(case)
            for item in case:
                res = bst.find(item)
                self.assertEqual(res.data, item)
                self.assertIsInstance(res, bstmod.BstNode)
            if case:
                self.assertIsNone(bst.find(max(case)+1))
            else:
                self.assertIsNone(bst.find(1))

    def test_augmentations_on_insert(self):
        bst = bstmod.BST()
        bst.insert(7)
        # 7
        self.assertEqual(bst.root.height, 1)
        self.assertEqual(bst.root.weight, 1)
        bst.insert(3)
        #   7
        #  /
        # 3
        self.assertEqual(bst.root.height, 2)
        self.assertEqual(bst.root.weight, 2)
        self.assertEqual(bst.root.left_child.height, 1)
        self.assertEqual(bst.root.left_child.weight, 1)
        bst.insert(10)
        #   7
        #  / \
        # 3   10
        self.assertEqual(bst.root.height, 2)
        self.assertEqual(bst.root.weight, 3)
        self.assertEqual(bst.root.left_child.height, 1)
        self.assertEqual(bst.root.left_child.weight, 1)
        self.assertEqual(bst.root.right_child.height, 1)
        self.assertEqual(bst.root.right_child.weight, 1)
        bst.insert(1)
        #      7
        #     / \
        #    3   10
        #   /
        #  1
        self.assertEqual(bst.root.height, 3)
        self.assertEqual(bst.root.weight, 4)
        self.assertEqual(bst.root.left_child.height, 2)
        self.assertEqual(bst.root.left_child.weight, 2)
        self.assertEqual(bst.root.left_child.left_child.height, 1)
        self.assertEqual(bst.root.left_child.left_child.weight, 1)
        self.assertEqual(bst.root.right_child.height, 1)
        self.assertEqual(bst.root.right_child.weight, 1)
        bst.insert(4)
        #      7
        #     / \
        #    3   10
        #   / \
        #  1   4
        self.assertEqual(bst.root.height, 3)
        self.assertEqual(bst.root.weight, 5)
        self.assertEqual(bst.root.left_child.height, 2)
        self.assertEqual(bst.root.left_child.weight, 3)
        self.assertEqual(bst.root.left_child.left_child.height, 1)
        self.assertEqual(bst.root.left_child.left_child.weight, 1)
        self.assertEqual(bst.root.left_child.right_child.height, 1)
        self.assertEqual(bst.root.left_child.right_child.weight, 1)
        self.assertEqual(bst.root.right_child.height, 1)
        self.assertEqual(bst.root.right_child.weight, 1)

    def test_augmentations_on_delete(self):
        #      7
        #     / \
        #    3   10
        #   / \
        #  1   4
        bst = self.build([7, 3, 10, 1, 4])
        self.assertEqual(bst.root.height, 3)
        self.assertEqual(bst.root.weight, 5)
        self.assertEqual(bst.root.left_child.height, 2)
        self.assertEqual(bst.root.left_child.weight, 3)
        self.assertEqual(bst.root.right_child.height, 1)
        self.assertEqual(bst.root.right_child.weight, 1)
        self.assertEqual(bst.root.left_child.right_child.height, 1)
        self.assertEqual(bst.root.left_child.right_child.weight, 1)

        bst.delete(bst.root)
        #     10
        #    /
        #   3
        #  / \
        # 1   4
        self.assertEqual(bst.root.height, 3)
        self.assertEqual(bst.root.weight, 4)
        self.assertEqual(bst.root.left_child.height, 2)
        self.assertEqual(bst.root.left_child.weight, 3)
        self.assertEqual(bst.root.left_child.right_child.height, 1)
        self.assertEqual(bst.root.left_child.right_child.weight, 1)

        bst.delete(bst.root.left_child)
        #     10
        #    /
        #   4
        #  /
        # 1
        self.assertEqual(bst.root.height, 3)
        self.assertEqual(bst.root.weight, 3)
        self.assertEqual(bst.root.left_child.height, 2)
        self.assertEqual(bst.root.left_child.weight, 2)
        self.assertEqual(bst.root.left_child.left_child.height, 1)
        self.assertEqual(bst.root.left_child.left_child.weight, 1)

        bst.delete(bst.root.left_child.left_child)
        #   10
        #  /
        # 4
        self.assertEqual(bst.root.height, 2)
        self.assertEqual(bst.root.weight, 2)
        self.assertEqual(bst.root.left_child.height, 1)
        self.assertEqual(bst.root.left_child.weight, 1)

        bst.delete(bst.root.left_child)
        # 10
        self.assertEqual(bst.root.height, 1)
        self.assertEqual(bst.root.weight, 1)

        #      7
        #     / \
        #    3   10
        #   / \
        #  1   4
        bst = self.build([7, 3, 10, 1, 4])
        bst.delete(bst.root.left_child)
        #      7
        #     / \
        #    4   10
        #   /
        #  1
        self.assertEqual(bst.root.height, 3)
        self.assertEqual(bst.root.weight, 4)
        self.assertEqual(bst.root.left_child.height, 2)
        self.assertEqual(bst.root.left_child.weight, 2)

    def check_augmentations(self, node):
        if node is None:
            return 0, 0
        left_aug = self.check_augmentations(node.left_child)
        right_aug = self.check_augmentations(node.right_child)
        height = 1 + max(left_aug[0], right_aug[0])
        weight = 1 + left_aug[1] + right_aug[1]
        self.assertEqual(node.height, height)
        self.assertEqual(node.weight, weight)
        return (height, weight)

    def test_augmentations_on_random(self):
        values = [random.randint(0, 1000) for i in xrange(500)]
        bst = bstmod.BST()
        for value in values:
            bst.insert(value)
            self.check_augmentations(bst.root)

        for value in values:
            bst.delete(bst.root)
            self.check_augmentations(bst.root)

    def check_bst_property(self, node):
        if node is None:
            return
        if node.right_child is not None:
            self.assertTrue(node.right_child.data >= node.data)
        if node.left_child is not None:
            self.assertTrue(node.left_child.data < node.data)
        self.check_bst_property(node.right_child)
        self.check_bst_property(node.left_child)

    def check_avl_property(self, node):
        if node is None:
            return
        self.assertTrue(-1 <= node.avl_balance() <= 1)
        self.check_avl_property(node.left_child)
        self.check_avl_property(node.right_child)

    def test_avl_insert(self):
        for seq in [[1, 2, 3, 4, 5, 6, 7, 8, 9],
                    [9, 8, 7, 6, 5, 4, 3, 2, 1],
                    [5, 4, 6, 3, 2, 7, 1, 9, 8]]:
            bst = bstmod.AvlBst()
            for item in seq:
                bst.insert(item)
                self.check_bst_property(bst.root)
                self.check_augmentations(bst.root)
                self.check_avl_property(bst.root)

if __name__ == "__main__":
    unittest.main()