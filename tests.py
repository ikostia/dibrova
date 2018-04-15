from __future__ import print_function
from __future__ import absolute_import

import unittest

import bst as bstmod

class TestBasicBST(unittest.TestCase):
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
        bst.rotate(bst.root, bst.RIGHT)
        # rotation does not change in-order traversals
        self.ale(bst, [1, 3, 4, 7, 10])
        self.ale(bst.iter_preorder(), [3, 1, 7, 4, 10])
        self.ale(bst.iter_postorder(), [1, 4, 10, 7, 3])
        bst.rotate(bst.root, bst.LEFT)
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
        bst.rotate(bst.root.right_child, bst.RIGHT)
        self.ale(bst, [0, 1, 3, 4, 7, 10])
        self.ale(bst.iter_preorder(), [0, 3, 1, 7, 4, 10])
        self.ale(bst.iter_postorder(), [1, 4, 10, 7, 3, 0])
        bst.rotate(bst.root.right_child, bst.LEFT)
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
        bst.rotate(bst.root.right_child, bst.RIGHT)
        self.ale(bst, [0, 1, 3, 4, 7])
        self.ale(bst.iter_preorder(), [0, 3, 1, 7, 4])
        self.ale(bst.iter_postorder(), [1, 4, 7, 3, 0])
        bst.rotate(bst.root.right_child, bst.LEFT)
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
        bst.rotate(bst.root.right_child, bst.RIGHT)
        self.ale(bst, [0, 1, 3, 7])
        self.ale(bst.iter_preorder(), [0, 3, 1, 7])
        self.ale(bst.iter_postorder(), [1, 7, 3, 0])
        bst.rotate(bst.root.right_child, bst.LEFT)
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

if __name__ == "__main__":
    unittest.main()