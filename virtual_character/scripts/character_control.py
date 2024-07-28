import pygame

class Character:
    def __init__(self, image_path, position):
        self.image = pygame.image.load(image_path)
        self.position = position

    def move(self, direction):
        if direction == "left":
            self.position[0] -= 5
        elif direction == "right":
            self.position[0] += 5
        elif direction == "up":
            self.position[1] -= 5
        elif direction == "down":
            self.position[1] += 5

    def draw(self, screen):
        screen.blit(self.image, self.position)

# Example usage
pygame.init()
screen = pygame.display.set_mode((800, 600))
character = Character("virtual_character/assets/character.png", [100, 100])

running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False

    keys = pygame.key.get_pressed()
    if keys[pygame.K_LEFT]:
        character.move("left")
    if keys[pygame.K_RIGHT]:
        character.move("right")
    if keys[pygame.K_UP]:
        character.move("up")
    if keys[pygame.K_DOWN]:
        character.move("down")

    screen.fill((255, 255, 255))
    character.draw(screen)
    pygame.display.flip()

pygame.quit()
