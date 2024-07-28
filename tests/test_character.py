import unittest
from virtual_character.scripts.character_control import Character

class TestCharacter(unittest.TestCase):
    def setUp(self):
        self.character = Character("virtual_character/assets/character.png", [100, 100])

    def test_move_left(self):
        self.character.move("left")
        self.assertEqual(self.character.position, [95, 100])

    def test_move_right(self):
        self.character.move("right")
        self.assertEqual(self.character.position, [105, 100])

    def test_move_up(self):
        self.character.move("up")
        self.assertEqual(self.character.position, [100, 95])

    def test_move_down(self):
        self.character.move("down")
        self.assertEqual(self.character.position, [100, 105])

if __name__ == '__main__':
    unittest.main()
