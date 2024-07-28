import pygame
import speech_recognition as sr
import pyttsx3

class UIInteraction:
    def __init__(self, character):
        self.character = character
        self.recognizer = sr.Recognizer()
        self.engine = pyttsx3.init()

    def listen(self):
        with sr.Microphone() as source:
            print("Listening...")
            audio = self.recognizer.listen(source)
        try:
            text = self.recognizer.recognize_google(audio)
            print(f"You said: {text}")
            return text
        except sr.UnknownValueError:
            print("Sorry, I did not understand that.")
            return ""

    def speak(self, text):
        self.engine.say(text)
        self.engine.runAndWait()

    def display_chat_bubble(self, screen, text):
        font = pygame.font.Font(None, 36)
        chat_bubble = font.render(text, True, (0, 0, 0))
        screen.blit(chat_bubble, (self.character.position[0] + 50, self.character.position[1] - 30))

# Example usage
pygame.init()
screen = pygame.display.set_mode((800, 600))
character = Character("virtual_character/assets/character.png", [100, 100])
ui = UIInteraction(character)

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
    ui.display_chat_bubble(screen, "Hello!")
    pygame.display.flip()

pygame.quit()
